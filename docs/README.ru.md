# redirector

> **Высокопроизводительный сервис сокращения ссылок и редиректов** на Rust, Axum, Redis и PostgreSQL. Безопасные промежуточные страницы, панель мониторинга в реальном времени и полная наблюдаемость.

[English](../README.md) | **Русский** | [中文](README.zh.md) | [हिंदी](README.hi.md) | [Español](README.es.md) | [Português](README.pt.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Polski](README.pl.md) | [Nederlands](README.nl.md) | [Italiano](README.it.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) | [עברית](README.he.md) | [Bahasa Indonesia](README.id.md) | [Tiếng Việt](README.vi.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![Docker Image Size](https://ghcr-badge.egpl.dev/brilliant-almazov/redirector/size)](https://github.com/brilliant-almazov/redirector/pkgs/container/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

**Ключевые слова**: сокращатель ссылок, url shortener, сервис редиректов, rust веб-сервис, axum фреймворк, redis кэш, postgresql, prometheus метрики, hashids, короткие ссылки, промежуточные страницы, безопасные редиректы, высокая производительность, микросервис

Безопасный сервис редиректов с промежуточными страницами и короткими ссылками на основе hashid. Идеально подходит для внутренних инструментов, корпоративного управления ссылками и брендированных URL-сервисов.

### Производительность

| Сценарий | RPS | Средн. задержка | P99 задержка |
|----------|-----|-----------------|--------------|
| 100% Cache Hit | **7,800+** | ~14ms | ~50ms |
| Cache Miss (10K URLs) | **2,300+** | ~44ms | ~81ms |

**Условия теста**: wrk -t4 -c100 -d30s, PostgreSQL 15, Dragonfly (Redis), macOS M1 (Docker)

> ⚠️ Результаты получены в Docker на macOS с overhead виртуализации. На native Linux ожидается **в 3-5 раз быстрее**.

## Проблема

Делиться длинными URL неудобно. Сокращатели ссылок существуют, но часто перенаправляют мгновенно, что может быть небезопасно. Пользователи должны видеть, куда их перенаправляют, прежде чем перейти.

**redirector** обеспечивает безопасные редиректы:
- Промежуточная страница показывает целевой URL перед редиректом
- Обратный отсчёт для осведомлённости пользователя
- Красивые, брендированные страницы

## Возможности

- 🔗 **Hashid URL** - Короткие, уникальные, непоследовательные ID (например, `/r/abc123`)
- ⏱️ **Промежуточная страница** - Обратный отсчёт показывает целевой URL перед редиректом
- ⚡ **Кэширование Redis** - Быстрые запросы с настраиваемым TTL
- 🛡️ **Circuit breaker** - Защита БД от каскадных сбоев
- 🚦 **Rate limiting** - Глобальный и на уровне БД
- 📊 **Prometheus метрики** - Полная наблюдаемость с Basic Auth защитой
- 🎨 **Красивые страницы** - Чистые страницы 404 и индекса с 4 темами
- 🔑 **Несколько солей** - Поддержка ротации соли hashid для миграции
- 📱 **Админ-панель** - Мониторинг метрик в реальном времени через SSE
- 📤 **Аналитика событий** - Опциональная публикация событий в RabbitMQ с консьюмером для PostgreSQL

## Скриншоты

| Светлая | Тёмная | Серая | Тёплая |
|---------|--------|-------|--------|
| ![Дашборд светлая](screenshots/dashboard-light.png) | ![Дашборд тёмная](screenshots/dashboard-dark.png) | ![Дашборд серая](screenshots/dashboard-gray.png) | ![Дашборд тёплая](screenshots/dashboard-warm.png) |
| ![Логин светлая](screenshots/login-light.png) | ![Логин тёмная](screenshots/login-dark.png) | ![Логин серая](screenshots/login-gray.png) | ![Логин тёплая](screenshots/login-warm.png) |
| ![Index Light](screenshots/index-light.png) | ![Index Dark](screenshots/index-dark.png) | ![Index Gray](screenshots/index-gray.png) | ![Index Warm](screenshots/index-warm.png) |
| ![Interstitial Light](screenshots/interstitial-light.png) | ![Interstitial Dark](screenshots/interstitial-dark.png) | ![Interstitial Gray](screenshots/interstitial-gray.png) | ![Interstitial Warm](screenshots/interstitial-warm.png) |
| ![404 Light](screenshots/404-light.png) | ![404 Dark](screenshots/404-dark.png) | ![404 Gray](screenshots/404-gray.png) | ![404 Warm](screenshots/404-warm.png) |

### Модалка нагрузочного тестирования

| Светлая | Тёмная | Серая | Тёплая |
|---------|--------|-------|--------|
| ![Модалка светлая](screenshots/dashboard-modal-light.png) | ![Модалка тёмная](screenshots/dashboard-modal-dark.png) | ![Модалка серая](screenshots/dashboard-modal-gray.png) | ![Модалка тёплая](screenshots/dashboard-modal-warm.png) |

## Стек технологий

- **Язык**: Rust (асинхронный с Tokio)
- **Веб-фреймворк**: Axum
- **Кэш**: Redis-compatible (Redis, Dragonfly, Valkey, KeyDB и др.)
- **База данных**: PostgreSQL (абстрактный слой хранения)
- **Метрики**: Prometheus + metrics-rs
- **Очередь сообщений**: RabbitMQ (опционально, для аналитики)
- **Хеширование паролей**: Argon2

> **Примечание**: Слои хранения и кэширования абстрагированы и могут быть заменены любым совместимым источником данных. Проект в активной разработке.

## Быстрый старт

### Docker

```bash
docker run -p 8080:8080 \
  -v $(pwd)/config.yaml:/config.yaml \
  ghcr.io/brilliant-almazov/redirector:latest
```

### Docker Compose

```yaml
services:
  redirector:
    image: ghcr.io/brilliant-almazov/redirector:latest
    ports:
      - "8080:8080"
    volumes:
      - ./config.yaml:/config.yaml
    depends_on:
      - postgres
      - redis

  postgres:
    image: postgres:16-alpine
    environment:
      POSTGRES_USER: redirector
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
      POSTGRES_DB: redirector

  redis:
    image: redis:7-alpine
```

## Конфигурация

Создайте `config.yaml`:

```yaml
server:
  host: "0.0.0.0"
  port: 8080

hashids:
  salts:
    - ${HASHID_SALT}          # Основная соль
    - ${HASHID_SALT_OLD}      # Опционально: старая соль для миграции
  min_length: 6

redis:
  url: ${REDIS_URL}
  cache_ttl_seconds: 86400    # 24 часа

database:
  url: ${DATABASE_URL}
  pool:
    max_connections: 5
    connect_timeout_seconds: 3
  rate_limit:
    max_requests_per_second: 50
  circuit_breaker:
    failure_threshold: 3
    reset_timeout_seconds: 60
  query:
    table: "dictionary.urls"    # Имя таблицы
    id_column: "id"             # Колонка ID
    url_column: "name"          # Колонка URL

interstitial:
  delay_seconds: 5            # Отсчёт перед редиректом

metrics:
  basic_auth:
    username: prometheus
    password: ${METRICS_PASSWORD}

rate_limit:
  requests_per_second: 1000
  burst: 100
```

### Параметры конфигурации

#### Сервер

| Параметр | По умолчанию | Описание |
|----------|--------------|----------|
| `host` | `0.0.0.0` | Адрес привязки |
| `port` | `8080` | HTTP порт |

#### Hashids

| Параметр | По умолчанию | Описание |
|----------|--------------|----------|
| `salts` | *обязательно* | Список солей hashid (первая = основная) |
| `min_length` | `6` | Минимальная длина hashid |

#### Redis

| Параметр | По умолчанию | Описание |
|----------|--------------|----------|
| `url` | *обязательно* | URL подключения к Redis |
| `cache_ttl_seconds` | `86400` | TTL кэша в секундах |

#### База данных

| Параметр | По умолчанию | Описание |
|----------|--------------|----------|
| `url` | *обязательно* | URL подключения к PostgreSQL |
| `pool.max_connections` | `3` | Размер пула соединений |
| `pool.connect_timeout_seconds` | `3` | Таймаут подключения |
| `rate_limit.max_requests_per_second` | `50` | Rate limit БД |
| `circuit_breaker.failure_threshold` | `3` | Сбоев до открытия |
| `circuit_breaker.reset_timeout_seconds` | `60` | Таймаут сброса circuit |

#### Rate Limit (глобальный)

| Параметр | По умолчанию | Описание |
|----------|--------------|----------|
| `requests_per_second` | `1000` | Глобальный rate limit |
| `burst` | `100` | Размер всплеска |

### Переменные окружения

Есть **три способа** конфигурации сервиса, в порядке приоритета (от высшего к низшему):

| Приоритет | Способ | Когда использовать |
|-----------|--------|-------------------|
| 1 | Переменные `REDIRECTOR__*` | Переопределение отдельных значений |
| 2 | Стандартные PaaS-переменные (`DATABASE_URL` и др.) | PaaS-платформы (Railway, Heroku, Render) |
| 3 | Конфигурационный файл (`config.yaml` или `CONFIG_BASE64`) | Базовая конфигурация |

#### Служебные переменные

| Переменная | По умолчанию | Описание |
|------------|--------------|----------|
| `CONFIG_PATH` | `config.yaml` | Путь к YAML-конфигурации |
| `CONFIG_BASE64` | — | YAML-конфигурация в Base64 (приоритет над `CONFIG_PATH`) |

#### Стандартные PaaS-переменные

Автоматически распознаются и применяются. Большинство PaaS-платформ устанавливают их за вас:

| Переменная | Путь в конфиге | Пример |
|------------|---------------|--------|
| `DATABASE_URL` | `database.url` | `postgres://user:pass@host:5432/db` |
| `REDIS_URL` | `redis.url` | `redis://host:6379` |
| `PORT` | `server.port` | `3000` |

> **Правило приоритета**: если установлены и `DATABASE_URL`, и `REDIRECTOR__DATABASE__URL`, то побеждает версия с префиксом `REDIRECTOR__`.

#### Переменные с префиксом (`REDIRECTOR__*`)

Любое значение конфигурации можно переопределить через переменные с префиксом `REDIRECTOR__` и разделителем `__` (двойное подчёркивание) для вложенности:

```
Путь в YAML              →  Переменная окружения
─────────────────────────────────────────────────────
server.port               →  REDIRECTOR__SERVER__PORT
server.host               →  REDIRECTOR__SERVER__HOST
database.url              →  REDIRECTOR__DATABASE__URL
database.pool.max_connections → REDIRECTOR__DATABASE__POOL__MAX_CONNECTIONS
redis.url                 →  REDIRECTOR__REDIS__URL
redis.cache_ttl_seconds   →  REDIRECTOR__REDIS__CACHE_TTL_SECONDS
interstitial.delay_seconds → REDIRECTOR__INTERSTITIAL__DELAY_SECONDS
metrics.basic_auth.username → REDIRECTOR__METRICS__BASIC_AUTH__USERNAME
metrics.basic_auth.password → REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD
rate_limit.requests_per_second → REDIRECTOR__RATE_LIMIT__REQUESTS_PER_SECOND
rate_limit.burst          →  REDIRECTOR__RATE_LIMIT__BURST
admin.enabled             →  REDIRECTOR__ADMIN__ENABLED
admin.session_ttl_hours   →  REDIRECTOR__ADMIN__SESSION_TTL_HOURS
```

#### Примеры для разных платформ

**Railway / Render / Fly.io** (PaaS с управляемыми базами):

```bash
# Эти переменные обычно устанавливаются платформой автоматически:
DATABASE_URL=postgres://user:pass@host:5432/db
REDIS_URL=redis://host:6379
PORT=3000

# Конфигурация через base64:
CONFIG_BASE64=c2VydmVyOgogIGhvc3Q6IC...

# Или переопределение отдельных значений:
REDIRECTOR__HASHIDS__SALTS__0=my-secret-salt
REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD=strong-password
```

**Docker / Docker Compose**:

```yaml
services:
  redirector:
    image: ghcr.io/brilliant-almazov/redirector:latest
    ports:
      - "8080:8080"
    environment:
      DATABASE_URL: "postgres://user:pass@postgres:5432/redirector"
      REDIS_URL: "redis://redis:6379"
      CONFIG_BASE64: "${CONFIG_BASE64}"
      # Или переопределение отдельных значений поверх конфига:
      REDIRECTOR__RATE_LIMIT__REQUESTS_PER_SECOND: "2000"
      REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD: "${METRICS_PASSWORD}"
    volumes:
      - ./config.yaml:/app/config.yaml  # не обязательно при CONFIG_BASE64
    depends_on:
      - postgres
      - redis
```

**Kubernetes**:

```yaml
apiVersion: apps/v1
kind: Deployment
spec:
  template:
    spec:
      containers:
        - name: redirector
          image: ghcr.io/brilliant-almazov/redirector:latest
          env:
            - name: DATABASE_URL
              valueFrom:
                secretKeyRef:
                  name: redirector-secrets
                  key: database-url
            - name: REDIS_URL
              valueFrom:
                secretKeyRef:
                  name: redirector-secrets
                  key: redis-url
            - name: REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD
              valueFrom:
                secretKeyRef:
                  name: redirector-secrets
                  key: metrics-password
            - name: CONFIG_BASE64
              valueFrom:
                configMapKeyRef:
                  name: redirector-config
                  key: config-base64
```

**Минимальная настройка (только env-переменные, без конфиг-файла)**:

```bash
export CONFIG_BASE64=$(cat <<'YAML' | base64
hashids:
  salts:
    - "my-secret-salt"
metrics:
  basic_auth:
    username: prometheus
    password: change-me
YAML
)
export DATABASE_URL=postgres://user:pass@localhost:5432/db
export REDIS_URL=redis://localhost:6379
export PORT=3000

./redirector
```

#### Конфигурация через Base64

Для окружений, где монтирование файлов затруднено (PaaS, serverless, CI/CD), передавайте конфигурацию в формате Base64:

```bash
# Кодирование
cat config.yaml | base64

# Проверка (декодирование)
echo "$CONFIG_BASE64" | base64 -d
```

`CONFIG_BASE64` имеет приоритет над `CONFIG_PATH`. Переменные окружения (`REDIRECTOR__*` и PaaS-переменные) применяются **поверх** декодированного конфига.

## База данных

Сервису нужна простая связь: **ID → URL**

Настройте таблицу и колонки в `config.yaml`:

```yaml
database:
  query:
    table: "dictionary.urls"    # Имя таблицы
    id_column: "id"             # Колонка ID (BIGINT)
    url_column: "name"          # Колонка URL (VARCHAR)
```

Пример схемы таблицы:

```sql
CREATE TABLE dictionary.urls (
    id BIGINT PRIMARY KEY,
    name VARCHAR(4096) NOT NULL
);
```

## Эндпоинты

| Эндпоинт | Авторизация | Описание |
|----------|-------------|----------|
| `GET /` | Нет | Главная страница |
| `GET /r/{hashid}` | Нет | Редирект с промежуточной страницей |
| `GET /d/{hashid}` | Нет | Демо-редирект (синтетическое нагрузочное тестирование) |
| `GET /health` | Нет | Проверка здоровья |
| `GET /metrics` | Basic | Prometheus метрики |
| `GET /admin` | Session | Вход в админ-панель |
| `GET /admin/dashboard` | Session | Админ-панель |

## Админ-панель

Сервис включает опциональную админ-панель для мониторинга метрик в реальном времени.

### Настройка

1. **Сгенерируйте хэш пароля:**

```bash
cargo run --bin hash_password
# Введите пароль или:
cargo run --bin hash_password -- "ваш-пароль"
```

2. **Добавьте в config.yaml:**

```yaml
admin:
  enabled: true
  session_ttl_hours: 24
  users:
    - username: admin
      password_hash: "$argon2id$v=19$m=19456,t=2,p=1$..."  # из шага 1
```

3. **Откройте панель:**

Перейдите на `http://localhost:8080/admin` и войдите с вашими учётными данными.

### Возможности

- Графики RPS и задержки в реальном времени
- Системные метрики (CPU, память, uptime)
- Мониторинг cache hit rate
- Список последних редиректов
- Симуляция нагрузки для тестирования
- Четыре темы: Светлая, Тёмная, Серая, Тёплая

## Аналитика событий

Опциональный пайплайн публикации событий для аналитики редиректов. При включении каждое событие редиректа публикуется в RabbitMQ и обрабатывается отдельным бинарником, который записывает данные в PostgreSQL с обогащением.

> **Полная документация**: [EVENT_ANALYTICS.md](EVENT_ANALYTICS.md)

### Возможности

- **Fire-and-forget публикация** — задержка редиректа не зависит от доступности очереди
- **Батчинг** — события группируются по размеру (100) или времени (1 сек)
- **Парсинг User-Agent** — браузер, версия, ОС, тип устройства через woothee
- **GeoIP обогащение** — страна и город из IP (MaxMind mmdb с горячей перезагрузкой)
- **Дедупликация ссылок** — MD5-дедупликация для referers и user agents
- **Месячное партиционирование** — автосоздание партиций для `redirect_events`

### Быстрый старт

```bash
# Включить в config.yaml
events:
  enabled: true
  rabbitmq:
    url: amqp://guest:guest@localhost:5672/%2f

# Или через переменные окружения
REDIRECTOR__EVENTS__ENABLED=true
RABBITMQ_URL=amqp://guest:guest@localhost:5672/%2f

# Запустить консьюмер
RABBITMQ_URL=amqp://... DATABASE_URL=postgres://... cargo run --bin event_consumer
```

### Docker Compose с событиями

```yaml
services:
  redirector:
    build: .
    environment:
      - REDIRECTOR__EVENTS__ENABLED=true
    depends_on: [redis, rabbitmq]

  event_consumer:
    build: .
    command: ["./event_consumer"]
    environment:
      - RABBITMQ_URL=amqp://guest:guest@rabbitmq:5672/%2f
      - DATABASE_URL=postgres://postgres:postgres@analytics-db:5432/analytics
      - GEOIP_DB_PATH=/data/GeoLite2-City.mmdb  # опционально
    depends_on: [rabbitmq, analytics-db]

  rabbitmq:
    image: rabbitmq:4-management-alpine
    ports: ["5672:5672", "15672:15672"]

  analytics-db:
    image: postgres:16-alpine
    environment:
      POSTGRES_DB: analytics
```

## Метрики

Сервис предоставляет Prometheus метрики на `/metrics` (требуется Basic Auth):

### Метрики сервиса
```
redirector_up 1
redirector_build_info{version="0.1.0"} 1
redirector_uptime_seconds 3600.5
```

### Метрики запросов
```
redirect_requests_total 150000
not_found_requests_total 50
request_duration_seconds{quantile="0.5"} 0.040
request_duration_seconds{quantile="0.99"} 0.081
```

### Метрики кэша
```
cache_hits_total 140000
cache_misses_total 10000
cache_get_duration_seconds{quantile="0.5"} 0.002
cache_set_duration_seconds{quantile="0.5"} 0.002
```

### Метрики БД
```
db_queries_total 10000
db_hits_total 9950
db_misses_total 50
db_query_duration_seconds{quantile="0.5"} 0.035
db_rate_limit_exceeded_total 0
circuit_breaker_rejections_total 0
```

### Метрики событий (при включении)
```
events_published 50000
events_dropped 0
events_publish_errors 0
events_serialize_errors 0
rabbitmq_connected 1
geoip_reloads_total 0
```

## Как это работает

1. Пользователь посещает `/r/{hashid}` (например, `/r/abc123`)
2. Сервис декодирует hashid в числовой ID
3. Проверяет кэш Redis на наличие URL
4. При промахе кэша — запрос к PostgreSQL
5. Кэширует результат в Redis
6. Показывает промежуточную страницу с отсчётом
7. После отсчёта редиректит на целевой URL

```
┌──────┐     ┌───────────┐     ┌───────┐     ┌──────────┐
│Клиент│────▶│Redirector │────▶│ Redis │────▶│PostgreSQL│
└──────┘     └───────────┘     └───────┘     └──────────┘
                  │  │
                  │  └──────────────────┐ (опционально)
                  ▼                     ▼
           ┌─────────────┐     ┌──────────────┐     ┌──────────────┐
           │Промежуточная│     │   RabbitMQ   │────▶│Event Consumer│
           │  страница   │     └──────────────┘     └──────┬───────┘
           └─────────────┘                                 │
                                                    ┌──────▼───────┐
                                                    │  Analytics   │
                                                    │  PostgreSQL  │
                                                    └──────────────┘
```

## Сборка

```bash
# Сборка
cargo build --release

# Запуск тестов
cargo test

# Запуск с покрытием
cargo llvm-cov --text

# Проверка стиля кода
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

## Лицензия

MIT License - подробности в [LICENSE](../LICENSE).

## Участие в разработке

Вклад приветствуется! Пожалуйста:

1. Форкните репозиторий
2. Создайте feature-ветку
3. Отправьте Pull Request

Защищённая master-ветка требует ревью PR.
