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
- 🎨 **Beautiful pages** - Clean 404 and index pages with 3 themes
- 🔑 **Multiple salts** - Hashid salt rotation support for migration
- 📱 **Admin Dashboard** - Real-time metrics monitoring with SSE

## Screenshots

| Light | Dark | Warm |
|-------|------|------|
| ![Dashboard Light](docs/screenshots/dashboard-light.png) | ![Dashboard Dark](docs/screenshots/dashboard-dark.png) | ![Dashboard Warm](docs/screenshots/dashboard-warm.png) |
| ![Login Light](docs/screenshots/login-light.png) | ![Login Dark](docs/screenshots/login-dark.png) | ![Login Warm](docs/screenshots/login-warm.png) |
| ![404 Light](docs/screenshots/not-found-light.png) | ![404 Dark](docs/screenshots/not-found-dark.png) | ![404 Warm](docs/screenshots/not-found-warm.png) |

| Index Page | Interstitial |
|------------|--------------|
| ![Index](docs/screenshots/index.png) | ![Interstitial](docs/screenshots/interstitial.png) |

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

All config values support `${VAR}` substitution. Additionally:

- `CONFIG_FILE` - Path to config file (default: `config.yaml`)

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
- Three themes: Light, Dark, Warm

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
