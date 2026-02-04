# redirector

> **{SEO_DESCRIPTION}** {BUILT_WITH_TEXT} Rust, Axum, Redis, {AND} PostgreSQL. {FEATURES_TEXT}

[English](../README.md) | [Русский](README.ru.md) | [中文](README.zh.md) | [हिंदी](README.hi.md) | [Español](README.es.md) | [Português](README.pt.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Polski](README.pl.md) | [Nederlands](README.nl.md) | [Italiano](README.it.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) | [עברית](README.he.md) | [Bahasa Indonesia](README.id.md) | [Tiếng Việt](README.vi.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![Docker Image Size](https://ghcr-badge.egpl.dev/brilliant-almazov/redirector/size)](https://github.com/brilliant-almazov/redirector/pkgs/container/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

**{KEYWORDS_LABEL}**: {KEYWORDS_LIST}

{SHORT_DESCRIPTION}

### {PERFORMANCE_TITLE}

| {SCENARIO} | RPS | {AVG_LATENCY} | {P99_LATENCY} |
|----------|-----|-------------|-------------|
| 100% Cache Hit | **7,800+** | ~14ms | ~50ms |
| Cache Miss (10K URLs) | **2,300+** | ~44ms | ~81ms |

**{TEST_CONDITIONS}**: wrk -t4 -c100 -d30s, PostgreSQL 15, Dragonfly (Redis), macOS M1 (Docker)

> ⚠️ {PERFORMANCE_NOTE}

## {PROBLEM_TITLE}

{PROBLEM_DESCRIPTION}

**redirector** {PROVIDES_SAFE_REDIRECTS}:
- {INTERSTITIAL_FEATURE}
- {COUNTDOWN_FEATURE}
- {BEAUTIFUL_PAGES_FEATURE}

## {FEATURES_TITLE}

- 🔗 **{HASHID_URLS_TITLE}** - {HASHID_URLS_DESC}
- ⏱️ **{INTERSTITIAL_TITLE}** - {INTERSTITIAL_DESC}
- ⚡ **{REDIS_CACHING_TITLE}** - {REDIS_CACHING_DESC}
- 🛡️ **{CIRCUIT_BREAKER_TITLE}** - {CIRCUIT_BREAKER_DESC}
- 🚦 **{RATE_LIMITING_TITLE}** - {RATE_LIMITING_DESC}
- 📊 **{PROMETHEUS_TITLE}** - {PROMETHEUS_DESC}
- 🎨 **{BEAUTIFUL_PAGES_TITLE}** - {BEAUTIFUL_PAGES_DESC}
- 🔑 **{MULTIPLE_SALTS_TITLE}** - {MULTIPLE_SALTS_DESC}
- 📱 **{ADMIN_DASHBOARD_TITLE}** - {ADMIN_DASHBOARD_DESC}

## {SCREENSHOTS_TITLE}

| {LIGHT} | {DARK} | {GRAY} | {WARM} |
|---------|--------|--------|--------|
| ![Dashboard {LIGHT}](screenshots/dashboard-light.png) | ![Dashboard {DARK}](screenshots/dashboard-dark.png) | ![Dashboard {GRAY}](screenshots/dashboard-gray.png) | ![Dashboard {WARM}](screenshots/dashboard-warm.png) |
| ![{LOGIN} {LIGHT}](screenshots/login-light.png) | ![{LOGIN} {DARK}](screenshots/login-dark.png) | ![{LOGIN} {GRAY}](screenshots/login-gray.png) | ![{LOGIN} {WARM}](screenshots/login-warm.png) |
| ![{INDEX} {LIGHT}](screenshots/index-light.png) | ![{INDEX} {DARK}](screenshots/index-dark.png) | ![{INDEX} {GRAY}](screenshots/index-gray.png) | ![{INDEX} {WARM}](screenshots/index-warm.png) |
| ![{INTERSTITIAL} {LIGHT}](screenshots/interstitial-light.png) | ![{INTERSTITIAL} {DARK}](screenshots/interstitial-dark.png) | ![{INTERSTITIAL} {GRAY}](screenshots/interstitial-gray.png) | ![{INTERSTITIAL} {WARM}](screenshots/interstitial-warm.png) |
| ![404 {LIGHT}](screenshots/404-light.png) | ![404 {DARK}](screenshots/404-dark.png) | ![404 {GRAY}](screenshots/404-gray.png) | ![404 {WARM}](screenshots/404-warm.png) |

### {LOAD_TEST_MODAL_TITLE}

| {LIGHT} | {DARK} | {GRAY} | {WARM} |
|---------|--------|--------|--------|
| ![{MODAL} {LIGHT}](screenshots/dashboard-modal-light.png) | ![{MODAL} {DARK}](screenshots/dashboard-modal-dark.png) | ![{MODAL} {GRAY}](screenshots/dashboard-modal-gray.png) | ![{MODAL} {WARM}](screenshots/dashboard-modal-warm.png) |

## {TECH_STACK_TITLE}

- **{LANGUAGE}**: Rust (async {WITH} Tokio)
- **{WEB_FRAMEWORK}**: Axum
- **{CACHE}**: Redis-compatible (Redis, Dragonfly, Valkey, KeyDB, {ETC})
- **{DATABASE}**: PostgreSQL ({PLUGGABLE_STORAGE})
- **{METRICS}**: Prometheus + metrics-rs
- **{PASSWORD_HASHING}**: Argon2

> **{NOTE}**: {STORAGE_NOTE}

## {QUICK_START_TITLE}

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

#### Base64 Configuration

For environments where mounting config files is not possible (e.g. Railway, serverless):

```bash
# Encode
cat config.yaml | base64

# Run with base64 config
CONFIG_BASE64="c2VydmVyOgogIGhvc3Q6IC..." docker run ghcr.io/brilliant-almazov/redirector:latest
```

## {HOW_IT_WORKS_TITLE}

1. {STEP_1}
2. {STEP_2}
3. {STEP_3}
4. {STEP_4}
5. {STEP_5}
6. {STEP_6}
7. {STEP_7}

## {ENDPOINTS_TITLE}

| Endpoint | Auth | {DESCRIPTION} |
|----------|------|-------------|
| `GET /` | {NO} | {INDEX_PAGE_DESC} |
| `GET /r/{hashid}` | {NO} | {REDIRECT_DESC} |
| `GET /d/{hashid}` | {NO} | {DEMO_REDIRECT_DESC} |
| `GET /health` | {NO} | {HEALTH_CHECK_DESC} |
| `GET /metrics` | Basic | {PROMETHEUS_METRICS_DESC} |
| `GET /admin` | Session | {ADMIN_LOGIN_DESC} |
| `GET /admin/dashboard` | Session | {ADMIN_DASHBOARD_DESC} |

## {ADMIN_DASHBOARD_SECTION_TITLE}

{ADMIN_DASHBOARD_INTRO}

### {SETUP_TITLE}

1. **{GENERATE_PASSWORD_HASH}:**

```bash
cargo run --bin hash_password
# {ENTER_PASSWORD_OR}
cargo run --bin hash_password -- "your-password"
```

2. **{ADD_TO_CONFIG}:**

```yaml
admin:
  enabled: true
  session_ttl_hours: 24
  users:
    - username: admin
      password_hash: "$argon2id$v=19$m=19456,t=2,p=1$..."  # {FROM_STEP_1}
```

3. **{ACCESS_DASHBOARD}:**

{OPEN_DASHBOARD_TEXT}

### {FEATURES_TITLE}

- {REALTIME_CHARTS}
- {SYSTEM_METRICS}
- {CACHE_HIT_RATE}
- {RECENT_REDIRECTS}
- {LOAD_SIMULATION}
- {THREE_THEMES}

## {LICENSE_TITLE}

{MIT_LICENSE} - {SEE_LICENSE}

## {CONTRIBUTING_TITLE}

{CONTRIBUTIONS_WELCOME}

1. {FORK_REPO}
2. {CREATE_BRANCH}
3. {SUBMIT_PR}

{PROTECTED_BRANCH_NOTE}
