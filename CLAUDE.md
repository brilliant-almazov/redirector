# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Important Rules

- **NEVER add Co-Authored-By or any attribution to Claude/AI in commits**
- **NEVER mention yourself as contributor or author anywhere**
- All work is attributed solely to the repository owner

## Build & Test Commands

```bash
# Build
cargo build --release

# Run tests (unit + integration)
cargo test

# Run single test
cargo test test_name

# Run tests with coverage
cargo llvm-cov --text

# Linting
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings

# Run locally (requires config.yaml, Redis, PostgreSQL)
cargo run
```

## Utility Tools

```bash
# Generate hashids for testing
cargo run --bin gen_hashids -- --salt "your-salt" --count 100

# Hash admin password
cargo run --bin hash_password -- "your-password"

# Generate simulation data
cargo run --bin gen_simulation_data
```

## Architecture

### Core Flow
```
Request → Rate Limiter → Redirect Handler → UrlResolver → Cache (Redis) → DB (PostgreSQL)
                                                    ↓
                                              Interstitial Page → Redirect
```

### Module Structure

- **`services/`** - Core business logic with trait-based abstractions:
  - `traits.rs` - Defines `Cache`, `HashidDecoder`, `UrlStorage` traits
  - `url_resolver.rs` - Orchestrates hashid decoding, cache lookup, DB fallback
  - `cache.rs` - Redis cache implementation
  - `hashid.rs` - Hashid encoding/decoding (supports multiple salts for migration)

- **`handlers/`** - HTTP handlers:
  - `redirect.rs` - Main redirect logic (`/r/{hashid}`, `/d/{hashid}`)
  - `index.rs` - Landing page
  - `metrics.rs` - Prometheus metrics endpoint

- **`middleware/`** - Axum middleware:
  - `rate_limit.rs` - Token bucket rate limiting
  - `basic_auth.rs` - Basic auth for metrics endpoint

- **`db/`** - Database access:
  - `main_storage.rs` - PostgreSQL with circuit breaker and rate limiting

- **`admin/`** - Admin dashboard (Leptos SSR):
  - `pages/` - Login, dashboard views
  - `sse.rs` - Server-sent events for live metrics
  - `auth.rs` - Session-based authentication

- **`config.rs`** - YAML configuration with env var substitution

### Key Design Patterns

- **Trait-based services**: All services implement traits (`Cache`, `UrlStorage`, `HashidDecoder`) for testability
- **Circuit breaker**: Database access protected by failsafe circuit breaker
- **Dual rate limiting**: Global rate limit (middleware) + per-database rate limit
- **Multi-salt support**: Hashid decoder tries multiple salts for migration scenarios
- **Askama templates**: HTML templates in `templates/` directory

### Test Organization

- Unit tests are in `*_test.rs` files next to source files
- Integration tests in `tests/` require testcontainers (Docker)
- Use `rstest` for parameterized tests
