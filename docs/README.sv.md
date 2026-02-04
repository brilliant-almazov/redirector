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

### Miljövariabler

Det finns **tre sätt** att konfigurera tjänsten, listade efter prioritet (högst först):

| Prioritet | Metod | Användningsfall |
|-----------|-------|-----------------|
| 1 | `REDIRECTOR__*` miljövariabler | Åsidosätta enskilda värden |
| 2 | Standard PaaS-variabler (`DATABASE_URL`, etc.) | PaaS-plattformar (Railway, Heroku, Render) |
| 3 | Konfigurationsfil (`config.yaml` eller `CONFIG_BASE64`) | Baskonfiguration |

#### Speciella variabler

| Variabel | Standard | Beskrivning |
|----------|----------|-------------|
| `CONFIG_PATH` | `config.yaml` | Sökväg till YAML-konfigurationsfil |
| `CONFIG_BASE64` | — | Base64-kodad YAML-konfiguration (har prioritet över `CONFIG_PATH`) |

#### Standard PaaS-miljövariabler

Dessa känns automatiskt igen och tillämpas. De flesta PaaS-plattformar ställer in dem automatiskt:

| Variabel | Konfigurationssökväg | Exempel |
|----------|---------------------|---------|
| `DATABASE_URL` | `database.url` | `postgres://user:pass@host:5432/db` |
| `REDIS_URL` | `redis.url` | `redis://host:6379` |
| `PORT` | `server.port` | `3000` |
| `HASHIDS_SALTS` | `hashids.salts` | `new-salt,old-salt` (kommaseparerade) |

> **Prioritetsregel**: Om både `DATABASE_URL` och `REDIRECTOR__DATABASE__URL` är satta, vinner `REDIRECTOR__`-versionen. Likaså har `REDIRECTOR__HASHIDS__SALTS__0` prioritet över `HASHIDS_SALTS`.

#### Miljövariabler med prefix (`REDIRECTOR__*`)

Alla konfigurationsvärden kan åsidosättas med `REDIRECTOR__`-prefixet och `__` (dubbelt understreck) som nästningsseparator. Nedan finns den **fullständiga referensen** av alla åsidosättbara variabler:

##### Server

| Miljövariabel | Konfigurationssökväg | Standard | Beskrivning |
|--------------|---------------------|----------|-------------|
| `REDIRECTOR__SERVER__HOST` | `server.host` | `0.0.0.0` | Bindadress |
| `REDIRECTOR__SERVER__PORT` | `server.port` | `8080` | HTTP-port |

##### Hashids

| Miljövariabel | Konfigurationssökväg | Standard | Beskrivning |
|--------------|---------------------|----------|-------------|
| `REDIRECTOR__HASHIDS__SALTS__0` | `hashids.salts[0]` | *obligatoriskt* | Primärt hashid-salt |
| `REDIRECTOR__HASHIDS__SALTS__1` | `hashids.salts[1]` | — | Gammalt salt (för migrering) |
| `REDIRECTOR__HASHIDS__MIN_LENGTH` | `hashids.min_length` | `6` | Minsta hashid-längd |

> **Arrayer**: Listelement indexeras med `__0`, `__1`, `__2`, etc. För hashid salt-rotation, ställ in `__0` för det nya saltet och `__1` för det gamla.

##### Redis / Cache

| Miljövariabel | Konfigurationssökväg | Standard | Beskrivning |
|--------------|---------------------|----------|-------------|
| `REDIRECTOR__REDIS__URL` | `redis.url` | *obligatoriskt* | Redis-anslutnings-URL |
| `REDIRECTOR__REDIS__CACHE_TTL_SECONDS` | `redis.cache_ttl_seconds` | `86400` | Cache-TTL (sekunder). `86400` = 24h |

##### Databas

| Miljövariabel | Konfigurationssökväg | Standard | Beskrivning |
|--------------|---------------------|----------|-------------|
| `REDIRECTOR__DATABASE__URL` | `database.url` | *obligatoriskt* | PostgreSQL-anslutnings-URL |
| `REDIRECTOR__DATABASE__POOL__MAX_CONNECTIONS` | `database.pool.max_connections` | `3` | Anslutningspoolstorlek |
| `REDIRECTOR__DATABASE__POOL__CONNECT_TIMEOUT_SECONDS` | `database.pool.connect_timeout_seconds` | `3` | Anslutningstimeout (sekunder) |
| `REDIRECTOR__DATABASE__RATE_LIMIT__MAX_REQUESTS_PER_SECOND` | `database.rate_limit.max_requests_per_second` | `50` | Max. databasförfrågningar per sekund |
| `REDIRECTOR__DATABASE__CIRCUIT_BREAKER__FAILURE_THRESHOLD` | `database.circuit_breaker.failure_threshold` | `3` | Konsekutiva fel innan kretsen öppnas |
| `REDIRECTOR__DATABASE__CIRCUIT_BREAKER__RESET_TIMEOUT_SECONDS` | `database.circuit_breaker.reset_timeout_seconds` | `60` | Sekunder innan halvöppet återförsök |
| `REDIRECTOR__DATABASE__QUERY__TABLE` | `database.query.table` | `dictionary.urls` | Tabellnamn för URL-sökningar |
| `REDIRECTOR__DATABASE__QUERY__ID_COLUMN` | `database.query.id_column` | `id` | Kolumnnamn för numeriskt ID |
| `REDIRECTOR__DATABASE__QUERY__URL_COLUMN` | `database.query.url_column` | `name` | Kolumnnamn för mål-URL |

##### Mellanliggande sida

| Miljövariabel | Konfigurationssökväg | Standard | Beskrivning |
|--------------|---------------------|----------|-------------|
| `REDIRECTOR__INTERSTITIAL__DELAY_SECONDS` | `interstitial.delay_seconds` | `5` | Nedräkning innan omdirigering |

##### Mätvärden

| Miljövariabel | Konfigurationssökväg | Standard | Beskrivning |
|--------------|---------------------|----------|-------------|
| `REDIRECTOR__METRICS__BASIC_AUTH__USERNAME` | `metrics.basic_auth.username` | *obligatoriskt* | Användarnamn för `/metrics`-endpoint |
| `REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD` | `metrics.basic_auth.password` | *obligatoriskt* | Lösenord för `/metrics`-endpoint |

##### Hastighetsbegränsning (global)

| Miljövariabel | Konfigurationssökväg | Standard | Beskrivning |
|--------------|---------------------|----------|-------------|
| `REDIRECTOR__RATE_LIMIT__REQUESTS_PER_SECOND` | `rate_limit.requests_per_second` | `1000` | Max. förfrågningar per sekund |
| `REDIRECTOR__RATE_LIMIT__BURST` | `rate_limit.burst` | `100` | Burstkapacitet ovanför RPS-gräns |

##### Adminpanel

| Miljövariabel | Konfigurationssökväg | Standard | Beskrivning |
|--------------|---------------------|----------|-------------|
| `REDIRECTOR__ADMIN__ENABLED` | `admin.enabled` | `false` | Aktivera adminpanel |
| `REDIRECTOR__ADMIN__SESSION_SECRET` | `admin.session_secret` | `change-me-...` | Sessionsigneringshemlighet (min. 32 tecken) |
| `REDIRECTOR__ADMIN__SESSION_TTL_HOURS` | `admin.session_ttl_hours` | `24` | Sessionslivstid i timmar |

> **Notera**: Administratörsanvändare (`admin.users`) med `username` och `password_hash` kan inte ställas in via miljövariabler på grund av deras komplexa struktur. Definiera dem i konfigurationsfilen eller `CONFIG_BASE64`.

#### Exempel efter distributionsplattform

**Railway / Render / Fly.io** (PaaS med hanterade databaser):

```bash
# Dessa ställs vanligtvis in automatiskt av plattformen:
DATABASE_URL=postgres://user:pass@host:5432/db
REDIS_URL=redis://host:6379
PORT=3000

# Ställ in din konfiguration via base64:
CONFIG_BASE64=c2VydmVyOgogIGhvc3Q6IC...

# Eller åsidosätt enskilda värden:
REDIRECTOR__HASHIDS__SALTS__0=my-secret-salt
REDIRECTOR__METRICS__BASIC_AUTH__USERNAME=prometheus
REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD=strong-password
REDIRECTOR__ADMIN__ENABLED=true
REDIRECTOR__ADMIN__SESSION_SECRET=random-32-byte-secret-for-sessions
```

**Docker Compose (fullständigt exempel med alla åsidosättningar)**:

```yaml
services:
  redirector:
    image: ghcr.io/brilliant-almazov/redirector:latest
    ports:
      - "8080:8080"
    environment:
      # --- Anslutnings-URLs (PaaS-stil) ---
      DATABASE_URL: "postgres://redirector:${DB_PASSWORD}@postgres:5432/redirector"
      REDIS_URL: "redis://redis:6379"

      # --- Konfigurationsfil ---
      CONFIG_BASE64: "${CONFIG_BASE64}"

      # --- Server ---
      REDIRECTOR__SERVER__HOST: "0.0.0.0"
      REDIRECTOR__SERVER__PORT: "8080"

      # --- Hashid-salts ---
      REDIRECTOR__HASHIDS__SALTS__0: "${HASHID_SALT}"        # primärt salt
      REDIRECTOR__HASHIDS__SALTS__1: "${HASHID_SALT_OLD}"    # gammalt salt för migrering
      REDIRECTOR__HASHIDS__MIN_LENGTH: "6"

      # --- Redis-cache ---
      REDIRECTOR__REDIS__CACHE_TTL_SECONDS: "43200"          # 12 timmar

      # --- Databaspool och motståndskraft ---
      REDIRECTOR__DATABASE__POOL__MAX_CONNECTIONS: "5"
      REDIRECTOR__DATABASE__POOL__CONNECT_TIMEOUT_SECONDS: "5"
      REDIRECTOR__DATABASE__RATE_LIMIT__MAX_REQUESTS_PER_SECOND: "100"
      REDIRECTOR__DATABASE__CIRCUIT_BREAKER__FAILURE_THRESHOLD: "5"
      REDIRECTOR__DATABASE__CIRCUIT_BREAKER__RESET_TIMEOUT_SECONDS: "30"

      # --- Anpassad tabellmappning ---
      REDIRECTOR__DATABASE__QUERY__TABLE: "public.short_urls"
      REDIRECTOR__DATABASE__QUERY__ID_COLUMN: "id"
      REDIRECTOR__DATABASE__QUERY__URL_COLUMN: "target_url"

      # --- Mellanliggande sida ---
      REDIRECTOR__INTERSTITIAL__DELAY_SECONDS: "3"

      # --- Autentisering av mätvärden ---
      REDIRECTOR__METRICS__BASIC_AUTH__USERNAME: "prometheus"
      REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD: "${METRICS_PASSWORD}"

      # --- Global hastighetsbegränsning ---
      REDIRECTOR__RATE_LIMIT__REQUESTS_PER_SECOND: "2000"
      REDIRECTOR__RATE_LIMIT__BURST: "200"

      # --- Adminpanel ---
      REDIRECTOR__ADMIN__ENABLED: "true"
      REDIRECTOR__ADMIN__SESSION_SECRET: "${SESSION_SECRET}"
      REDIRECTOR__ADMIN__SESSION_TTL_HOURS: "8"
    depends_on:
      - postgres
      - redis

  postgres:
    image: postgres:16-alpine
    environment:
      POSTGRES_USER: redirector
      POSTGRES_PASSWORD: ${DB_PASSWORD}
      POSTGRES_DB: redirector

  redis:
    image: redis:7-alpine
```

**Kubernetes**:

```yaml
apiVersion: apps/v1
kind: Deployment
spec:
  template:
    spec:
      containers:
        - name: redirector
          image: ghcr.io/brilliant-almazov/redirector:latest
          env:
            - name: DATABASE_URL
              valueFrom:
                secretKeyRef:
                  name: redirector-secrets
                  key: database-url
            - name: REDIS_URL
              valueFrom:
                secretKeyRef:
                  name: redirector-secrets
                  key: redis-url
            - name: REDIRECTOR__HASHIDS__SALTS__0
              valueFrom:
                secretKeyRef:
                  name: redirector-secrets
                  key: hashid-salt
            - name: REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD
              valueFrom:
                secretKeyRef:
                  name: redirector-secrets
                  key: metrics-password
            - name: REDIRECTOR__ADMIN__SESSION_SECRET
              valueFrom:
                secretKeyRef:
                  name: redirector-secrets
                  key: session-secret
            - name: CONFIG_BASE64
              valueFrom:
                configMapKeyRef:
                  name: redirector-config
                  key: config-base64
```

**Vanlig Docker (enstaka kommando)**:

```bash
docker run -p 8080:8080 \
  -e DATABASE_URL="postgres://user:pass@host:5432/db" \
  -e REDIS_URL="redis://host:6379" \
  -e REDIRECTOR__HASHIDS__SALTS__0="my-secret-salt" \
  -e REDIRECTOR__METRICS__BASIC_AUTH__USERNAME="prometheus" \
  -e REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD="strong-password" \
  -e REDIRECTOR__INTERSTITIAL__DELAY_SECONDS="3" \
  -e CONFIG_BASE64="$(cat config.yaml | base64)" \
  ghcr.io/brilliant-almazov/redirector:latest
```

**Minimal konfiguration (enbart miljövariabler, ingen konfigurationsfil)**:

```bash
export CONFIG_BASE64=$(cat <<'YAML' | base64
hashids:
  salts:
    - "my-secret-salt"
metrics:
  basic_auth:
    username: prometheus
    password: change-me
YAML
)
export DATABASE_URL=postgres://user:pass@localhost:5432/db
export REDIS_URL=redis://localhost:6379
export PORT=3000

./redirector
```

#### Saltrotation via miljövariabler

Vid rotation av hashid-salts försöker tjänsten salts i ordning -- den första träffen vinner. Ställ in det nya saltet först så att nya länkar använder det, och behåll det gamla saltet för bakåtkompatibilitet:

**Alternativ 1: Enskild variabel med kommaseparator** (rekommenderat):

```bash
# Före rotation
HASHIDS_SALTS=original-salt

# Efter rotation — nytt salt först, gammalt salt för befintliga länkar
HASHIDS_SALTS=new-salt,original-salt
```

**Alternativ 2: Indexerade variabler**:

```bash
# Före rotation
REDIRECTOR__HASHIDS__SALTS__0=original-salt

# Efter rotation
REDIRECTOR__HASHIDS__SALTS__0=new-salt
REDIRECTOR__HASHIDS__SALTS__1=original-salt
```

> **Notera**: Om `REDIRECTOR__HASHIDS__SALTS__0` är satt, ignoreras `HASHIDS_SALTS`.

#### Base64-konfiguration

För miljöer där montering av konfigurationsfiler inte är praktiskt (PaaS, serverless, CI/CD), skicka hela konfigurationen som en base64-kodad sträng:

```bash
# Encode
cat config.yaml | base64

# Avkoda (för att verifiera)
echo "$CONFIG_BASE64" | base64 -d
```

`CONFIG_BASE64` har prioritet över `CONFIG_PATH`. Miljövariabelåsidosättningar (`REDIRECTOR__*` och PaaS-variabler) tillämpas **ovanpå** den avkodade konfigurationen.

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
