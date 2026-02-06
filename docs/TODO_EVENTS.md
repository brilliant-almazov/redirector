# TODO: Event Analytics Pipeline

Список улучшений для v0.4.0+

---

## Medium

### Persistent Buffer (SQLite)

**Проблема**: при недоступности RabbitMQ события теряются.

**Текущее поведение**:
```rust
// publisher.rs:109-113
if !queue.is_connected() {
    if let Err(e) = queue.reconnect().await {  // ← блокирует ~30 сек
        tracing::warn!(error = %e, "Failed to reconnect");
    }
}
```

При disconnect:
1. `reconnect()` блокирует flush loop на ~30 сек (TCP timeout)
2. `try_send()` в handler'ах продолжает работать
3. mpsc канал заполняется (10K буфер)
4. Новые события дропаются (`TrySendError::Full`)
5. Потеря данных

**Последствия**:
- RabbitMQ maintenance = потеря событий
- Network glitch = потеря событий
- Нет гарантии доставки

**Что делать**:

#### Шаг 1: SQLite схема

```sql
-- events_buffer.db
CREATE TABLE event_buffer (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    payload BLOB NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE INDEX idx_buffer_created ON event_buffer(created_at);
```

#### Шаг 2: Dual-write в publisher

```rust
struct PersistentBuffer {
    conn: rusqlite::Connection,
    path: PathBuf,
}

impl PersistentBuffer {
    fn push(&self, payload: &[u8]) -> rusqlite::Result<()> {
        self.conn.execute(
            "INSERT INTO event_buffer (payload) VALUES (?)",
            [payload],
        )?;
        Ok(())
    }

    fn pop_batch(&self, limit: usize) -> rusqlite::Result<Vec<(i64, Vec<u8>)>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, payload FROM event_buffer ORDER BY id LIMIT ?"
        )?;
        let rows = stmt.query_map([limit as i64], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })?;
        rows.collect()
    }

    fn delete(&self, ids: &[i64]) -> rusqlite::Result<()> {
        let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        self.conn.execute(
            &format!("DELETE FROM event_buffer WHERE id IN ({})", placeholders),
            rusqlite::params_from_iter(ids),
        )?;
        Ok(())
    }

    fn count(&self) -> rusqlite::Result<usize> {
        self.conn.query_row("SELECT COUNT(*) FROM event_buffer", [], |row| row.get(0))
    }
}
```

#### Шаг 3: Изменить flush логику

```rust
async fn flush(
    buffer: &mut Vec<RedirectEvent>,
    queue: &mut dyn EventQueue,
    persistent: &PersistentBuffer,
    snowflake: &SnowflakeGenerator,
) {
    // Сериализовать batch
    let batch = EventBatch::Redirect {
        events: std::mem::take(buffer),
        batch_id: snowflake.generate(),
        produced_at: chrono::Utc::now().naive_utc(),
    };
    let payload = serde_json::to_vec(&batch).unwrap();

    if queue.is_connected() {
        // Сначала drain persistent buffer (FIFO)
        while let Ok(pending) = persistent.pop_batch(10) {
            if pending.is_empty() { break; }

            let mut success_ids = Vec::new();
            for (id, old_payload) in &pending {
                if queue.publish(old_payload).await.is_ok() {
                    success_ids.push(*id);
                } else {
                    break;  // Connection lost, stop draining
                }
            }
            persistent.delete(&success_ids).ok();

            if success_ids.len() < pending.len() {
                break;  // Partial success, connection issues
            }
        }

        // Теперь текущий batch
        match queue.publish(&payload).await {
            Ok(()) => {
                metrics::counter!("events_published").increment(batch.event_count() as u64);
            }
            Err(e) => {
                tracing::warn!(error = %e, "RabbitMQ publish failed, persisting to SQLite");
                persistent.push(&payload).ok();
                metrics::counter!("events_persisted").increment(batch.event_count() as u64);
            }
        }
    } else {
        // No connection — persist
        persistent.push(&payload).ok();
        metrics::counter!("events_persisted").increment(batch.event_count() as u64);
        metrics::gauge!("events_buffer_size").set(persistent.count().unwrap_or(0) as f64);
    }
}
```

#### Шаг 4: Конфигурация

```yaml
events:
  publisher:
    buffer_path: "/var/lib/redirector/events_buffer.db"  # SQLite path
    buffer_max_size_mb: 100  # Ограничение размера
```

#### Шаг 5: Метрики и алертинг

```rust
// Gauge для размера буфера
metrics::gauge!("events_buffer_size").set(persistent.count() as f64);
metrics::gauge!("events_buffer_oldest_seconds").set(oldest_event_age);
```

Grafana alert: `events_buffer_size > 1000` → Warning
Grafana alert: `events_buffer_oldest_seconds > 3600` → Critical

**Файлы для изменения**:
- `src/events/publisher.rs` — PersistentBuffer, изменённый flush
- `src/events/mod.rs` — re-export
- `src/config.rs` — buffer_path, buffer_max_size_mb
- `Cargo.toml` — добавить `rusqlite`

**Оценка**: 4-5 часов

---

## Medium

### DLQ (Dead Letter Queue)

**Проблема**: poison messages (невалидный JSON, ошибка десериализации) теряются — `nack(requeue=false)` и всё.

**Текущее поведение**:
```rust
// consumer.rs:93-101
Err(e) => {
    tracing::error!(error = %e, "Failed to deserialize event batch");
    delivery
        .nack(BasicNackOptions { requeue: false, .. })  // ← сообщение исчезает
        .await
}
```

**Последствия**:
- Нет способа узнать, какие сообщения потерялись
- Нет возможности отладить проблему
- Нет алертинга
- При баге в сериализации — silent data loss

**Что делать**:

#### Шаг 1: RabbitMQ конфигурация

Создать Dead Letter Exchange и очередь:

```bash
# Создать exchange для dead letters
rabbitmqadmin declare exchange \
  name=events.dlx \
  type=direct \
  durable=true

# Создать очередь для dead letters
rabbitmqadmin declare queue \
  name=events.analytics.dlq \
  durable=true

# Связать exchange с очередью
rabbitmqadmin declare binding \
  source=events.dlx \
  destination=events.analytics.dlq \
  routing_key=events.analytics
```

#### Шаг 2: Изменить основную очередь

В `queue_rabbitmq.rs`, при declare очереди добавить DLX:

```rust
// queue_rabbitmq.rs:75-88
channel
    .queue_declare(
        &config.rabbitmq.queue,
        QueueDeclareOptions {
            durable: true,
            ..Default::default()
        },
        {
            let mut args = FieldTable::default();
            args.insert(
                "x-dead-letter-exchange".into(),
                AMQPValue::LongString("events.dlx".into()),
            );
            args.insert(
                "x-dead-letter-routing-key".into(),
                AMQPValue::LongString(config.rabbitmq.queue.clone().into()),
            );
            args
        },
    )
    .await?;
```

#### Шаг 3: Таблица для failed events

Новая миграция `migrations/analytics/20260206000001_create_failed_events.sql`:

```sql
CREATE TABLE failed_events (
    id BIGSERIAL PRIMARY KEY,

    -- Оригинальный payload
    payload BYTEA NOT NULL,
    payload_text TEXT,  -- попытка декодировать как UTF-8

    -- Информация об ошибке
    error_type TEXT NOT NULL,      -- 'deserialization', 'insert', 'enrichment'
    error_message TEXT NOT NULL,
    error_details JSONB,           -- stack trace, context

    -- Метаданные
    queue_name TEXT NOT NULL,
    delivery_tag BIGINT,
    failed_at TIMESTAMP NOT NULL DEFAULT NOW(),

    -- Для retry логики
    retry_count INT DEFAULT 0,
    last_retry_at TIMESTAMP,
    resolved BOOLEAN DEFAULT FALSE,
    resolved_at TIMESTAMP,
    resolved_by TEXT               -- 'manual', 'auto-retry', 'ignored'
);

CREATE INDEX idx_failed_events_failed_at ON failed_events(failed_at);
CREATE INDEX idx_failed_events_resolved ON failed_events(resolved) WHERE NOT resolved;

COMMENT ON TABLE failed_events IS 'Dead letter storage for failed event processing';
```

#### Шаг 4: Логирование в consumer

Добавить в `consumer.rs`:

```rust
async fn log_failed_event(
    pool: &PgPool,
    payload: &[u8],
    error_type: &str,
    error: &dyn std::error::Error,
    queue_name: &str,
    delivery_tag: u64,
) -> anyhow::Result<()> {
    let payload_text = String::from_utf8_lossy(payload);

    sqlx::query(
        "INSERT INTO failed_events (payload, payload_text, error_type, error_message, queue_name, delivery_tag)
         VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(payload)
    .bind(payload_text.as_ref())
    .bind(error_type)
    .bind(error.to_string())
    .bind(queue_name)
    .bind(delivery_tag as i64)
    .execute(pool)
    .await?;

    metrics::counter!("events_failed_total", "type" => error_type).increment(1);

    Ok(())
}
```

Использование:

```rust
// В process_delivery()
Err(e) => {
    tracing::error!(error = %e, "Failed to deserialize event batch");

    // Сохранить в failed_events
    if let Err(log_err) = log_failed_event(
        pool,
        &delivery.data,
        "deserialization",
        &e,
        queue_name,
        delivery.delivery_tag,
    ).await {
        tracing::error!(error = %log_err, "Failed to log failed event");
    }

    delivery.nack(BasicNackOptions { requeue: false, .. }).await;
}
```

#### Шаг 5: Алертинг

Grafana alert query:

```sql
SELECT COUNT(*) as failed_count
FROM failed_events
WHERE failed_at > NOW() - INTERVAL '1 hour'
  AND NOT resolved
```

Alert условие: `failed_count > 0` → PagerDuty/Slack

#### Шаг 6: CLI для работы с DLQ

Утилита `tools/dlq_manager.rs`:

```rust
/// Просмотр failed events
/// cargo run --bin dlq_manager -- list --limit 10

/// Retry конкретного события
/// cargo run --bin dlq_manager -- retry --id 123

/// Пометить как resolved
/// cargo run --bin dlq_manager -- resolve --id 123 --reason "manual fix"
```

**Файлы для изменения**:
- `src/events/consumer.rs` — добавить log_failed_event
- `migrations/analytics/` — новая миграция
- `queue_rabbitmq.rs` — DLX arguments
- `tools/dlq_manager.rs` — новый бинарник (опционально)

**Оценка**: 2-3 часа

---

## Critical

### Prometheus metrics (publisher + consumer)

**Приоритет**: CRITICAL — без метрик не видно проблем.

**Проблема**: consumer — чёрный ящик, publisher тоже. Не видно:
- Сколько событий обработано
- Какая latency обработки
- Есть ли ошибки enrichment
- Какой lag от RabbitMQ

**Текущее состояние**: только tracing логи, никаких метрик.

**Что делать**:

#### Шаг 1: Добавить HTTP сервер для метрик

В `event_consumer.rs`:

```rust
use axum::{routing::get, Router};
use std::net::SocketAddr;

async fn metrics_handler() -> String {
    let encoder = prometheus::TextEncoder::new();
    let metric_families = prometheus::gather();
    encoder.encode_to_string(&metric_families).unwrap()
}

async fn start_metrics_server(port: u16) {
    let app = Router::new().route("/metrics", get(metrics_handler));
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    tracing::info!(port, "Starting metrics server");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ... existing setup ...

    let metrics_port = std::env::var("METRICS_PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(9090);

    // Запустить metrics server в фоне
    tokio::spawn(start_metrics_server(metrics_port));

    // ... existing consumer loop ...
}
```

#### Шаг 2: Определить метрики

Создать `src/events/consumer_metrics.rs`:

```rust
use once_cell::sync::Lazy;
use prometheus::{
    Counter, CounterVec, Histogram, HistogramVec, Gauge, GaugeVec,
    register_counter, register_counter_vec,
    register_histogram, register_histogram_vec,
    register_gauge, register_gauge_vec,
};

// Counters
pub static EVENTS_CONSUMED: Lazy<Counter> = Lazy::new(|| {
    register_counter!(
        "events_consumed_total",
        "Total number of events consumed and processed"
    ).unwrap()
});

pub static EVENTS_ERRORS: Lazy<CounterVec> = Lazy::new(|| {
    register_counter_vec!(
        "events_consume_errors_total",
        "Total number of event processing errors",
        &["error_type"]  // deserialization, insert, enrichment
    ).unwrap()
});

pub static BATCHES_PROCESSED: Lazy<Counter> = Lazy::new(|| {
    register_counter!(
        "events_batches_processed_total",
        "Total number of batches processed"
    ).unwrap()
});

// Histograms
pub static BATCH_PROCESSING_DURATION: Lazy<Histogram> = Lazy::new(|| {
    register_histogram!(
        "events_batch_processing_seconds",
        "Time spent processing a batch",
        vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0]
    ).unwrap()
});

pub static BATCH_SIZE: Lazy<Histogram> = Lazy::new(|| {
    register_histogram!(
        "events_batch_size",
        "Number of events in a batch",
        vec![1.0, 5.0, 10.0, 25.0, 50.0, 100.0, 250.0, 500.0]
    ).unwrap()
});

pub static ENRICHMENT_DURATION: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        "events_enrichment_seconds",
        "Time spent on enrichment",
        &["enrichment_type"],  // user_agent, geoip, referer
        vec![0.0001, 0.0005, 0.001, 0.005, 0.01, 0.05, 0.1]
    ).unwrap()
});

pub static DB_INSERT_DURATION: Lazy<Histogram> = Lazy::new(|| {
    register_histogram!(
        "events_db_insert_seconds",
        "Time spent inserting events to database",
        vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0]
    ).unwrap()
});

// Gauges
pub static RABBITMQ_CONNECTED: Lazy<Gauge> = Lazy::new(|| {
    register_gauge!(
        "events_consumer_rabbitmq_connected",
        "Whether consumer is connected to RabbitMQ"
    ).unwrap()
});

pub static LAST_BATCH_TIMESTAMP: Lazy<Gauge> = Lazy::new(|| {
    register_gauge!(
        "events_consumer_last_batch_timestamp",
        "Unix timestamp of last processed batch"
    ).unwrap()
});
```

#### Шаг 3: Инструментировать consumer

В `consumer.rs`:

```rust
use crate::events::consumer_metrics::*;
use std::time::Instant;

async fn process_delivery(delivery: &Delivery, storage: &dyn EventStorage) {
    let batch_start = Instant::now();

    match serde_json::from_slice::<EventBatch>(&delivery.data) {
        Ok(batch) => {
            BATCH_SIZE.observe(batch.event_count() as f64);

            match storage.insert_batch(&batch).await {
                Ok(count) => {
                    EVENTS_CONSUMED.inc_by(count as f64);
                    BATCHES_PROCESSED.inc();
                    BATCH_PROCESSING_DURATION.observe(batch_start.elapsed().as_secs_f64());
                    LAST_BATCH_TIMESTAMP.set(chrono::Utc::now().timestamp() as f64);

                    delivery.ack(BasicAckOptions::default()).await.ok();
                }
                Err(e) => {
                    EVENTS_ERRORS.with_label_values(&["insert"]).inc();
                    delivery.nack(...).await.ok();
                }
            }
        }
        Err(e) => {
            EVENTS_ERRORS.with_label_values(&["deserialization"]).inc();
            delivery.nack(...).await.ok();
        }
    }
}
```

#### Шаг 4: Инструментировать enrichment

В `storage_postgres.rs`:

```rust
async fn insert_redirects(&self, events: &[RedirectEvent], batch_id: i64) -> anyhow::Result<usize> {
    let insert_start = Instant::now();

    // ... existing code ...

    for event in events {
        // User-Agent enrichment timing
        let ua_start = Instant::now();
        let user_agent_id = resolve_or_create(&self.user_agents, &mut tx, event.user_agent.as_deref()).await?;
        ENRICHMENT_DURATION.with_label_values(&["user_agent"]).observe(ua_start.elapsed().as_secs_f64());

        // GeoIP enrichment timing
        let geo_start = Instant::now();
        let geo_location_id = resolve_or_create(&self.geo_locations, &mut tx, event.ip.as_deref()).await?;
        ENRICHMENT_DURATION.with_label_values(&["geoip"]).observe(geo_start.elapsed().as_secs_f64());

        // ... rest ...
    }

    tx.commit().await?;
    DB_INSERT_DURATION.observe(insert_start.elapsed().as_secs_f64());

    Ok(events.len())
}
```

**Файлы для изменения**:
- `src/bin/event_consumer.rs` — HTTP сервер
- `src/events/consumer_metrics.rs` — новый файл с метриками
- `src/events/consumer.rs` — инструментация
- `src/events/storage_postgres.rs` — инструментация enrichment
- `Cargo.toml` — добавить `prometheus` crate

**Оценка**: 2-3 часа

---

## Medium

### Configurable machine_id

**Проблема**:

```rust
// publisher.rs:49
let snowflake = Arc::new(create_snowflake_generator(0));  // hardcoded 0
```

Snowflake ID структура:
```
| 41 bits timestamp | 10 bits machine_id | 12 bits sequence |
```

Если два publisher'а с одинаковым machine_id генерируют ID в одну миллисекунду — коллизия.

**Когда это проблема**:
- Несколько инстансов redirector за load balancer
- Kubernetes с несколькими репликами
- Blue-green deployment

**Что делать**:

#### Шаг 1: Добавить в конфиг

В `config.rs`:

```rust
#[derive(Debug, Deserialize, Clone)]
pub struct PublisherConfig {
    #[serde(default = "default_channel_buffer")]
    pub channel_buffer_size: usize,
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,
    #[serde(default = "default_flush_interval_ms")]
    pub flush_interval_ms: u64,
    #[serde(default)]
    pub machine_id: u16,  // 0-1023, default 0
}
```

#### Шаг 2: Валидация

```rust
impl PublisherConfig {
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.machine_id > 1023 {
            anyhow::bail!("machine_id must be 0-1023, got {}", self.machine_id);
        }
        Ok(())
    }
}
```

#### Шаг 3: Использовать в publisher

В `publisher.rs`:

```rust
async fn publisher_loop(mut rx: mpsc::Receiver<RedirectEvent>, config: &EventsConfig) {
    let snowflake = Arc::new(create_snowflake_generator(config.publisher.machine_id));
    // ... rest unchanged ...
}
```

#### Шаг 4: Документация

В `config.yaml.example`:

```yaml
events:
  publisher:
    machine_id: 0  # 0-1023, unique per publisher instance
                   # For K8s: use StatefulSet ordinal or pod IP hash
```

В README:

```markdown
### Multiple Publisher Instances

When running multiple redirector instances, each must have a unique `machine_id`:

```bash
# Instance 1
REDIRECTOR__EVENTS__PUBLISHER__MACHINE_ID=0

# Instance 2
REDIRECTOR__EVENTS__PUBLISHER__MACHINE_ID=1
```

For Kubernetes StatefulSet, derive from pod ordinal:

```yaml
env:
  - name: POD_NAME
    valueFrom:
      fieldRef:
        fieldPath: metadata.name
  - name: REDIRECTOR__EVENTS__PUBLISHER__MACHINE_ID
    value: "$(echo $POD_NAME | grep -oE '[0-9]+$')"
```
```

**Файлы для изменения**:
- `src/config.rs` — добавить поле
- `src/events/publisher.rs` — использовать из конфига
- `config.yaml.example` — документация
- `README.md` — документация

**Оценка**: 30 минут

---

## Critical

### Graceful shutdown в consumer

**Приоритет**: CRITICAL — обязателен для production.

**Проблема**:

```rust
// consumer.rs:50-58
while let Some(delivery) = consumer.next().await {
    match delivery {
        Ok(delivery) => {
            process_delivery(&delivery, storage).await;  // ← может быть прервано
        }
        // ...
    }
}
```

При SIGTERM:
1. Tokio runtime начинает shutdown
2. `consumer.next()` прерывается
3. Текущий `process_delivery()` может быть посередине
4. Сообщение не acked/nacked
5. RabbitMQ через timeout делает redelivery
6. → Дубликаты

**Что делать**:

#### Шаг 1: Добавить shutdown signal

В `event_consumer.rs`:

```rust
use tokio::sync::watch;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ... setup ...

    // Канал для shutdown signal
    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    // Обработка SIGTERM/SIGINT
    let shutdown_tx_clone = shutdown_tx.clone();
    tokio::spawn(async move {
        let ctrl_c = tokio::signal::ctrl_c();

        #[cfg(unix)]
        let mut sigterm = tokio::signal::unix::signal(
            tokio::signal::unix::SignalKind::terminate()
        ).unwrap();

        tokio::select! {
            _ = ctrl_c => {}
            #[cfg(unix)]
            _ = sigterm.recv() => {}
        }

        tracing::info!("Shutdown signal received");
        let _ = shutdown_tx_clone.send(true);
    });

    // Запустить consumer с shutdown channel
    run_with_shutdown(&config, &*storage, shutdown_rx).await
}
```

#### Шаг 2: Изменить consumer loop

В `consumer.rs`:

```rust
pub async fn run_with_shutdown(
    config: &EventsConfig,
    storage: &dyn EventStorage,
    mut shutdown: watch::Receiver<bool>,
) -> anyhow::Result<()> {
    // ... connection setup ...

    loop {
        tokio::select! {
            // Проверить shutdown первым (biased)
            biased;

            _ = shutdown.changed() => {
                if *shutdown.borrow() {
                    tracing::info!("Shutdown requested, stopping consumer");
                    break;
                }
            }

            delivery = consumer.next() => {
                match delivery {
                    Some(Ok(delivery)) => {
                        // process_delivery выполнится полностью
                        process_delivery(&delivery, storage).await;
                    }
                    Some(Err(e)) => {
                        tracing::error!(error = %e, "Consumer delivery error");
                    }
                    None => {
                        tracing::warn!("Consumer stream ended");
                        break;
                    }
                }
            }
        }
    }

    tracing::info!("Consumer shutdown complete");
    Ok(())
}
```

#### Шаг 3: Drain in-flight messages

```rust
// После выхода из loop, дождаться завершения текущих операций
tracing::info!("Waiting for in-flight operations to complete...");

// Если используем semaphore для ограничения concurrency
if let Some(semaphore) = &semaphore {
    let _ = semaphore.acquire_many(MAX_CONCURRENT as u32).await;
}

tracing::info!("All operations complete, shutting down");
```

#### Шаг 4: Docker/K8s конфигурация

```yaml
# docker-compose.yaml
services:
  event_consumer:
    stop_grace_period: 30s  # дать время на graceful shutdown

# kubernetes
spec:
  terminationGracePeriodSeconds: 30
  containers:
    - name: event-consumer
      lifecycle:
        preStop:
          exec:
            command: ["sleep", "5"]  # дать время на deregister
```

**Файлы для изменения**:
- `src/bin/event_consumer.rs` — signal handling
- `src/events/consumer.rs` — shutdown-aware loop
- `docker-compose.yaml` — grace period

**Оценка**: 1-2 часа

---

## Medium

### Async reconnect в publisher

**Проблема**:

```rust
// publisher.rs:108-113
if !queue.is_connected() {
    if let Err(e) = queue.reconnect().await {  // ← блокирует flush loop
        tracing::warn!(error = %e, "Failed to reconnect to event queue");
    }
}
```

Если RabbitMQ недоступен:
1. `reconnect()` пытается подключиться (timeout ~30s)
2. Весь flush loop блокирован
3. События копятся в буфере
4. Буфер переполняется → события дропаются

**Что делать**:

#### Шаг 1: Фоновый reconnect task

В `publisher.rs`:

```rust
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

struct ConnectionState {
    connected: AtomicBool,
    queue: tokio::sync::RwLock<Box<dyn EventQueue>>,
}

async fn reconnect_loop(state: Arc<ConnectionState>, config: EventsConfig) {
    let mut interval = tokio::time::interval(Duration::from_secs(5));

    loop {
        interval.tick().await;

        if !state.connected.load(Ordering::Relaxed) {
            tracing::debug!("Attempting to reconnect to RabbitMQ...");

            let new_queue = crate::events::create_queue(&config).await;

            if new_queue.is_connected() {
                let mut guard = state.queue.write().await;
                *guard = new_queue;
                state.connected.store(true, Ordering::Relaxed);
                tracing::info!("Reconnected to RabbitMQ");
                metrics::gauge!("rabbitmq_connected").set(1.0);
            }
        }
    }
}
```

#### Шаг 2: Изменить flush

```rust
async fn flush(
    buffer: &mut Vec<RedirectEvent>,
    state: &Arc<ConnectionState>,
    snowflake: &SnowflakeGenerator,
) {
    let batch = EventBatch::Redirect {
        events: std::mem::take(buffer),
        batch_id: snowflake.generate(),
        produced_at: chrono::Utc::now().naive_utc(),
    };

    let payload = match serde_json::to_vec(&batch) {
        Ok(p) => p,
        Err(e) => {
            tracing::error!(error = %e, "Failed to serialize event batch");
            metrics::counter!("events_serialize_errors").increment(1);
            return;
        }
    };

    // Просто проверяем состояние, не пытаемся reconnect
    if !state.connected.load(Ordering::Relaxed) {
        tracing::warn!(
            dropped_count = batch.event_count(),
            "No RabbitMQ connection, dropping batch"
        );
        metrics::counter!("events_dropped").increment(batch.event_count() as u64);
        return;
    }

    // Публикация
    let queue = state.queue.read().await;
    match queue.publish(&payload).await {
        Ok(()) => {
            metrics::counter!("events_published").increment(batch.event_count() as u64);
        }
        Err(e) => {
            tracing::warn!(error = %e, "Failed to publish event batch");
            metrics::counter!("events_publish_errors").increment(1);
            // Пометить как disconnected, reconnect loop подхватит
            drop(queue);
            state.connected.store(false, Ordering::Relaxed);
            metrics::gauge!("rabbitmq_connected").set(0.0);
        }
    }
}
```

#### Шаг 3: Запуск обоих tasks

```rust
pub fn start_publisher(config: &EventsConfig) -> EventSender {
    let (tx, rx) = mpsc::channel(config.publisher.channel_buffer_size);

    let config = config.clone();
    tokio::spawn(async move {
        let queue = crate::events::create_queue(&config).await;
        let state = Arc::new(ConnectionState {
            connected: AtomicBool::new(queue.is_connected()),
            queue: tokio::sync::RwLock::new(queue),
        });

        // Запустить reconnect loop
        let reconnect_state = Arc::clone(&state);
        let reconnect_config = config.clone();
        tokio::spawn(async move {
            reconnect_loop(reconnect_state, reconnect_config).await;
        });

        // Запустить publisher loop
        publisher_loop(rx, &config, state).await;
    });

    EventSender { tx }
}
```

**Файлы для изменения**:
- `src/events/publisher.rs` — ConnectionState, reconnect_loop, изменённый flush

**Оценка**: 1-2 часа

---

## Low

### GeoIP setup

**Задача**: Настроить GeoLite2 для геолокации по IP.

**Шаги**:
1. Зарегистрироваться на https://www.maxmind.com/en/geolite2/signup
2. Получить license key в аккаунте
3. Скачать GeoLite2-City.mmdb
4. Положить в `/var/lib/redirector/GeoLite2-City.mmdb`
5. Указать путь в конфиге:
   ```yaml
   events:
     consumer:
       geoip_path: "/var/lib/redirector/GeoLite2-City.mmdb"
   ```
6. Настроить cron для обновления (раз в неделю)

**Без GeoIP**: consumer работает, просто geo_location_id = 1 (unknown) для всех событий.

**Оценка**: 30 минут

---

## Won't Fix

| Пункт | Причина |
|-------|---------|
| Bulk INSERT | Архитектура решения понятна (batch SELECT + multi-row INSERT). 500µs/event = 172M events/day — достаточно |
| Ротация referers/user_agents | Таблицы маленькие, растут медленно. 1M уникальных UA = ~100MB |
| Pre-create партиций | Consumer создаёт on-demand. Первый INSERT месяца +50ms — приемлемо |
| FK constraints | Всё пишется в одной транзакции. Консистентность гарантирована. FK = overhead на каждый INSERT |
| RwLock для GeoIP | Consumer single-threaded. Mutex достаточно |
| GeoIP hourly reload | Overkill — база обновляется раз в неделю. Рестарт consumer при обновлении |
