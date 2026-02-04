# redirector

> **Hoogwaardige URL-verkortings- en omleidingsservice** gebouwd met Rust, Axum, Redis en PostgreSQL. Met veilige tussenpagina's, realtime admin-dashboard en enterprise-grade observeerbaarheid.

[English](../README.md) | [Русский](README.ru.md) | [中文](README.zh.md) | [हिंदी](README.hi.md) | [Español](README.es.md) | [Português](README.pt.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Polski](README.pl.md) | **Nederlands** | [Italiano](README.it.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) | [עברית](README.he.md) | [Bahasa Indonesia](README.id.md) | [Tiếng Việt](README.vi.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![Docker Image Size](https://ghcr-badge.egpl.dev/brilliant-almazov/redirector/size)](https://github.com/brilliant-almazov/redirector/pkgs/container/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

**Trefwoorden**: URL-verkorter, linkverkorter, omleidingsservice, Rust webservice, Axum framework, Redis cache, PostgreSQL, Prometheus metrics, hashids, korte links, tussenpagina's, veilige omleidingen, hoge prestaties, microservice

Veilige URL-omleidingsservice met tussenpagina's en hashid-gebaseerde korte links. Perfect voor interne tools, bedrijfslinkbeheer en merkgebonden korte URL-services.

### Prestaties

| Scenario | RPS | Gem. Latentie | P99 Latentie |
|----------|-----|---------------|--------------|
| 100% Cache Hit | **7.800+** | ~14ms | ~50ms |
| Cache Miss (10K URLs) | **2.300+** | ~44ms | ~81ms |

**Testcondities**: wrk -t4 -c100 -d30s, PostgreSQL 15, Dragonfly (Redis), macOS M1 (Docker)

> ⚠️ Resultaten zijn van Docker op macOS met VM-overhead. Native Linux-deployment verwacht **3-5x sneller** te zijn.

## Probleem

Het delen van lange URL's is onhandig. URL-verkorters bestaan, maar leiden vaak direct door, wat een veiligheidsrisico kan zijn. Gebruikers moeten zien waar ze naartoe gaan voordat ze worden doorgestuurd.

**redirector** biedt veilige omleidingen met:
- Tussenpagina die doel-URL toont voor omleiding
- Afteltimer voor gebruikersbewustzijn
- Mooie, merkgebonden pagina's

## Functies

- 🔗 **Hashid URLs** - Korte, unieke, niet-sequentiële IDs (bijv. `/r/abc123`)
- ⏱️ **Tussenpagina** - Afteltimer toont doel-URL voor omleiding
- ⚡ **Redis caching** - Snelle lookups met configureerbare TTL
- 🛡️ **Circuit breaker** - Databasebescherming tegen cascade-fouten
- 🚦 **Rate limiting** - Globale en database-niveau rate limits
- 📊 **Prometheus metrics** - Volledige observeerbaarheid met Basic Auth-bescherming
- 🎨 **Mooie pagina's** - Schone 404- en indexpagina's met 4 thema's
- 🔑 **Meerdere salts** - Hashid salt-rotatie ondersteuning voor migratie
- 📱 **Admin dashboard** - Realtime metrics monitoring met SSE

## Screenshots

| Licht | Donker | Grijs | Warm |
|-------|--------|-------|------|
| ![Dashboard Licht](screenshots/dashboard-light.png) | ![Dashboard Donker](screenshots/dashboard-dark.png) | ![Dashboard Grijs](screenshots/dashboard-gray.png) | ![Dashboard Warm](screenshots/dashboard-warm.png) |
| ![Login Licht](screenshots/login-light.png) | ![Login Donker](screenshots/login-dark.png) | ![Login Grijs](screenshots/login-gray.png) | ![Login Warm](screenshots/login-warm.png) |
| ![Index Light](screenshots/index-light.png) | ![Index Dark](screenshots/index-dark.png) | ![Index Gray](screenshots/index-gray.png) | ![Index Warm](screenshots/index-warm.png) |
| ![Interstitial Light](screenshots/interstitial-light.png) | ![Interstitial Dark](screenshots/interstitial-dark.png) | ![Interstitial Gray](screenshots/interstitial-gray.png) | ![Interstitial Warm](screenshots/interstitial-warm.png) |
| ![404 Light](screenshots/404-light.png) | ![404 Dark](screenshots/404-dark.png) | ![404 Gray](screenshots/404-gray.png) | ![404 Warm](screenshots/404-warm.png) |

### Belastingtest-modal

| Licht | Donker | Grijs | Warm |
|-------|--------|-------|------|
| ![Modal Licht](screenshots/dashboard-modal-light.png) | ![Modal Donker](screenshots/dashboard-modal-dark.png) | ![Modal Grijs](screenshots/dashboard-modal-gray.png) | ![Modal Warm](screenshots/dashboard-modal-warm.png) |

## Technologie stack

- **Taal**: Rust (async met Tokio)
- **Web framework**: Axum
- **Cache**: Redis-compatibel (Redis, Dragonfly, Valkey, KeyDB, etc.)
- **Database**: PostgreSQL (verwisselbare opslaglaag)
- **Metrics**: Prometheus + metrics-rs
- **Wachtwoord hashing**: Argon2

> **Opmerking**: De opslag- en cachelagen zijn geabstraheerd en kunnen worden vervangen door elke compatibele databron. Momenteel in actieve ontwikkeling.

## Snel starten

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

#### Base64-configuratie

Voor omgevingen waar het mounten van configuratiebestanden niet mogelijk is (bijv. Railway, serverless):

```bash
# Encode
cat config.yaml | base64

# Run with base64 config
CONFIG_BASE64="c2VydmVyOgogIGhvc3Q6IC..." docker run ghcr.io/brilliant-almazov/redirector:latest
```

## Hoe het werkt

1. Gebruiker bezoekt `/r/{hashid}` (bijv. `/r/abc123`)
2. Service decodeert hashid naar numeriek ID
3. Controleert Redis cache op URL
4. Bij cache miss, query naar PostgreSQL
5. Cached resultaat in Redis
6. Toont tussenpagina met aftelling
7. Na aftelling, omleiding naar doel-URL

## Endpoints

| Endpoint | Auth | Beschrijving |
|----------|------|--------------|
| `GET /` | Nee | Indexpagina |
| `GET /r/{hashid}` | Nee | Omleiding met tussenpagina |
| `GET /d/{hashid}` | Nee | Demo-omleiding (synthetische belastingtest) |
| `GET /health` | Nee | Health check |
| `GET /metrics` | Basic | Prometheus metrics |
| `GET /admin` | Session | Admin dashboard login |
| `GET /admin/dashboard` | Session | Admin dashboard |

## Admin dashboard

De service bevat een optioneel admin dashboard voor realtime metrics monitoring.

### Configuratie

1. **Genereer wachtwoord hash:**

```bash
cargo run --bin hash_password
# Voer wachtwoord in, of:
cargo run --bin hash_password -- "your-password"
```

2. **Voeg toe aan config.yaml:**

```yaml
admin:
  enabled: true
  session_ttl_hours: 24
  users:
    - username: admin
      password_hash: "$argon2id$v=19$m=19456,t=2,p=1$..."  # van stap 1
```

3. **Toegang tot dashboard:**

Open `http://localhost:8080/admin` en log in met uw credentials.

### Functies

- Realtime RPS en latentie grafieken
- Systeemmetrics (CPU, geheugen, uptime)
- Cache hit rate monitoring
- Lijst van recente omleidingen
- Belastingsimulatie voor tests
- Drie thema's: Licht, Donker, Warm

## Licentie

MIT Licentie - zie [LICENSE](../LICENSE) voor details.

## Bijdragen

Bijdragen zijn welkom! Gelieve:

1. Fork de repository
2. Maak een feature branch
3. Dien een Pull Request in

Beveiligde master branch vereist PR-review.
