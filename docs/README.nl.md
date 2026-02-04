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

### Omgevingsvariabelen

Er zijn **drie manieren** om de service te configureren, gerangschikt op prioriteit (hoogste eerst):

| Prioriteit | Methode | Toepassing |
|------------|---------|------------|
| 1 | `REDIRECTOR__*` omgevingsvariabelen | Individuele waarden overschrijven |
| 2 | Standaard PaaS-variabelen (`DATABASE_URL`, etc.) | PaaS-platformen (Railway, Heroku, Render) |
| 3 | Configuratiebestand (`config.yaml` of `CONFIG_BASE64`) | Basisconfiguratie |

#### Speciale variabelen

| Variabele | Standaard | Beschrijving |
|-----------|-----------|--------------|
| `CONFIG_PATH` | `config.yaml` | Pad naar YAML-configuratiebestand |
| `CONFIG_BASE64` | — | Base64-gecodeerde YAML-configuratie (heeft prioriteit boven `CONFIG_PATH`) |

#### Standaard PaaS-omgevingsvariabelen

Deze worden automatisch herkend en toegepast. De meeste PaaS-platformen stellen ze automatisch in:

| Variabele | Configuratiepad | Voorbeeld |
|-----------|-----------------|-----------|
| `DATABASE_URL` | `database.url` | `postgres://user:pass@host:5432/db` |
| `REDIS_URL` | `redis.url` | `redis://host:6379` |
| `PORT` | `server.port` | `3000` |
| `HASHIDS_SALTS` | `hashids.salts` | `new-salt,old-salt` (kommagescheiden) |

> **Prioriteitsregel**: Als zowel `DATABASE_URL` als `REDIRECTOR__DATABASE__URL` zijn ingesteld, wint de `REDIRECTOR__`-versie. Hetzelfde geldt: `REDIRECTOR__HASHIDS__SALTS__0` heeft prioriteit boven `HASHIDS_SALTS`.

#### Omgevingsvariabelen met prefix (`REDIRECTOR__*`)

Elke configuratiewaarde kan worden overschreven met het `REDIRECTOR__`-prefix en `__` (dubbele underscore) als nestingsscheidingsteken. Hieronder staat de **volledige referentie** van alle overschrijfbare variabelen:

##### Server

| Omgevingsvariabele | Configuratiepad | Standaard | Beschrijving |
|-------------------|-----------------|-----------|--------------|
| `REDIRECTOR__SERVER__HOST` | `server.host` | `0.0.0.0` | Bindadres |
| `REDIRECTOR__SERVER__PORT` | `server.port` | `8080` | HTTP-poort |

##### Hashids

| Omgevingsvariabele | Configuratiepad | Standaard | Beschrijving |
|-------------------|-----------------|-----------|--------------|
| `REDIRECTOR__HASHIDS__SALTS__0` | `hashids.salts[0]` | *vereist* | Primaire hashid-salt |
| `REDIRECTOR__HASHIDS__SALTS__1` | `hashids.salts[1]` | — | Oude salt (voor migratie) |
| `REDIRECTOR__HASHIDS__MIN_LENGTH` | `hashids.min_length` | `6` | Minimale hashid-lengte |

> **Arrays**: Lijstitems worden geïndexeerd met `__0`, `__1`, `__2`, etc. Voor hashid salt-rotatie, stel `__0` in voor de nieuwe salt en `__1` voor de oude.

##### Redis / Cache

| Omgevingsvariabele | Configuratiepad | Standaard | Beschrijving |
|-------------------|-----------------|-----------|--------------|
| `REDIRECTOR__REDIS__URL` | `redis.url` | *vereist* | Redis-verbindings-URL |
| `REDIRECTOR__REDIS__CACHE_TTL_SECONDS` | `redis.cache_ttl_seconds` | `86400` | Cache-TTL (seconden). `86400` = 24u |

##### Database

| Omgevingsvariabele | Configuratiepad | Standaard | Beschrijving |
|-------------------|-----------------|-----------|--------------|
| `REDIRECTOR__DATABASE__URL` | `database.url` | *vereist* | PostgreSQL-verbindings-URL |
| `REDIRECTOR__DATABASE__POOL__MAX_CONNECTIONS` | `database.pool.max_connections` | `3` | Verbindingspoolgrootte |
| `REDIRECTOR__DATABASE__POOL__CONNECT_TIMEOUT_SECONDS` | `database.pool.connect_timeout_seconds` | `3` | Verbindingstime-out (seconden) |
| `REDIRECTOR__DATABASE__RATE_LIMIT__MAX_REQUESTS_PER_SECOND` | `database.rate_limit.max_requests_per_second` | `50` | Max. databasequery's per seconde |
| `REDIRECTOR__DATABASE__CIRCUIT_BREAKER__FAILURE_THRESHOLD` | `database.circuit_breaker.failure_threshold` | `3` | Opeenvolgende fouten voordat circuit opent |
| `REDIRECTOR__DATABASE__CIRCUIT_BREAKER__RESET_TIMEOUT_SECONDS` | `database.circuit_breaker.reset_timeout_seconds` | `60` | Seconden voor half-open herpoging |
| `REDIRECTOR__DATABASE__QUERY__TABLE` | `database.query.table` | `dictionary.urls` | Tabelnaam voor URL-lookups |
| `REDIRECTOR__DATABASE__QUERY__ID_COLUMN` | `database.query.id_column` | `id` | Kolomnaam voor numeriek ID |
| `REDIRECTOR__DATABASE__QUERY__URL_COLUMN` | `database.query.url_column` | `name` | Kolomnaam voor doel-URL |

##### Tussenpagina

| Omgevingsvariabele | Configuratiepad | Standaard | Beschrijving |
|-------------------|-----------------|-----------|--------------|
| `REDIRECTOR__INTERSTITIAL__DELAY_SECONDS` | `interstitial.delay_seconds` | `5` | Aftelling voor omleiding |

##### Metrics

| Omgevingsvariabele | Configuratiepad | Standaard | Beschrijving |
|-------------------|-----------------|-----------|--------------|
| `REDIRECTOR__METRICS__BASIC_AUTH__USERNAME` | `metrics.basic_auth.username` | *vereist* | Gebruikersnaam voor `/metrics`-endpoint |
| `REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD` | `metrics.basic_auth.password` | *vereist* | Wachtwoord voor `/metrics`-endpoint |

##### Snelheidsbeperking (globaal)

| Omgevingsvariabele | Configuratiepad | Standaard | Beschrijving |
|-------------------|-----------------|-----------|--------------|
| `REDIRECTOR__RATE_LIMIT__REQUESTS_PER_SECOND` | `rate_limit.requests_per_second` | `1000` | Max. verzoeken per seconde |
| `REDIRECTOR__RATE_LIMIT__BURST` | `rate_limit.burst` | `100` | Burstcapaciteit boven RPS-limiet |

##### Admin-dashboard

| Omgevingsvariabele | Configuratiepad | Standaard | Beschrijving |
|-------------------|-----------------|-----------|--------------|
| `REDIRECTOR__ADMIN__ENABLED` | `admin.enabled` | `false` | Admin-dashboard inschakelen |
| `REDIRECTOR__ADMIN__SESSION_SECRET` | `admin.session_secret` | `change-me-...` | Sessie-ondertekeningsgeheim (min. 32 tekens) |
| `REDIRECTOR__ADMIN__SESSION_TTL_HOURS` | `admin.session_ttl_hours` | `24` | Sessielevensduur in uren |

> **Opmerking**: Admin-gebruikers (`admin.users`) met `username` en `password_hash` kunnen niet via omgevingsvariabelen worden ingesteld vanwege hun complexe structuur. Definieer ze in het configuratiebestand of `CONFIG_BASE64`.

#### Voorbeelden per deploymentplatform

**Railway / Render / Fly.io** (PaaS met beheerde databases):

```bash
# Deze worden meestal automatisch ingesteld door het platform:
DATABASE_URL=postgres://user:pass@host:5432/db
REDIS_URL=redis://host:6379
PORT=3000

# Stel je configuratie in via base64:
CONFIG_BASE64=c2VydmVyOgogIGhvc3Q6IC...

# Of overschrijf individuele waarden:
REDIRECTOR__HASHIDS__SALTS__0=my-secret-salt
REDIRECTOR__METRICS__BASIC_AUTH__USERNAME=prometheus
REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD=strong-password
REDIRECTOR__ADMIN__ENABLED=true
REDIRECTOR__ADMIN__SESSION_SECRET=random-32-byte-secret-for-sessions
```

**Docker Compose (volledig voorbeeld met alle overschrijvingen)**:

```yaml
services:
  redirector:
    image: ghcr.io/brilliant-almazov/redirector:latest
    ports:
      - "8080:8080"
    environment:
      # --- Verbindings-URLs (PaaS-stijl) ---
      DATABASE_URL: "postgres://redirector:${DB_PASSWORD}@postgres:5432/redirector"
      REDIS_URL: "redis://redis:6379"

      # --- Configuratiebestand ---
      CONFIG_BASE64: "${CONFIG_BASE64}"

      # --- Server ---
      REDIRECTOR__SERVER__HOST: "0.0.0.0"
      REDIRECTOR__SERVER__PORT: "8080"

      # --- Hashid salts ---
      REDIRECTOR__HASHIDS__SALTS__0: "${HASHID_SALT}"        # primaire salt
      REDIRECTOR__HASHIDS__SALTS__1: "${HASHID_SALT_OLD}"    # oude salt voor migratie
      REDIRECTOR__HASHIDS__MIN_LENGTH: "6"

      # --- Redis cache ---
      REDIRECTOR__REDIS__CACHE_TTL_SECONDS: "43200"          # 12 uur

      # --- Databasepool & veerkracht ---
      REDIRECTOR__DATABASE__POOL__MAX_CONNECTIONS: "5"
      REDIRECTOR__DATABASE__POOL__CONNECT_TIMEOUT_SECONDS: "5"
      REDIRECTOR__DATABASE__RATE_LIMIT__MAX_REQUESTS_PER_SECOND: "100"
      REDIRECTOR__DATABASE__CIRCUIT_BREAKER__FAILURE_THRESHOLD: "5"
      REDIRECTOR__DATABASE__CIRCUIT_BREAKER__RESET_TIMEOUT_SECONDS: "30"

      # --- Aangepaste tabelmapping ---
      REDIRECTOR__DATABASE__QUERY__TABLE: "public.short_urls"
      REDIRECTOR__DATABASE__QUERY__ID_COLUMN: "id"
      REDIRECTOR__DATABASE__QUERY__URL_COLUMN: "target_url"

      # --- Tussenpagina ---
      REDIRECTOR__INTERSTITIAL__DELAY_SECONDS: "3"

      # --- Metrics-authenticatie ---
      REDIRECTOR__METRICS__BASIC_AUTH__USERNAME: "prometheus"
      REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD: "${METRICS_PASSWORD}"

      # --- Globale snelheidsbeperking ---
      REDIRECTOR__RATE_LIMIT__REQUESTS_PER_SECOND: "2000"
      REDIRECTOR__RATE_LIMIT__BURST: "200"

      # --- Admin-dashboard ---
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

**Gewone Docker (enkele opdracht)**:

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

**Minimale setup (alleen omgevingsvariabelen, geen configuratiebestand)**:

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

#### Salt-rotatie via omgevingsvariabelen

Bij het roteren van hashid salts probeert de service salts in volgorde — de eerste match wint. Stel de nieuwe salt als eerste in zodat nieuwe links deze gebruiken, en behoud de oude salt voor achterwaartse compatibiliteit:

**Optie 1: Enkele variabele met kommascheidingsteken** (aanbevolen):

```bash
# Voor rotatie
HASHIDS_SALTS=original-salt

# Na rotatie — nieuwe salt eerst, oude salt voor bestaande links
HASHIDS_SALTS=new-salt,original-salt
```

**Optie 2: Geïndexeerde variabelen**:

```bash
# Voor rotatie
REDIRECTOR__HASHIDS__SALTS__0=original-salt

# Na rotatie
REDIRECTOR__HASHIDS__SALTS__0=new-salt
REDIRECTOR__HASHIDS__SALTS__1=original-salt
```

> **Opmerking**: Als `REDIRECTOR__HASHIDS__SALTS__0` is ingesteld, wordt `HASHIDS_SALTS` genegeerd.

#### Base64-configuratie

Voor omgevingen waar het mounten van configuratiebestanden niet praktisch is (PaaS, serverless, CI/CD), geef de volledige configuratie door als een base64-gecodeerde string:

```bash
# Encode
cat config.yaml | base64

# Decoderen (ter verificatie)
echo "$CONFIG_BASE64" | base64 -d
```

`CONFIG_BASE64` heeft prioriteit boven `CONFIG_PATH`. Overschrijvingen via omgevingsvariabelen (`REDIRECTOR__*` en PaaS-variabelen) worden **bovenop** de gedecodeerde configuratie toegepast.

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
