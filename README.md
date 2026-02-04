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
- **Password Hashing**: Argon2

> **Note**: The storage and cache layers are abstracted and can be replaced with any compatible data source. Currently in active development.

## Quick Start

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

## Configuration

Create `config.yaml`:

```yaml
server:
  host: "0.0.0.0"
  port: 8080

hashids:
  salts:
    - ${HASHID_SALT}          # Primary salt
    - ${HASHID_SALT_OLD}      # Optional: old salt for migration
  min_length: 6

redis:
  url: ${REDIS_URL}
  cache_ttl_seconds: 86400    # 24 hours

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
    table: "dictionary.urls"    # Your table name
    id_column: "id"             # ID column
    url_column: "name"          # URL column

interstitial:
  delay_seconds: 5            # Countdown before redirect

metrics:
  basic_auth:
    username: prometheus
    password: ${METRICS_PASSWORD}

rate_limit:
  requests_per_second: 1000
  burst: 100
```

### Configuration Options

#### Server

| Option | Default | Description |
|--------|---------|-------------|
| `host` | `0.0.0.0` | Bind address |
| `port` | `8080` | HTTP port |

#### Hashids

| Option | Default | Description |
|--------|---------|-------------|
| `salts` | *required* | List of hashid salts (first = primary) |
| `min_length` | `6` | Minimum hashid length |

#### Redis

| Option | Default | Description |
|--------|---------|-------------|
| `url` | *required* | Redis connection URL |
| `cache_ttl_seconds` | `86400` | Cache TTL in seconds |

#### Database

| Option | Default | Description |
|--------|---------|-------------|
| `url` | *required* | PostgreSQL connection URL |
| `pool.max_connections` | `3` | Connection pool size |
| `pool.connect_timeout_seconds` | `3` | Connection timeout |
| `rate_limit.max_requests_per_second` | `50` | DB rate limit |
| `circuit_breaker.failure_threshold` | `3` | Failures before opening |
| `circuit_breaker.reset_timeout_seconds` | `60` | Circuit reset timeout |

#### Rate Limit (Global)

| Option | Default | Description |
|--------|---------|-------------|
| `requests_per_second` | `1000` | Global rate limit |
| `burst` | `100` | Burst allowance |

### Environment Variables

There are **three ways** to configure the service, listed by priority (highest first):

| Priority | Method | Use Case |
|----------|--------|----------|
| 1 | `REDIRECTOR__*` env vars | Override individual values |
| 2 | Standard PaaS env vars (`DATABASE_URL`, etc.) | PaaS platforms (Railway, Heroku, Render) |
| 3 | Config file (`config.yaml` or `CONFIG_BASE64`) | Base configuration |

#### Special Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `CONFIG_PATH` | `config.yaml` | Path to YAML config file |
| `CONFIG_BASE64` | — | Base64-encoded YAML config (takes priority over `CONFIG_PATH`) |

#### Standard PaaS Environment Variables

These are automatically recognized and applied. Most PaaS platforms set them for you:

| Variable | Config Path | Example |
|----------|-------------|---------|
| `DATABASE_URL` | `database.url` | `postgres://user:pass@host:5432/db` |
| `REDIS_URL` | `redis.url` | `redis://host:6379` |
| `PORT` | `server.port` | `3000` |
| `HASHIDS_SALTS` | `hashids.salts` | `new-salt,old-salt` (comma-separated) |

> **Priority rule**: If both `DATABASE_URL` and `REDIRECTOR__DATABASE__URL` are set, the `REDIRECTOR__` prefixed version wins. Similarly, `REDIRECTOR__HASHIDS__SALTS__0` takes priority over `HASHIDS_SALTS`.

#### Prefixed Environment Variables (`REDIRECTOR__*`)

Any config value can be overridden using the `REDIRECTOR__` prefix with `__` (double underscore) as the nesting separator. Below is the **complete reference** of all overridable variables:

##### Server

| Environment Variable | Config Path | Default | Description |
|---------------------|-------------|---------|-------------|
| `REDIRECTOR__SERVER__HOST` | `server.host` | `0.0.0.0` | Bind address |
| `REDIRECTOR__SERVER__PORT` | `server.port` | `8080` | HTTP port |

##### Hashids

| Environment Variable | Config Path | Default | Description |
|---------------------|-------------|---------|-------------|
| `REDIRECTOR__HASHIDS__SALTS__0` | `hashids.salts[0]` | *required* | Primary hashid salt |
| `REDIRECTOR__HASHIDS__SALTS__1` | `hashids.salts[1]` | — | Old salt (for migration) |
| `REDIRECTOR__HASHIDS__MIN_LENGTH` | `hashids.min_length` | `6` | Minimum hashid length |

> **Arrays**: List items are indexed with `__0`, `__1`, `__2`, etc. For hashid salt rotation, set `__0` for the new salt and `__1` for the old one.

##### Redis / Cache

| Environment Variable | Config Path | Default | Description |
|---------------------|-------------|---------|-------------|
| `REDIRECTOR__REDIS__URL` | `redis.url` | *required* | Redis connection URL |
| `REDIRECTOR__REDIS__CACHE_TTL_SECONDS` | `redis.cache_ttl_seconds` | `86400` | Cache TTL (seconds). `86400` = 24h |

##### Database

| Environment Variable | Config Path | Default | Description |
|---------------------|-------------|---------|-------------|
| `REDIRECTOR__DATABASE__URL` | `database.url` | *required* | PostgreSQL connection URL |
| `REDIRECTOR__DATABASE__POOL__MAX_CONNECTIONS` | `database.pool.max_connections` | `3` | Connection pool size |
| `REDIRECTOR__DATABASE__POOL__CONNECT_TIMEOUT_SECONDS` | `database.pool.connect_timeout_seconds` | `3` | Connection timeout (seconds) |
| `REDIRECTOR__DATABASE__RATE_LIMIT__MAX_REQUESTS_PER_SECOND` | `database.rate_limit.max_requests_per_second` | `50` | Max DB queries per second |
| `REDIRECTOR__DATABASE__CIRCUIT_BREAKER__FAILURE_THRESHOLD` | `database.circuit_breaker.failure_threshold` | `3` | Consecutive failures before circuit opens |
| `REDIRECTOR__DATABASE__CIRCUIT_BREAKER__RESET_TIMEOUT_SECONDS` | `database.circuit_breaker.reset_timeout_seconds` | `60` | Seconds before half-open retry |
| `REDIRECTOR__DATABASE__QUERY__TABLE` | `database.query.table` | `dictionary.urls` | Table name for URL lookups |
| `REDIRECTOR__DATABASE__QUERY__ID_COLUMN` | `database.query.id_column` | `id` | Column name for numeric ID |
| `REDIRECTOR__DATABASE__QUERY__URL_COLUMN` | `database.query.url_column` | `name` | Column name for target URL |

##### Interstitial Page

| Environment Variable | Config Path | Default | Description |
|---------------------|-------------|---------|-------------|
| `REDIRECTOR__INTERSTITIAL__DELAY_SECONDS` | `interstitial.delay_seconds` | `5` | Countdown before redirect |

##### Metrics

| Environment Variable | Config Path | Default | Description |
|---------------------|-------------|---------|-------------|
| `REDIRECTOR__METRICS__BASIC_AUTH__USERNAME` | `metrics.basic_auth.username` | *required* | Username for `/metrics` endpoint |
| `REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD` | `metrics.basic_auth.password` | *required* | Password for `/metrics` endpoint |

##### Rate Limiting (Global)

| Environment Variable | Config Path | Default | Description |
|---------------------|-------------|---------|-------------|
| `REDIRECTOR__RATE_LIMIT__REQUESTS_PER_SECOND` | `rate_limit.requests_per_second` | `1000` | Max requests per second |
| `REDIRECTOR__RATE_LIMIT__BURST` | `rate_limit.burst` | `100` | Burst allowance above RPS limit |

##### Admin Dashboard

| Environment Variable | Config Path | Default | Description |
|---------------------|-------------|---------|-------------|
| `REDIRECTOR__ADMIN__ENABLED` | `admin.enabled` | `false` | Enable admin dashboard |
| `REDIRECTOR__ADMIN__SESSION_SECRET` | `admin.session_secret` | `change-me-...` | Session signing secret (min 32 chars) |
| `REDIRECTOR__ADMIN__SESSION_TTL_HOURS` | `admin.session_ttl_hours` | `24` | Session lifetime in hours |

> **Note**: Admin users (`admin.users`) with `username` and `password_hash` cannot be set via env vars due to their complex structure. Define them in the config file or `CONFIG_BASE64`.

#### Examples by Deployment Platform

**Railway / Render / Fly.io** (PaaS with managed databases):

```bash
# These are usually set automatically by the platform:
DATABASE_URL=postgres://user:pass@host:5432/db
REDIS_URL=redis://host:6379
PORT=3000

# Set your config via base64:
CONFIG_BASE64=c2VydmVyOgogIGhvc3Q6IC...

# Or override individual values:
REDIRECTOR__HASHIDS__SALTS__0=my-secret-salt
REDIRECTOR__METRICS__BASIC_AUTH__USERNAME=prometheus
REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD=strong-password
REDIRECTOR__ADMIN__ENABLED=true
REDIRECTOR__ADMIN__SESSION_SECRET=random-32-byte-secret-for-sessions
```

**Docker Compose (full example with all overrides)**:

```yaml
services:
  redirector:
    image: ghcr.io/brilliant-almazov/redirector:latest
    ports:
      - "8080:8080"
    environment:
      # --- Connection URLs (PaaS-style) ---
      DATABASE_URL: "postgres://redirector:${DB_PASSWORD}@postgres:5432/redirector"
      REDIS_URL: "redis://redis:6379"

      # --- Config file ---
      CONFIG_BASE64: "${CONFIG_BASE64}"

      # --- Server ---
      REDIRECTOR__SERVER__HOST: "0.0.0.0"
      REDIRECTOR__SERVER__PORT: "8080"

      # --- Hashid salts ---
      REDIRECTOR__HASHIDS__SALTS__0: "${HASHID_SALT}"        # primary salt
      REDIRECTOR__HASHIDS__SALTS__1: "${HASHID_SALT_OLD}"    # old salt for migration
      REDIRECTOR__HASHIDS__MIN_LENGTH: "6"

      # --- Redis cache ---
      REDIRECTOR__REDIS__CACHE_TTL_SECONDS: "43200"          # 12 hours

      # --- Database pool & resilience ---
      REDIRECTOR__DATABASE__POOL__MAX_CONNECTIONS: "5"
      REDIRECTOR__DATABASE__POOL__CONNECT_TIMEOUT_SECONDS: "5"
      REDIRECTOR__DATABASE__RATE_LIMIT__MAX_REQUESTS_PER_SECOND: "100"
      REDIRECTOR__DATABASE__CIRCUIT_BREAKER__FAILURE_THRESHOLD: "5"
      REDIRECTOR__DATABASE__CIRCUIT_BREAKER__RESET_TIMEOUT_SECONDS: "30"

      # --- Custom table mapping ---
      REDIRECTOR__DATABASE__QUERY__TABLE: "public.short_urls"
      REDIRECTOR__DATABASE__QUERY__ID_COLUMN: "id"
      REDIRECTOR__DATABASE__QUERY__URL_COLUMN: "target_url"

      # --- Interstitial ---
      REDIRECTOR__INTERSTITIAL__DELAY_SECONDS: "3"

      # --- Metrics auth ---
      REDIRECTOR__METRICS__BASIC_AUTH__USERNAME: "prometheus"
      REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD: "${METRICS_PASSWORD}"

      # --- Global rate limit ---
      REDIRECTOR__RATE_LIMIT__REQUESTS_PER_SECOND: "2000"
      REDIRECTOR__RATE_LIMIT__BURST: "200"

      # --- Admin dashboard ---
      REDIRECTOR__ADMIN__ENABLED: "true"
      REDIRECTOR__ADMIN__SESSION_SECRET: "${SESSION_SECRET}"
      REDIRECTOR__ADMIN__SESSION_TTL_HOURS: "8"
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
            - name: REDIRECTOR__HASHIDS__SALTS__0
              valueFrom:
                secretKeyRef:
                  name: redirector-secrets
                  key: hashid-salt
            - name: REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD
              valueFrom:
                secretKeyRef:
                  name: redirector-secrets
                  key: metrics-password
            - name: REDIRECTOR__ADMIN__SESSION_SECRET
              valueFrom:
                secretKeyRef:
                  name: redirector-secrets
                  key: session-secret
            - name: CONFIG_BASE64
              valueFrom:
                configMapKeyRef:
                  name: redirector-config
                  key: config-base64
```

**Plain Docker (single command)**:

```bash
docker run -p 8080:8080 \
  -e DATABASE_URL="postgres://user:pass@host:5432/db" \
  -e REDIS_URL="redis://host:6379" \
  -e REDIRECTOR__HASHIDS__SALTS__0="my-secret-salt" \
  -e REDIRECTOR__METRICS__BASIC_AUTH__USERNAME="prometheus" \
  -e REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD="strong-password" \
  -e REDIRECTOR__INTERSTITIAL__DELAY_SECONDS="3" \
  -e CONFIG_BASE64="$(cat config.yaml | base64)" \
  ghcr.io/brilliant-almazov/redirector:latest
```

**Minimal setup (env vars only, no config file)**:

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

#### Salt Rotation via Environment Variables

When rotating hashid salts, the service tries salts in order — the first match wins. Set the new salt first so new links use it, and keep the old salt for backward compatibility:

**Option 1: Single variable with comma separator** (recommended):

```bash
# Before rotation
HASHIDS_SALTS=original-salt

# After rotation — new salt first, old salt for existing links
HASHIDS_SALTS=new-salt,original-salt
```

**Option 2: Indexed variables**:

```bash
# Before rotation
REDIRECTOR__HASHIDS__SALTS__0=original-salt

# After rotation
REDIRECTOR__HASHIDS__SALTS__0=new-salt
REDIRECTOR__HASHIDS__SALTS__1=original-salt
```

> **Note**: If `REDIRECTOR__HASHIDS__SALTS__0` is set, `HASHIDS_SALTS` is ignored.

#### Base64 Configuration

For environments where mounting config files is not practical (PaaS, serverless, CI/CD), pass the entire config as a base64-encoded string:

```bash
# Encode
cat config.yaml | base64

# Decode (to verify)
echo "$CONFIG_BASE64" | base64 -d
```

`CONFIG_BASE64` takes priority over `CONFIG_PATH`. Environment variable overrides (`REDIRECTOR__*` and PaaS vars) are applied **on top** of the decoded config.

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
                  │
                  ▼
           ┌─────────────┐
           │ Interstitial│
           │    Page     │
           └─────────────┘
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
