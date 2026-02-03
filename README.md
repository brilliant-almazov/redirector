# Redirector

Safe redirect service with hashid decoding and caching.

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yaml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yaml)
[![Coverage](https://codecov.io/gh/brilliant-almazov/redirector/branch/master/graph/badge.svg)](https://codecov.io/gh/brilliant-almazov/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- **Hashid decoding** with multiple salts support (fallback)
- **Redis caching** with configurable TTL
- **Interstitial page** with countdown timer before redirect
- **Rate limiting** to protect main database
- **Circuit breaker** for resilience
- **Prometheus metrics** with Basic Auth
- **Beautiful error pages**

## Quick Start

### Using Docker Compose

```bash
# Copy and configure
cp config.yaml.example config.yaml
# Edit config.yaml with your settings

# Start
docker-compose up -d
```

### Manual

```bash
# Install dependencies
cargo build --release

# Configure
cp config.yaml.example config.yaml

# Run
CONFIG_PATH=config.yaml ./target/release/redirector
```

## Configuration

See [config.yaml.example](config.yaml.example) for all options.

Environment variables override config file (prefix `REDIRECTOR__`):

```bash
export REDIRECTOR__SERVER__PORT=3000
export REDIRECTOR__REDIS__URL=redis://localhost:6379
export REDIRECTOR__DATABASE__URL=postgres://user:pass@host/db
```

## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/` | GET | Service info page |
| `/{hashid}` | GET | Redirect with interstitial |
| `/metrics` | GET | Prometheus metrics (Basic Auth) |

## Architecture

```
Request → Rate Limiter → Hashid Decode → Redis Cache
                                              ↓ (miss)
                                         PostgreSQL
                                              ↓
                                         Cache Result
                                              ↓
                                      Interstitial Page
                                              ↓
                                        302 Redirect
```

## Development

```bash
# Run tests
cargo test

# Run with hot reload
cargo watch -x run

# Format code
cargo fmt

# Lint
cargo clippy
```

## License

MIT
