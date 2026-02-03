# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Admin Dashboard** - Real-time monitoring with SSE-based live updates
  - RPS and latency charts using Chart.js
  - System metrics (CPU, memory, uptime)
  - Cache hit rate monitoring
  - Recent redirects list
  - Load simulation for testing (configurable RPS)
- **Three color themes** - Light, Dark, and Warm themes with CSS variables
- **Beautiful 404 page** - Themed "Link Not Found" page
- **Password hashing utility** - `cargo run --bin hash_password`
- **Session-based authentication** - Argon2 password hashing with configurable TTL
- **Screenshots** - Documentation screenshots for all themes
- **SEO improvements** - Keywords and meta tags in README

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

[Unreleased]: https://github.com/brilliant-almazov/redirector/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/brilliant-almazov/redirector/releases/tag/v0.1.0
