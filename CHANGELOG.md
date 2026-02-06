# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2026-02-06

### Added
- **Environment Variable Only Configuration** - Full configuration via env vars without requiring config files
  - New `Config::load_from_env()` method for pure environment variable configuration
  - Required vars: `DATABASE_URL`, `REDIS_URL`, `HASHIDS_SALTS`, `METRICS_USERNAME`, `METRICS_PASSWORD`
  - JSON array support for `ADMIN_USERS`: `[{"username":"x","password_hash":"y"}]`
  - Clear error messages listing all missing required variables
  - Loading priority: `CONFIG_BASE64` > `CONFIG_PATH`/`config.yaml` > env vars only
  - PaaS-friendly: works with Railway, Heroku, Render without config files

### Changed
- Configuration loading now auto-detects available sources and falls back gracefully

## [0.3.0] - 2026-02-05

### Added
- **Event Analytics Pipeline** - Full analytics system for redirect events
  - RabbitMQ event publishing with fire-and-forget pattern (non-blocking)
  - Batching by size (100 events) and time interval (1 second)
  - Type-safe `EventBatch` enum with internally-tagged serde
  - Snowflake ID generation for batch identifiers (custom epoch 2025-01-01)
  - Separate `event_consumer` binary for running as own container
  - PostgreSQL analytics storage with monthly table partitioning
  - Auto-partition creation for `redirect_events` table
  - Reference tables with MD5 deduplication: `referers`, `user_agents`, `referer_domains`
  - User-Agent parsing with woothee (browser, version, OS, device type)
  - GeoIP enrichment with MaxMind mmdb support and hot-reload
  - Domain normalization (strips www., lowercases, removes trailing dots)
  - PostgreSQL ENUM for data source (`cache`, `database`)
  - PostgreSQL INET type for IP addresses
  - `RABBITMQ_URL` PaaS environment variable support
  - Graceful degradation: redirects work without RabbitMQ connection

- **New binaries**
  - `event_consumer` - Standalone consumer binary for analytics pipeline
  - `load_test` - HTTP load testing tool

- **Trait-based abstractions**
  - `EventQueue` trait for pluggable message brokers
  - `EventStorage` trait for pluggable analytics backends
  - `EntityResolver` trait for reference table resolution

### Changed
- Version bump to 0.3.0
- Added `chrono`, `md5`, `lapin`, `snowflaked`, `woothee`, `maxminddb` dependencies
- Updated configuration to include `events` section
- Enhanced metrics: `events_published`, `events_dropped`, `events_serialize_errors`, `geoip_reloads_total`

### Database Migrations
- `create_enums.sql` - PostgreSQL ENUM type for data source
- `create_user_agents.sql` - User-Agent reference table with parsed fields
- `create_referers.sql` - Referer reference table with MD5 hash index
- `create_referer_domains.sql` - Normalized domain reference table
- `create_geo_locations.sql` - GeoIP location reference table
- `create_redirect_events.sql` - Main analytics table with monthly partitioning
- `alter_ip_to_inet.sql` - Change IP column from TEXT to INET
- `create_urls.sql` - URL reference table with hash partitioning (10 buckets)

## [0.2.4] - 2026-01-20

### Added
- **Admin Dashboard** - Real-time monitoring with SSE-based live updates
  - RPS and latency charts using Chart.js
  - System metrics (CPU, memory, uptime)
  - Cache hit rate monitoring
  - Recent redirects list
  - Load simulation for testing (configurable RPS)
- **Four color themes** - Light, Dark, Gray, and Warm themes with CSS variables
- **Beautiful 404 page** - Themed "Link Not Found" page
- **Password hashing utility** - `cargo run --bin hash_password`
- **Session-based authentication** - Argon2 password hashing with configurable TTL
- **Screenshots** - Documentation screenshots for all themes
- **SEO improvements** - Keywords and meta tags in README
- **Health check endpoints** - `/healthz` for liveness, `/health` for readiness
- **X-Version header** - Version middleware for all responses
- **PaaS environment variables** - `DATABASE_URL`, `REDIS_URL`, `PORT`, `HASHIDS_SALTS`

### Changed
- Templates moved to `templates/` directory for hot-reload in development
- HTML minification enabled in release builds
- Updated all 18 README translations with Admin Dashboard section

### Security
- All admin routes protected by session authentication
- SSE endpoint `/admin/events` requires authentication
- Secure cookie handling with configurable session TTL

## [0.1.0] - 2025-01-15

### Added
- Initial release
- Hashid-based URL shortening
- Redis caching with configurable TTL
- PostgreSQL storage backend
- Circuit breaker for database protection
- Rate limiting (global and database-level)
- Prometheus metrics with Basic Auth
- Interstitial pages with countdown
- Multiple hashid salt support for migration
- Health check endpoint
- Docker and Docker Compose support

[0.3.0]: https://github.com/brilliant-almazov/redirector/compare/v0.2.4...v0.3.0
[0.2.4]: https://github.com/brilliant-almazov/redirector/compare/v0.1.0...v0.2.4
[0.1.0]: https://github.com/brilliant-almazov/redirector/releases/tag/v0.1.0
