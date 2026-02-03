# redirector

> **Högpresterande URL-förkortare och omdirigeringstjänst** byggd med Rust, Axum, Redis och PostgreSQL. Med säkra mellanliggande sidor, realtidsadministratorpanel och observerbarhet på företagsnivå.

[English](../README.md) | [Русский](README.ru.md) | [中文](README.zh.md) | [हिंदी](README.hi.md) | [Español](README.es.md) | [Português](README.pt.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Polski](README.pl.md) | [Nederlands](README.nl.md) | [Italiano](README.it.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) | [עברית](README.he.md) | [Bahasa Indonesia](README.id.md) | [Tiếng Việt](README.vi.md) | **Svenska** | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![Docker Image Size](https://ghcr-badge.egpl.dev/brilliant-almazov/redirector/size)](https://github.com/brilliant-almazov/redirector/pkgs/container/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

**Nyckelord**: URL-förkortare, länkförkortare, omdirigeringstjänst, Rust webbtjänst, Axum ramverk, Redis cache, PostgreSQL, Prometheus mätvärden, hashids, korta länkar, mellanliggande sidor, säkra omdirigeringar, hög prestanda, mikrotjänst

Säker URL-omdirigeringstjänst med mellanliggande sidor och hashid-baserade korta länkar. Perfekt för interna verktyg, företagslänkhantering och varumärkesförsedda korta URL-tjänster.

### Prestanda

| Scenario | RPS | Medel latens | P99 latens |
|----------|-----|--------------|------------|
| 100% Cache Hit | **7 800+** | ~14ms | ~50ms |
| Cache Miss (10K URLs) | **2 300+** | ~44ms | ~81ms |

**Testvillkor**: wrk -t4 -c100 -d30s, PostgreSQL 15, Dragonfly (Redis), macOS M1 (Docker)

> ⚠️ Resultat från Docker på macOS med VM-overhead. Native Linux-distribution förväntas vara **3-5x snabbare**.

## Problem

Att dela långa URL:er är opraktiskt. URL-förkortare finns men omdirigerar ofta omedelbart, vilket kan vara en säkerhetsrisk. Användare bör se vart de går innan de omdirigeras.

**redirector** tillhandahåller säkra omdirigeringar med:
- Mellanliggande sida som visar mål-URL före omdirigering
- Nedräkningstimer för användarmedvetenhet
- Vackra, varumärkesförsedda sidor

## Funktioner

- 🔗 **Hashid-URL:er** - Korta, unika, icke-sekventiella ID:n (t.ex. `/r/abc123`)
- ⏱️ **Mellanliggande sida** - Nedräkningstimer visar mål-URL före omdirigering
- ⚡ **Redis-caching** - Snabba uppslag med konfigurerbar TTL
- 🛡️ **Circuit breaker** - Databasskydd mot kaskadfel
- 🚦 **Hastighetsbegränsning** - Både globala och databasnivåbegränsningar
- 📊 **Prometheus-mätvärden** - Full observerbarhet med Basic Auth-skydd
- 🎨 **Vackra sidor** - Rena 404- och indexsidor med 4 teman
- 🔑 **Multipla salts** - Stöd för hashid salt-rotation för migrering
- 📱 **Adminpanel** - Realtidsövervakning av mätvärden med SSE

## Skärmdumpar

| Ljus | Mörk | Grå | Varm |
|------|------|-----|------|
| ![Dashboard Ljus](screenshots/dashboard-light.png) | ![Dashboard Mörk](screenshots/dashboard-dark.png) | ![Dashboard Grå](screenshots/dashboard-gray.png) | ![Dashboard Varm](screenshots/dashboard-warm.png) |
| ![Login Ljus](screenshots/login-light.png) | ![Login Mörk](screenshots/login-dark.png) | ![Login Grå](screenshots/login-gray.png) | ![Login Varm](screenshots/login-warm.png) |
| ![Index Light](screenshots/index-light.png) | ![Index Dark](screenshots/index-dark.png) | ![Index Gray](screenshots/index-gray.png) | ![Index Warm](screenshots/index-warm.png) |
| ![Interstitial Light](screenshots/interstitial-light.png) | ![Interstitial Dark](screenshots/interstitial-dark.png) | ![Interstitial Gray](screenshots/interstitial-gray.png) | ![Interstitial Warm](screenshots/interstitial-warm.png) |
| ![404 Light](screenshots/404-light.png) | ![404 Dark](screenshots/404-dark.png) | ![404 Gray](screenshots/404-gray.png) | ![404 Warm](screenshots/404-warm.png) |

### Lasttest-modal

| Ljus | Mörk | Grå | Varm |
|------|------|-----|------|
| ![Modal Ljus](screenshots/dashboard-modal-light.png) | ![Modal Mörk](screenshots/dashboard-modal-dark.png) | ![Modal Grå](screenshots/dashboard-modal-gray.png) | ![Modal Varm](screenshots/dashboard-modal-warm.png) |

## Teknikstack

- **Språk**: Rust (async med Tokio)
- **Webbramverk**: Axum
- **Cache**: Redis-kompatibel (Redis, Dragonfly, Valkey, KeyDB, etc.)
- **Databas**: PostgreSQL (pluggbart lagringslager)
- **Mätvärden**: Prometheus + metrics-rs
- **Lösenordshashning**: Argon2

> **Notera**: Lagrings- och cache-lagren är abstraherade och kan ersättas med vilken kompatibel datakälla som helst. För närvarande under aktiv utveckling.

## Snabbstart

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

## Hur det fungerar

1. Användaren besöker `/r/{hashid}` (t.ex. `/r/abc123`)
2. Tjänsten avkodar hashid till numeriskt ID
3. Kontrollerar Redis-cache för URL
4. Vid cache miss, frågar PostgreSQL
5. Cachelagrar resultat i Redis
6. Visar mellanliggande sida med nedräkning
7. Efter nedräkning, omdirigerar till mål-URL

## Endpoints

| Endpoint | Auth | Beskrivning |
|----------|------|-------------|
| `GET /` | Nej | Indexsida |
| `GET /r/{hashid}` | Nej | Omdirigering med mellanliggande sida |
| `GET /d/{hashid}` | Nej | Demo-omdirigering (syntetisk lasttestning) |
| `GET /health` | Nej | Hälsokontroll |
| `GET /metrics` | Basic | Prometheus-mätvärden |
| `GET /admin` | Session | Adminpanel-inloggning |
| `GET /admin/dashboard` | Session | Adminpanel |

## Adminpanel

Tjänsten inkluderar en valfri adminpanel för realtidsövervakning av mätvärden.

### Installation

1. **Generera lösenordshash:**

```bash
cargo run --bin hash_password
# Ange lösenord, eller:
cargo run --bin hash_password -- "ditt-lösenord"
```

2. **Lägg till i config.yaml:**

```yaml
admin:
  enabled: true
  session_ttl_hours: 24
  users:
    - username: admin
      password_hash: "$argon2id$v=19$m=19456,t=2,p=1$..."  # från steg 1
```

3. **Öppna panelen:**

Gå till `http://localhost:8080/admin` och logga in med dina uppgifter.

### Funktioner

- Realtids RPS- och latensdiagram
- Systemmätvärden (CPU, minne, drifttid)
- Övervakning av cache hit rate
- Lista över senaste omdirigeringar
- Lastsimulering för testning
- Tre teman: Ljus, Mörk, Varm

## Licens

MIT-licens - se [LICENSE](../LICENSE) för detaljer.

## Bidra

Bidrag välkomnas! Vänligen:

1. Forka repositoryt
2. Skapa en funktionsgren
3. Skicka in en Pull Request

Skyddad master-gren kräver PR-granskning.
