# redirector

> **High-performance URL shortener and redirect service** built with Rust, Axum, Redis, and PostgreSQL. Features secure interstitial pages, real-time admin dashboard, and enterprise-grade observability.

**English** | [Русский](docs/README.ru.md) | [中文](docs/README.zh.md) | [हिंदी](docs/README.hi.md) | [Español](docs/README.es.md) | [Português](docs/README.pt.md) | [Français](docs/README.fr.md) | [Deutsch](docs/README.de.md) | [日本語](docs/README.ja.md) | [한국어](docs/README.ko.md) | [Polski](docs/README.pl.md) | [Nederlands](docs/README.nl.md) | [Italiano](docs/README.it.md) | [Türkçe](docs/README.tr.md) | [Українська](docs/README.uk.md) | [עברית](docs/README.he.md) | [Bahasa Indonesia](docs/README.id.md) | [Tiếng Việt](docs/README.vi.md) | [Svenska](docs/README.sv.md) | [Suomi](docs/README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![Docker Image Size](https://ghcr-badge.egpl.dev/brilliant-almazov/redirector/size?v=1)](https://github.com/brilliant-almazov/redirector/pkgs/container/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

**Keywords**: url shortener, link shortener, redirect service, rust web service, axum framework, redis cache, postgresql, prometheus metrics, hashids, short links, interstitial pages, safe redirects, high performance, microservice

Safe URL redirect service with interstitial pages and hashid-based short links. Perfect for internal tools, enterprise link management, and branded short URL services.

### Performance

| Scenario | RPS | Avg Latency | P99 Latency |
|----------|-----|-------------|-------------|
| 100% Cache Hit | **7,800+** | ~14ms | ~50ms |
| Cache Miss (10K URLs) | **2,300+** | ~44ms | ~81ms |

**Test conditions**: wrk -t4 -c100 -d30s, PostgreSQL 15, Dragonfly (Redis), macOS M1 (Docker)

> ⚠️ Results are from Docker on macOS with VM overhead. Native Linux deployment expected to be **3-5x faster**.

## Problem

Sharing long URLs is inconvenient. URL shorteners exist but often redirect immediately, which can be a security risk. Users should see where they're going before being redirected.

**redirector** provides safe redirects with:
- Interstitial page showing target URL before redirect
- Countdown timer for user awareness
- Beautiful, branded pages

## Features

- 🔗 **Hashid URLs** - Short, unique, non-sequential IDs (e.g., `/r/abc123`)
- ⏱️ **Interstitial page** - Countdown timer shows target URL before redirect
- ⚡ **Redis caching** - Fast lookups with configurable TTL
- 🛡️ **Circuit breaker** - Database protection against cascading failures
- 🚦 **Rate limiting** - Both global and database-level rate limits
- 📊 **Prometheus metrics** - Full observability with Basic Auth protection
- 🎨 **Beautiful pages** - Clean 404 and index pages with 4 themes
- 🔑 **Multiple salts** - Hashid salt rotation support for migration
- 📱 **Admin Dashboard** - Real-time metrics monitoring with SSE
- 📤 **Event Analytics** - Optional RabbitMQ event publishing with PostgreSQL consumer

## Screenshots

| Light | Dark | Gray | Warm |
|-------|------|------|------|
| ![Dashboard Light](docs/screenshots/dashboard-light.png) | ![Dashboard Dark](docs/screenshots/dashboard-dark.png) | ![Dashboard Gray](docs/screenshots/dashboard-gray.png) | ![Dashboard Warm](docs/screenshots/dashboard-warm.png) |
| ![Login Light](docs/screenshots/login-light.png) | ![Login Dark](docs/screenshots/login-dark.png) | ![Login Gray](docs/screenshots/login-gray.png) | ![Login Warm](docs/screenshots/login-warm.png) |
| ![Index Light](docs/screenshots/index-light.png) | ![Index Dark](docs/screenshots/index-dark.png) | ![Index Gray](docs/screenshots/index-gray.png) | ![Index Warm](docs/screenshots/index-warm.png) |
| ![Interstitial Light](docs/screenshots/interstitial-light.png) | ![Interstitial Dark](docs/screenshots/interstitial-dark.png) | ![Interstitial Gray](docs/screenshots/interstitial-gray.png) | ![Interstitial Warm](docs/screenshots/interstitial-warm.png) |
| ![404 Light](docs/screenshots/404-light.png) | ![404 Dark](docs/screenshots/404-dark.png) | ![404 Gray](docs/screenshots/404-gray.png) | ![404 Warm](docs/screenshots/404-warm.png) |

### Load Test Modal

| Light | Dark | Gray | Warm |
|-------|------|------|------|
| ![Modal Light](docs/screenshots/dashboard-modal-light.png) | ![Modal Dark](docs/screenshots/dashboard-modal-dark.png) | ![Modal Gray](docs/screenshots/dashboard-modal-gray.png) | ![Modal Warm](docs/screenshots/dashboard-modal-warm.png) |

## Tech Stack

- **Language**: Rust (async with Tokio)
- **Web Framework**: Axum
- **Cache**: Redis-compatible (Redis, Dragonfly, Valkey, KeyDB, etc.)
- **Database**: PostgreSQL (pluggable storage layer)
- **Metrics**: Prometheus + metrics-rs
- **Message Queue**: RabbitMQ (optional, for event analytics)
- **Password Hashing**: Argon2

> **Note**: The storage and cache layers are abstracted and can be replaced with any compatible data source. Currently in active development.

## Quick Start

### Docker

```bash
docker run -p 8080:8080 \
  -e DATABASE_URL="postgres://user:pass@host:5432/db" \
  -e REDIS_URL="redis://host:6379" \
  -e HASHIDS_SALTS="my-secret-salt" \
  -e METRICS_USERNAME="prometheus" \
  -e METRICS_PASSWORD="strong-password" \
  ghcr.io/brilliant-almazov/redirector:latest
```

### Docker Compose

```yaml
services:
  redirector:
    image: ghcr.io/brilliant-almazov/redirector:latest
    ports:
      - "8080:8080"
    environment:
      DATABASE_URL: "postgres://redirector:${POSTGRES_PASSWORD}@postgres:5432/redirector"
      REDIS_URL: "redis://redis:6379"
      HASHIDS_SALTS: "${HASHID_SALT}"
      METRICS_USERNAME: "prometheus"
      METRICS_PASSWORD: "${METRICS_PASSWORD}"
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

## Configuration

The service is configured **exclusively via environment variables**. No config files are required.

### Required Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | `postgres://user:pass@host:5432/db` |
| `REDIS_URL` | Redis connection string | `redis://host:6379` |
| `HASHIDS_SALTS` | Comma-separated hashid salts | `primary-salt,fallback-salt` |
| `METRICS_USERNAME` | Basic auth username for `/metrics` | `prometheus` |
| `METRICS_PASSWORD` | Basic auth password for `/metrics` | `strong-password` |

### Optional Environment Variables

#### Server

| Variable | Default | Description |
|----------|---------|-------------|
| `HOST` | `0.0.0.0` | Bind address |
| `PORT` | `8080` | HTTP port |

#### Hashids

| Variable | Default | Description |
|----------|---------|-------------|
| `HASHIDS_MIN_LENGTH` | `6` | Minimum hashid length |

#### Database

| Variable | Default | Description |
|----------|---------|-------------|
| `DB_TABLE` | `dictionary.urls` | Table name for URL lookups |
| `DB_ID_COLUMN` | `id` | Column name for numeric ID |
| `DB_URL_COLUMN` | `name` | Column name for target URL |

#### Events / Analytics

| Variable | Default | Description |
|----------|---------|-------------|
| `EVENTS_ENABLED` | `false` | Enable event publishing to RabbitMQ |
| `RABBITMQ_URL` | *required if EVENTS_ENABLED=true* | RabbitMQ connection URL |
| `RABBITMQ_QUEUE` | `events.analytics` | Queue name for events |

#### Admin Dashboard

| Variable | Default | Description |
|----------|---------|-------------|
| `ADMIN_ENABLED` | `false` | Enable admin dashboard |
| `ADMIN_SESSION_SECRET` | *required if ADMIN_ENABLED=true* | Session signing secret (min 32 chars) |
| `ADMIN_USERS` | `[]` | JSON array of admin users |

> **Admin users format**: `ADMIN_USERS='[{"username":"admin","password_hash":"$argon2id$v=19$m=19456,t=2,p=1$..."}]'`
>
> Generate password hashes with: `cargo run --bin hash_password -- "your-password"`

### Full Environment Variables Reference

| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_URL` | *required* | PostgreSQL connection URL |
| `REDIS_URL` | *required* | Redis connection URL |
| `HASHIDS_SALTS` | *required* | Comma-separated hashid salts |
| `METRICS_USERNAME` | *required* | Username for `/metrics` endpoint |
| `METRICS_PASSWORD` | *required* | Password for `/metrics` endpoint |
| `HOST` | `0.0.0.0` | Server bind address |
| `PORT` | `8080` | Server HTTP port |
| `HASHIDS_MIN_LENGTH` | `6` | Minimum hashid length |
| `REDIS_CACHE_TTL` | `86400` | Cache TTL in seconds (default: 24h) |
| `DB_MAX_CONNECTIONS` | `3` | Database connection pool size |
| `DB_CONNECT_TIMEOUT` | `3` | Database connection timeout (seconds) |
| `DB_RPS` | `50` | Max database queries per second |
| `CB_FAILURE_THRESHOLD` | `3` | Circuit breaker failure threshold |
| `CB_RESET_TIMEOUT` | `60` | Circuit breaker reset timeout (seconds) |
| `DB_TABLE` | `dictionary.urls` | Database table name |
| `DB_ID_COLUMN` | `id` | ID column name |
| `DB_URL_COLUMN` | `name` | URL column name |
| `INTERSTITIAL_DELAY` | `5` | Interstitial countdown (seconds) |
| `RATE_LIMIT_RPS` | `1000` | Global rate limit (requests/second) |
| `RATE_LIMIT_BURST` | `100` | Rate limit burst allowance |
| `EVENTS_ENABLED` | `false` | Enable RabbitMQ event publishing |
| `RABBITMQ_URL` | `amqp://guest:guest@localhost:5672/%2f` | RabbitMQ connection URL |
| `RABBITMQ_QUEUE` | `events.analytics` | RabbitMQ queue name |
| `ADMIN_ENABLED` | `false` | Enable admin dashboard |
| `ADMIN_SESSION_SECRET` | `change-me-in-production-32-bytes!` | Session signing secret |
| `ADMIN_SESSION_TTL_HOURS` | `24` | Session lifetime (hours) |
| `ADMIN_USERS` | `[]` | Admin users JSON array |

### Deployment Examples

#### Railway / Render / Fly.io

```bash
# Required
DATABASE_URL=postgres://user:pass@host:5432/db
REDIS_URL=redis://host:6379
HASHIDS_SALTS=my-secret-salt
METRICS_USERNAME=prometheus
METRICS_PASSWORD=strong-password

# Optional - enable admin dashboard
ADMIN_ENABLED=true
ADMIN_SESSION_SECRET=random-32-byte-secret-for-sessions
ADMIN_USERS='[{"username":"admin","password_hash":"$argon2id$..."}]'

# Optional - enable event analytics
EVENTS_ENABLED=true
RABBITMQ_URL=amqp://guest:guest@host:5672/%2f
```

#### Docker Compose

```yaml
services:
  redirector:
    image: ghcr.io/brilliant-almazov/redirector:latest
    ports:
      - "8080:8080"
    environment:
      DATABASE_URL: "postgres://redirector:${DB_PASSWORD}@postgres:5432/redirector"
      REDIS_URL: "redis://redis:6379"
      HASHIDS_SALTS: "${HASHID_SALT}"
      METRICS_USERNAME: "prometheus"
      METRICS_PASSWORD: "${METRICS_PASSWORD}"
      ADMIN_ENABLED: "true"
      ADMIN_SESSION_SECRET: "${SESSION_SECRET}"
      ADMIN_USERS: '[{"username":"admin","password_hash":"${ADMIN_PASSWORD_HASH}"}]'
    depends_on:
      - postgres
      - redis

  postgres:
    image: postgres:16-alpine
    environment:
      POSTGRES_USER: redirector
      POSTGRES_PASSWORD: ${DB_PASSWORD}
      POSTGRES_DB: redirector

  redis:
    image: redis:7-alpine
```

#### Kubernetes

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
            - name: HASHIDS_SALTS
              valueFrom:
                secretKeyRef:
                  name: redirector-secrets
                  key: hashid-salts
            - name: METRICS_USERNAME
              value: "prometheus"
            - name: METRICS_PASSWORD
              valueFrom:
                secretKeyRef:
                  name: redirector-secrets
                  key: metrics-password
            - name: ADMIN_ENABLED
              value: "true"
            - name: ADMIN_SESSION_SECRET
              valueFrom:
                secretKeyRef:
                  name: redirector-secrets
                  key: session-secret
```

#### Plain Docker

```bash
docker run -p 8080:8080 \
  -e DATABASE_URL="postgres://user:pass@host:5432/db" \
  -e REDIS_URL="redis://host:6379" \
  -e HASHIDS_SALTS="my-secret-salt" \
  -e METRICS_USERNAME="prometheus" \
  -e METRICS_PASSWORD="strong-password" \
  ghcr.io/brilliant-almazov/redirector:latest
```

### Salt Rotation

When rotating hashid salts, the service tries salts in order. Set the new salt first:

```bash
# Before rotation
HASHIDS_SALTS=original-salt

# After rotation — new salt first, old salt for existing links
HASHIDS_SALTS=new-salt,original-salt
```

## Database

The service needs a simple mapping: **ID → URL**

Configure your table and columns in `config.yaml`:

```yaml
database:
  query:
    table: "dictionary.urls"    # Your table name
    id_column: "id"             # ID column (BIGINT)
    url_column: "name"          # URL column (VARCHAR)
```

Example table schema:

```sql
CREATE TABLE dictionary.urls (
    id BIGINT PRIMARY KEY,
    name VARCHAR(4096) NOT NULL
);
```

## Endpoints

| Endpoint | Auth | Description |
|----------|------|-------------|
| `GET /` | No | Index page |
| `GET /r/{hashid}` | No | Redirect with interstitial |
| `GET /d/{hashid}` | No | Demo redirect (synthetic load testing) |
| `GET /health` | No | Health check |
| `GET /metrics` | Basic | Prometheus metrics |
| `GET /admin` | Session | Admin dashboard login |
| `GET /admin/dashboard` | Session | Admin dashboard |

### Demo Endpoints

The `/d/{hashid}` endpoint provides demo redirects for load testing without hitting the real database. It uses pre-generated simulation data from `static/simulation_data.bin` containing sample hashids and URLs. This is useful for:

- Performance testing and benchmarking
- Admin dashboard demonstration
- Development and testing without database setup

## Admin Dashboard

The service includes an optional admin dashboard for monitoring live metrics.

### Setup

1. **Generate password hash:**

```bash
cargo run --bin hash_password
# Enter password when prompted, or:
cargo run --bin hash_password -- "your-password"
```

2. **Add to config.yaml:**

```yaml
admin:
  enabled: true
  session_ttl_hours: 24
  users:
    - username: admin
      password_hash: "$argon2id$v=19$m=19456,t=2,p=1$..."  # from step 1
```

3. **Access dashboard:**

Open `http://localhost:8080/admin` and login with your credentials.

### Features

- Real-time RPS and latency charts
- System metrics (CPU, memory, uptime)
- Cache hit rate monitoring
- Recent redirects list
- Load simulation for testing
- Four themes: Light, Dark, Gray, Warm

## Event Analytics

Optional event publishing pipeline for redirect analytics. When enabled, every redirect event is published to RabbitMQ and consumed by a separate binary that writes to PostgreSQL with rich enrichment.

> **Full documentation**: [docs/EVENT_ANALYTICS.md](docs/EVENT_ANALYTICS.md)

### Features

- **Fire-and-forget publishing** — redirect latency unaffected by queue availability
- **Batching** — events grouped by size (100) or time (1 second)
- **User-Agent parsing** — browser, version, OS, device type via woothee
- **GeoIP enrichment** — country and city from IP (MaxMind mmdb with hot-reload)
- **Reference deduplication** — MD5-based dedup for referers and user agents
- **Monthly partitioning** — automatic partition creation for `redirect_events`
- **Domain normalization** — `WWW.Example.COM` → `example.com`

### Architecture

```
Redirect Handler
    │
    ├── try_send(RedirectEvent) ──► [tokio::mpsc channel]
    │   (non-blocking,                    │
    │    fire-and-forget)                 ▼
    │                              Background Task
    │                              (batch by size/time)
    │                                     │
    │                                     ▼
    │                                [RabbitMQ Queue]
    │                                     │
    │                                     ▼
    │                              Event Consumer
    │                              (separate binary/container)
    │                                     │
    │                                     ▼
    │                              [PostgreSQL Analytics]
    │                              (monthly partitioned)
```

### Quick Start

```bash
# Enable in config.yaml
events:
  enabled: true
  rabbitmq:
    url: amqp://guest:guest@localhost:5672/%2f

# Or via environment
REDIRECTOR__EVENTS__ENABLED=true
RABBITMQ_URL=amqp://guest:guest@localhost:5672/%2f

# Run consumer
RABBITMQ_URL=amqp://... DATABASE_URL=postgres://... cargo run --bin event_consumer
```

### Docker Compose with Events

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
      - GEOIP_DB_PATH=/data/GeoLite2-City.mmdb  # optional
    depends_on: [rabbitmq, analytics-db]

  rabbitmq:
    image: rabbitmq:4-management-alpine
    ports: ["5672:5672", "15672:15672"]

  analytics-db:
    image: postgres:16-alpine
    environment:
      POSTGRES_DB: analytics
```

### Key Design Decisions

- **Never blocks redirects**: `try_send()` on bounded channel, drops events if full
- **Type-safe event batches**: `EventBatch` is a Rust enum tagged by `event_type`
- **Snowflake batch IDs**: Custom epoch 2025-01-01, ~69 years of unique IDs
- **Graceful degradation**: If RabbitMQ is down, redirects continue; events are dropped with metrics

## Metrics

The service exposes comprehensive Prometheus metrics at `/metrics` (requires Basic Auth):

### Service Metrics
```
redirector_up 1
redirector_build_info{version="0.1.0"} 1
redirector_uptime_seconds 3600.5
```

### Request Metrics
```
redirect_requests_total 150000
not_found_requests_total 50
request_duration_seconds{quantile="0.5"} 0.040
request_duration_seconds{quantile="0.99"} 0.081
```

### Cache Metrics
```
cache_hits_total 140000
cache_misses_total 10000
cache_get_duration_seconds{quantile="0.5"} 0.002
cache_set_duration_seconds{quantile="0.5"} 0.002
```

### Database Metrics
```
db_queries_total 10000
db_hits_total 9950
db_misses_total 50
db_errors_total 0
db_query_duration_seconds{quantile="0.5"} 0.035
db_rate_limit_exceeded_total 0
circuit_breaker_rejections_total 0
```

### Rate Limiting
```
rate_limit_exceeded_total 0
```

### Events (when enabled)
```
events_published 50000
events_dropped 0
events_publish_errors 0
events_serialize_errors 0
rabbitmq_connected 1
```

## How It Works

1. User visits `/r/{hashid}` (e.g., `/r/abc123`)
2. Service decodes hashid to numeric ID
3. Checks Redis cache for URL
4. On cache miss, queries PostgreSQL
5. Caches result in Redis
6. Shows interstitial page with countdown
7. After countdown, redirects to target URL

```
┌──────┐     ┌───────────┐     ┌───────┐     ┌──────────┐
│Client│────▶│Redirector │────▶│ Redis │────▶│PostgreSQL│
└──────┘     └───────────┘     └───────┘     └──────────┘
                  │  │
                  │  └──────────────────┐ (optional)
                  ▼                     ▼
           ┌─────────────┐     ┌──────────────┐     ┌──────────────┐
           │ Interstitial│     │   RabbitMQ   │────▶│Event Consumer│
           │    Page     │     └──────────────┘     └──────┬───────┘
           └─────────────┘                                 │
                                                    ┌──────▼───────┐
                                                    │  Analytics   │
                                                    │  PostgreSQL  │
                                                    └──────────────┘
```

## Building

```bash
# Build
cargo build --release

# Run tests
cargo test

# Run with coverage
cargo llvm-cov --text

# Check code style
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Submit a Pull Request

Protected master branch requires PR review.
