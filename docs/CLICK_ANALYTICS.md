# Click Analytics Design

## Overview

Record redirect clicks for analytics: who clicked, where from, when. Low traffic expected — simple synchronous inserts into PostgreSQL. When scaling is needed, switch to RabbitMQ (already available in infrastructure).

## Responsibility Boundaries

### Redirector (this service)

- Collects raw click data from the HTTP request
- Sends a **raw event** (url_id, IP, User-Agent string, Referer string, timestamp)
- Phase 1: direct INSERT of raw data into PG
- Phase 2: publish raw event to RabbitMQ

Redirector does NOT normalize data — no UA dictionaries, no referrer parsing, no domain extraction.

### Consumer service (separate)

- Reads raw click events (from PG or RabbitMQ queue)
- Normalizes User-Agent into dictionary
- Extracts referrer domain
- Manages partitions
- Builds aggregations if needed

## Database Schema

### Raw clicks table — written by redirector

```sql
CREATE SCHEMA IF NOT EXISTS analytics;

CREATE TABLE analytics.clicks (
    id              BIGSERIAL,
    url_id          BIGINT NOT NULL,
    ip              INET NOT NULL,
    user_agent      TEXT,
    referrer        TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (id, created_at)
) PARTITION BY RANGE (created_at);

CREATE INDEX idx_clicks_url_id ON analytics.clicks (url_id);
CREATE INDEX idx_clicks_created_at ON analytics.clicks (created_at);
CREATE INDEX idx_clicks_ip ON analytics.clicks USING gist (ip inet_ops);
```

Monthly partitions:

```sql
CREATE TABLE analytics.clicks_2026_02 PARTITION OF analytics.clicks
    FOR VALUES FROM ('2026-02-01') TO ('2026-03-01');
```

Old partitions can be detached/dropped without vacuum overhead.

### Normalized tables — managed by consumer service

```sql
CREATE TABLE analytics.user_agents (
    id      SERIAL PRIMARY KEY,
    raw     TEXT NOT NULL UNIQUE
);

-- Normalized click view (consumer adds ua_id, referrer_domain)
ALTER TABLE analytics.clicks ADD COLUMN ua_id INT REFERENCES analytics.user_agents(id);
ALTER TABLE analytics.clicks ADD COLUMN referrer_domain TEXT;
```

## Data Flow

### Phase 1: Direct insert (current plan)

```
Client → Redirect Handler → INSERT raw click into PG → Response
```

Simple synchronous insert in the redirect handler. At low traffic (+2-3ms per redirect) this is acceptable. Consumer service normalizes data asynchronously.

### Phase 2: RabbitMQ with in-memory buffer (when needed)

```
Client → Redirect Handler → push to buffer (Vec) → Response
                                       ↓
                            Background flush task
                         (every 500ms or 100 events)
                                       ↓
                         batch publish to RabbitMQ
                                       ↓
                              Consumer service
                                       ↓
                         Normalize + INSERT into PG
```

- Redirect handler pushes raw event into `Vec` behind `Mutex` — nanoseconds, zero impact on latency
- Background task flushes buffer to RabbitMQ by threshold (e.g. 100 events) or by timer (e.g. 500ms) — whichever comes first
- One batch = one Rabbit message with array of events, not per-click publishing
- Buffer memory footprint is negligible — 1000 clicks ~50 KB
- Consumer service handles normalization and writes to PG
- RabbitMQ is already in infrastructure

**Trade-off**: events in buffer are lost on crash. At low traffic and flush every 500ms — worst case loss is a few hundred clicks. Acceptable for analytics.

```rust
struct ClickBuffer {
    events: Mutex<Vec<RawClickEvent>>,
    flush_notify: Notify,
}

impl ClickBuffer {
    fn push(&self, event: RawClickEvent) {
        let mut events = self.events.lock().unwrap();
        events.push(event);
        if events.len() >= BATCH_SIZE {
            self.flush_notify.notify_one();
        }
    }
}

// Background task
async fn flush_loop(buffer: Arc<ClickBuffer>, rabbit: Channel) {
    loop {
        tokio::select! {
            _ = buffer.flush_notify.notified() => {},
            _ = tokio::time::sleep(FLUSH_INTERVAL) => {},
        }
        let batch = {
            let mut events = buffer.events.lock().unwrap();
            std::mem::take(&mut *events)
        };
        if !batch.is_empty() {
            rabbit.basic_publish(serialize_batch(&batch)).await;
        }
    }
}
```

## Click Event Structure (raw, from redirector)

| Field | Type | Source | Notes |
|-------|------|--------|-------|
| `url_id` | BIGINT | Decoded from hashid | Already available in handler |
| `ip` | INET | Request headers | `X-Forwarded-For` or `X-Real-IP` behind proxy |
| `user_agent` | TEXT | `User-Agent` header | Raw string, normalized by consumer |
| `referrer` | TEXT | `Referer` header | Raw value, parsed by consumer |
| `created_at` | TIMESTAMPTZ | `now()` | DB default |

## PostgreSQL Type Choices

- **`INET`** for IP: 4 bytes (IPv4) / 16 bytes (IPv6), native subnet operators (`>>=`), GiST indexable
- **`TIMESTAMPTZ`** for time: proper timezone handling, partition-friendly
- **`BIGSERIAL`** for id: auto-increment, no application-side generation

## In-App Implementation (redirector only)

### Redirect handler change

```rust
// After resolving URL, before showing interstitial
analytics::record_click(url_id, &request).await;
```

### IP extraction

```rust
fn client_ip(headers: &HeaderMap, addr: SocketAddr) -> IpAddr {
    headers.get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(addr.ip())
}
```

## Configuration

```yaml
analytics:
  enabled: true
  # Phase 1
  storage: postgres        # or "rabbitmq" for Phase 2
  # Phase 2 (future)
  rabbitmq:
    url: ${RABBITMQ_URL}
    queue: redirector.clicks
    exchange: redirector
```

## Cleanup

Monthly partition management (DBA / consumer responsibility):

```sql
-- Detach old data (instant, no locks)
ALTER TABLE analytics.clicks DETACH PARTITION analytics.clicks_2025_01;

-- Drop when no longer needed
DROP TABLE analytics.clicks_2025_01;
```

## Out of Scope (for redirector)

- UA normalization / dictionary — consumer service
- Referrer domain extraction — consumer service
- Geographic data (GeoIP) — consumer service
- Analytics dashboard — separate service
- Aggregation tables — consumer service
- Export to CSV/JSON — standard `COPY` from partitions
