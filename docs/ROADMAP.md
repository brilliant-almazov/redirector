# Roadmap

## v0.1.0 (Released)

### Core Features
- [x] Hashid-based URL shortening (`/r/{hashid}`)
- [x] Interstitial page with countdown timer (`/d/{hashid}`)
- [x] Redis caching with configurable TTL
- [x] PostgreSQL storage with circuit breaker
- [x] Prometheus metrics with Basic Auth
- [x] Rate limiting (global and per-database)
- [x] Multiple salt support for hashid migration
- [x] Beautiful 404 and index pages
- [x] Health check endpoints (`/health`, `/healthz`)

### Admin Dashboard
- [x] Web interface for monitoring
- [x] Real-time analytics with SSE
- [x] RPS and latency charts (Chart.js)
- [x] System metrics (CPU, memory, uptime)
- [x] Cache hit rate monitoring
- [x] Load simulation for testing
- [x] Session-based authentication (Argon2)
- [x] Four color themes (Light, Dark, Gray, Warm)

### Event Analytics Pipeline
- [x] RabbitMQ event publishing (optional, configurable)
- [x] Non-blocking fire-and-forget event sending (bounded channel)
- [x] Batching by size and time interval
- [x] Type-safe `EventBatch` enum with `event_type` tag
- [x] Separate `event_consumer` binary (own Docker container)
- [x] PostgreSQL analytics with monthly table partitioning
- [x] Auto-migration system with `schema_migrations` table
- [x] Reference tables for User-Agent and Referer (MD5 dedup)
- [x] User-Agent parsing with woothee
- [x] GeoIP enrichment with MaxMind mmdb
- [x] PostgreSQL ENUM for data source, INET for IP
- [x] Graceful degradation (redirects work without RabbitMQ)

### Configuration
- [x] Environment variable only configuration
- [x] PaaS support (Railway, Heroku, Render)
- [x] `CONFIG_BASE64` for secrets
- [x] Docker and docker-compose support
- [x] CI/CD with coverage and performance badges
- [x] 18 language translations

## v0.2.0 (Planned)

### Admin API
- [ ] `POST /api/urls` - create new redirect
- [ ] `GET /api/urls/{id}` - get redirect info
- [ ] `PUT /api/urls/{id}` - update redirect
- [ ] `DELETE /api/urls/{id}` - delete redirect
- [ ] `GET /api/urls` - list redirects with pagination
- [ ] API authentication (API keys or JWT)

### Observability
- [ ] Structured JSON logging
- [ ] Request tracing (OpenTelemetry)

### Event Pipeline Extensions
- [ ] `EventTransport` trait for pluggable message brokers (RabbitMQ, Kafka, Redis Streams)
- [ ] `AnalyticsStorage` trait for pluggable storage backends (PostgreSQL, ClickHouse, MySQL, SQLite)
- [ ] Configuration-driven transport/storage selection
- [ ] Kafka transport implementation
- [ ] ClickHouse storage implementation

### Link Features
- [ ] Custom short codes (user-defined hashid)
- [ ] TTL on redirects (expiring links)
- [ ] Password-protected links
- [ ] Link metadata (title, description)

## v0.3.0 (Planned)

### QR Codes
- [ ] `GET /r/{hashid}/qr` - generate QR code
- [ ] Configurable QR size and format
- [ ] QR code with logo

### Bulk Operations
- [ ] Bulk import/export
- [ ] CSV/JSON upload

## Future

### Infrastructure
- [ ] Helm chart for Kubernetes
- [ ] OpenAPI/Swagger specification
- [ ] Terraform modules
- [ ] Multi-region support

### gRPC API
- [ ] Proto definitions for Admin API
- [ ] gRPC server (tonic)
- [ ] CreateUrl, GetUrl, UpdateUrl, DeleteUrl, ListUrls
- [ ] Streaming metrics endpoint
- [ ] CLI client using gRPC

### Integrations
- [ ] Webhook notifications
- [ ] Slack/Discord bot
- [ ] Browser extension
- [ ] CLI tool (gRPC-based)

---

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for how to contribute to this project.

Ideas and feedback welcome in [GitHub Issues](https://github.com/brilliant-almazov/redirector/issues).
