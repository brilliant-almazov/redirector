# redirector

**English** | [Русский](docs/README.ru.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yaml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yaml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Safe URL redirect service with interstitial pages and hashid-based short links.

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

- 🔗 **Hashid URLs** - Short, unique, non-sequential IDs (e.g., `/abc123`)
- ⏱️ **Interstitial page** - Countdown timer shows target URL before redirect
- ⚡ **Redis caching** - Fast lookups with configurable TTL
- 🛡️ **Circuit breaker** - Database protection against cascading failures
- 🚦 **Rate limiting** - Both global and database-level rate limits
- 📊 **Prometheus metrics** - Full observability with Basic Auth protection
- 🎨 **Beautiful pages** - Clean 404 and index pages
- 🔑 **Multiple salts** - Hashid salt rotation support for migration

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

Currently uses a built-in query for `dictionary.urls` + `dictionary.domains` tables.

> **Coming soon**: Configurable query structure. You'll be able to specify your own ID and URL columns declaratively, regardless of your database schema.

## Endpoints

| Endpoint | Auth | Description |
|----------|------|-------------|
| `GET /` | No | Index page |
| `GET /{hashid}` | No | Redirect with interstitial |
| `GET /health` | No | Health check |
| `GET /metrics` | Basic | Prometheus metrics |

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

1. User visits `/{hashid}` (e.g., `/abc123`)
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
