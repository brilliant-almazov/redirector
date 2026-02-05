# redirector

> **Hochleistungs-URL-Verkürzer und Weiterleitungsdienst** gebaut mit Rust, Axum, Redis und PostgreSQL. Mit sicheren Zwischenseiten, Echtzeit-Admin-Dashboard und Überwachung auf Enterprise-Niveau.

[English](../README.md) | [Русский](README.ru.md) | [中文](README.zh.md) | [हिंदी](README.hi.md) | [Español](README.es.md) | [Português](README.pt.md) | [Français](README.fr.md) | **Deutsch** | [日本語](README.ja.md) | [한국어](README.ko.md) | [Polski](README.pl.md) | [Nederlands](README.nl.md) | [Italiano](README.it.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) | [עברית](README.he.md) | [Bahasa Indonesia](README.id.md) | [Tiếng Việt](README.vi.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![Docker Image Size](https://ghcr-badge.egpl.dev/brilliant-almazov/redirector/size)](https://github.com/brilliant-almazov/redirector/pkgs/container/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

**Schlüsselwörter**: URL-Verkürzer, Link-Verkürzer, Weiterleitungsdienst, Rust-Webdienst, Axum-Framework, Redis-Cache, PostgreSQL, Prometheus-Metriken, Hashids, Kurzlinks, Zwischenseiten, sichere Weiterleitungen, Hochleistung, Microservice

Sicherer URL-Weiterleitungsdienst mit Zwischenseiten und hashid-basierten Kurzlinks. Perfekt für interne Tools, Unternehmens-Link-Management und gebrandete Kurzlink-Dienste.

### Leistung

| Szenario | RPS | Avg Latenz | P99 Latenz |
|----------|-----|------------|------------|
| 100% Cache Hit | **7.800+** | ~14ms | ~50ms |
| Cache Miss (10K URLs) | **2.300+** | ~44ms | ~81ms |

**Testbedingungen**: wrk -t4 -c100 -d30s, PostgreSQL 15, Dragonfly (Redis), macOS M1 (Docker)

> ⚠️ Ergebnisse stammen aus Docker auf macOS mit VM-Overhead. Native Linux-Deployment erwartet **3-5x schneller**.

## Problem

Das Teilen langer URLs ist unpraktisch. URL-Verkürzer existieren, leiten aber oft sofort weiter, was ein Sicherheitsrisiko darstellen kann. Benutzer sollten sehen, wohin sie weitergeleitet werden, bevor die Weiterleitung erfolgt.

**redirector** bietet sichere Weiterleitungen mit:
- Zwischenseite zeigt Ziel-URL vor der Weiterleitung
- Countdown-Timer für Benutzeraufmerksamkeit
- Schöne, gebrandete Seiten

## Funktionen

- 🔗 **Hashid URLs** - Kurze, einzigartige, nicht-sequentielle IDs (z.B. `/r/abc123`)
- ⏱️ **Zwischenseite** - Countdown-Timer zeigt Ziel-URL vor der Weiterleitung
- ⚡ **Redis-Caching** - Schnelle Abfragen mit konfigurierbarem TTL
- 🛡️ **Circuit Breaker** - Datenbankschutz gegen kaskadierende Fehler
- 🚦 **Rate Limiting** - Globale und datenbankbezogene Ratenlimits
- 📊 **Prometheus-Metriken** - Vollständige Observability mit Basic Auth-Schutz
- 🎨 **Schöne Seiten** - Saubere 404- und Index-Seiten mit 4 Themes
- 🔑 **Mehrere Salts** - Hashid-Salt-Rotation für Migration
- 📱 **Admin-Dashboard** - Echtzeit-Metriken-Überwachung mit SSE
- 📤 **Event-Analytik** - Optionale RabbitMQ-Ereignisveröffentlichung mit PostgreSQL-Consumer

## Screenshots

| Hell | Dunkel | Grau | Warm |
|------|--------|------|------|
| ![Dashboard Hell](screenshots/dashboard-light.png) | ![Dashboard Dunkel](screenshots/dashboard-dark.png) | ![Dashboard Grau](screenshots/dashboard-gray.png) | ![Dashboard Warm](screenshots/dashboard-warm.png) |
| ![Login Hell](screenshots/login-light.png) | ![Login Dunkel](screenshots/login-dark.png) | ![Login Grau](screenshots/login-gray.png) | ![Login Warm](screenshots/login-warm.png) |
| ![Index Light](screenshots/index-light.png) | ![Index Dark](screenshots/index-dark.png) | ![Index Gray](screenshots/index-gray.png) | ![Index Warm](screenshots/index-warm.png) |
| ![Interstitial Light](screenshots/interstitial-light.png) | ![Interstitial Dark](screenshots/interstitial-dark.png) | ![Interstitial Gray](screenshots/interstitial-gray.png) | ![Interstitial Warm](screenshots/interstitial-warm.png) |
| ![404 Light](screenshots/404-light.png) | ![404 Dark](screenshots/404-dark.png) | ![404 Gray](screenshots/404-gray.png) | ![404 Warm](screenshots/404-warm.png) |

### Lasttest-Modal

| Hell | Dunkel | Grau | Warm |
|------|--------|------|------|
| ![Modal Hell](screenshots/dashboard-modal-light.png) | ![Modal Dunkel](screenshots/dashboard-modal-dark.png) | ![Modal Grau](screenshots/dashboard-modal-gray.png) | ![Modal Warm](screenshots/dashboard-modal-warm.png) |

## Technologie-Stack

- **Sprache**: Rust (async mit Tokio)
- **Web-Framework**: Axum
- **Cache**: Redis-kompatibel (Redis, Dragonfly, Valkey, KeyDB usw.)
- **Datenbank**: PostgreSQL (austauschbare Speicherschicht)
- **Metriken**: Prometheus + metrics-rs
- **Nachrichtenwarteschlange**: RabbitMQ (optional, für Event-Analytik)
- **Passwort-Hashing**: Argon2

> **Hinweis**: Die Speicher- und Cache-Schichten sind abstrahiert und können durch jede kompatible Datenquelle ersetzt werden. In aktiver Entwicklung.

## Schnellstart

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

## Konfiguration

Erstellen Sie `config.yaml`:

```yaml
server:
  host: "0.0.0.0"
  port: 8080

hashids:
  salts:
    - ${HASHID_SALT}          # Primärer Salt
    - ${HASHID_SALT_OLD}      # Optional: alter Salt für Migration
  min_length: 6

redis:
  url: ${REDIS_URL}
  cache_ttl_seconds: 86400    # 24 Stunden

database:
  url: ${DATABASE_URL}
  pool:
    max_connections: 5
    connect_timeout_seconds: 3
  rate_limit:
    max_requests_per_second: 50
  circuit_breaker:
    failure_threshold: 3
    reset_timeout_seconds: 60
  query:
    table: "dictionary.urls"    # Ihr Tabellenname
    id_column: "id"             # ID-Spalte
    url_column: "name"          # URL-Spalte

interstitial:
  delay_seconds: 5            # Countdown vor Weiterleitung

metrics:
  basic_auth:
    username: prometheus
    password: ${METRICS_PASSWORD}

rate_limit:
  requests_per_second: 1000
  burst: 100
```

### Konfigurationsoptionen

#### Server

| Option | Standard | Beschreibung |
|--------|----------|--------------|
| `host` | `0.0.0.0` | Bind-Adresse |
| `port` | `8080` | HTTP-Port |

#### Hashids

| Option | Standard | Beschreibung |
|--------|----------|--------------|
| `salts` | *erforderlich* | Liste der Hashid-Salts (erster = primär) |
| `min_length` | `6` | Minimale Hashid-Länge |

#### Redis

| Option | Standard | Beschreibung |
|--------|----------|--------------|
| `url` | *erforderlich* | Redis-Verbindungs-URL |
| `cache_ttl_seconds` | `86400` | Cache-TTL in Sekunden |

#### Datenbank

| Option | Standard | Beschreibung |
|--------|----------|--------------|
| `url` | *erforderlich* | PostgreSQL-Verbindungs-URL |
| `pool.max_connections` | `3` | Verbindungspool-Größe |
| `pool.connect_timeout_seconds` | `3` | Verbindungs-Timeout |
| `rate_limit.max_requests_per_second` | `50` | DB-Rate-Limit |
| `circuit_breaker.failure_threshold` | `3` | Fehler vor Öffnung |
| `circuit_breaker.reset_timeout_seconds` | `60` | Circuit-Reset-Timeout |

#### Rate Limit (Global)

| Option | Standard | Beschreibung |
|--------|----------|--------------|
| `requests_per_second` | `1000` | Globales Rate-Limit |
| `burst` | `100` | Burst-Kapazität |

### Umgebungsvariablen

Es gibt **drei Methoden** zur Konfiguration des Dienstes, geordnet nach Priorität (höchste zuerst):

| Priorität | Methode | Anwendungsfall |
|-----------|---------|----------------|
| 1 | `REDIRECTOR__*` Env-Variablen | Einzelne Werte überschreiben |
| 2 | Standard PaaS-Variablen (`DATABASE_URL`, etc.) | PaaS-Plattformen (Railway, Heroku, Render) |
| 3 | Konfigurationsdatei (`config.yaml` oder `CONFIG_BASE64`) | Basiskonfiguration |

#### Spezielle Variablen

| Variable | Standard | Beschreibung |
|----------|----------|--------------|
| `CONFIG_PATH` | `config.yaml` | Pfad zur YAML-Konfigurationsdatei |
| `CONFIG_BASE64` | — | Base64-codierte YAML-Konfiguration (hat Vorrang vor `CONFIG_PATH`) |

#### Standard PaaS-Umgebungsvariablen

Diese werden automatisch erkannt und angewendet. Die meisten PaaS-Plattformen setzen sie automatisch:

| Variable | Konfigurationspfad | Beispiel |
|----------|---------------------|----------|
| `DATABASE_URL` | `database.url` | `postgres://user:pass@host:5432/db` |
| `REDIS_URL` | `redis.url` | `redis://host:6379` |
| `PORT` | `server.port` | `3000` |

> **Prioritätsregel**: Wenn sowohl `DATABASE_URL` als auch `REDIRECTOR__DATABASE__URL` gesetzt sind, gewinnt die `REDIRECTOR__`-Variante.

#### Variablen mit Präfix (`REDIRECTOR__*`)

Jeder Konfigurationswert kann mit dem Präfix `REDIRECTOR__` und `__` (doppelter Unterstrich) als Verschachtelungstrenner überschrieben werden:

```
YAML-Konfigurationspfad    →  Umgebungsvariable
─────────────────────────────────────────────────────
server.port               →  REDIRECTOR__SERVER__PORT
server.host               →  REDIRECTOR__SERVER__HOST
database.url              →  REDIRECTOR__DATABASE__URL
database.pool.max_connections → REDIRECTOR__DATABASE__POOL__MAX_CONNECTIONS
redis.url                 →  REDIRECTOR__REDIS__URL
redis.cache_ttl_seconds   →  REDIRECTOR__REDIS__CACHE_TTL_SECONDS
interstitial.delay_seconds → REDIRECTOR__INTERSTITIAL__DELAY_SECONDS
metrics.basic_auth.username → REDIRECTOR__METRICS__BASIC_AUTH__USERNAME
metrics.basic_auth.password → REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD
rate_limit.requests_per_second → REDIRECTOR__RATE_LIMIT__REQUESTS_PER_SECOND
rate_limit.burst          →  REDIRECTOR__RATE_LIMIT__BURST
admin.enabled             →  REDIRECTOR__ADMIN__ENABLED
admin.session_ttl_hours   →  REDIRECTOR__ADMIN__SESSION_TTL_HOURS
```

#### Beispiele nach Deployment-Plattform

**Railway / Render / Fly.io** (PaaS mit verwalteten Datenbanken):

```bash
# Diese werden normalerweise automatisch von der Plattform gesetzt:
DATABASE_URL=postgres://user:pass@host:5432/db
REDIS_URL=redis://host:6379
PORT=3000

# Konfiguration via Base64:
CONFIG_BASE64=c2VydmVyOgogIGhvc3Q6IC...

# Oder einzelne Werte überschreiben:
REDIRECTOR__HASHIDS__SALTS__0=my-secret-salt
REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD=strong-password
```

**Docker / Docker Compose**:

```yaml
services:
  redirector:
    image: ghcr.io/brilliant-almazov/redirector:latest
    ports:
      - "8080:8080"
    environment:
      DATABASE_URL: "postgres://user:pass@postgres:5432/redirector"
      REDIS_URL: "redis://redis:6379"
      CONFIG_BASE64: "${CONFIG_BASE64}"
      # Oder einzelne Werte über die Konfigurationsdatei hinaus überschreiben:
      REDIRECTOR__RATE_LIMIT__REQUESTS_PER_SECOND: "2000"
      REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD: "${METRICS_PASSWORD}"
    volumes:
      - ./config.yaml:/app/config.yaml  # optional bei CONFIG_BASE64
    depends_on:
      - postgres
      - redis
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
            - name: REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD
              valueFrom:
                secretKeyRef:
                  name: redirector-secrets
                  key: metrics-password
            - name: CONFIG_BASE64
              valueFrom:
                configMapKeyRef:
                  name: redirector-config
                  key: config-base64
```

**Einfaches Docker**:

```bash
docker run -p 8080:8080 \
  -e DATABASE_URL="postgres://user:pass@host:5432/db" \
  -e REDIS_URL="redis://host:6379" \
  -e CONFIG_BASE64="$(cat config.yaml | base64)" \
  ghcr.io/brilliant-almazov/redirector:latest
```

**Minimale Einrichtung (nur Env-Variablen, keine Konfigurationsdatei)**:

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

#### Base64-Konfiguration

Für Umgebungen, in denen das Mounten von Konfigurationsdateien nicht praktikabel ist (PaaS, Serverless, CI/CD), übergeben Sie die gesamte Konfiguration als Base64-codierten String:

```bash
# Codieren
cat config.yaml | base64

# Decodieren (zur Überprüfung)
echo "$CONFIG_BASE64" | base64 -d
```

`CONFIG_BASE64` hat Vorrang vor `CONFIG_PATH`. Umgebungsvariablen-Überschreibungen (`REDIRECTOR__*` und PaaS-Variablen) werden **zusätzlich** zur decodierten Konfiguration angewendet.

## Wie es funktioniert

1. Benutzer besucht `/r/{hashid}` (z.B. `/r/abc123`)
2. Dienst dekodiert Hashid zu numerischer ID
3. Prüft Redis-Cache auf URL
4. Bei Cache-Miss, Abfrage an PostgreSQL
5. Ergebnis in Redis cachen
6. Zwischenseite mit Countdown anzeigen
7. Nach Countdown zur Ziel-URL weiterleiten

## Endpoints

| Endpoint | Auth | Beschreibung |
|----------|------|--------------|
| `GET /` | Nein | Startseite |
| `GET /r/{hashid}` | Nein | Weiterleitung mit Zwischenseite |
| `GET /d/{hashid}` | Nein | Demo-Weiterleitung (synthetischer Lasttest) |
| `GET /health` | Nein | Health-Check |
| `GET /metrics` | Basic | Prometheus-Metriken |
| `GET /admin` | Session | Admin-Dashboard Login |
| `GET /admin/dashboard` | Session | Admin-Dashboard |

## Admin-Dashboard

Der Service enthält ein optionales Admin-Dashboard zur Echtzeit-Überwachung von Metriken.

### Einrichtung

1. **Passwort-Hash generieren:**

```bash
cargo run --bin hash_password
# Passwort eingeben, oder:
cargo run --bin hash_password -- "your-password"
```

2. **Zu config.yaml hinzufügen:**

```yaml
admin:
  enabled: true
  session_ttl_hours: 24
  users:
    - username: admin
      password_hash: "$argon2id$v=19$m=19456,t=2,p=1$..."  # aus Schritt 1
```

3. **Dashboard öffnen:**

Öffnen Sie `http://localhost:8080/admin` und melden Sie sich mit Ihren Zugangsdaten an.

### Funktionen

- Echtzeit RPS- und Latenz-Diagramme
- Systemmetriken (CPU, Speicher, Betriebszeit)
- Cache-Trefferquoten-Überwachung
- Liste der letzten Weiterleitungen
- Lastsimulation für Tests
- Drei Themes: Hell, Dunkel, Warm

## Event-Analyse

Optionale Event-Publishing-Pipeline für Redirect-Analyse. Wenn aktiviert, wird jedes Redirect-Event an RabbitMQ veröffentlicht und von einem separaten Binary verarbeitet, das in PostgreSQL schreibt.

> **Vollständige Dokumentation**: [EVENT_ANALYTICS.md](EVENT_ANALYTICS.md)

### Funktionen

- **Fire-and-forget Publishing** — Redirect-Latenz nicht von Queue-Verfügbarkeit betroffen
- **Batching** — Events werden nach Größe (100) oder Zeit (1 Sek.) gruppiert
- **User-Agent-Parsing** — Browser, Version, OS, Gerätetyp via woothee
- **GeoIP-Anreicherung** — Land und Stadt aus IP (MaxMind mmdb mit Hot-Reload)
- **Referenz-Deduplizierung** — MD5-basierte Deduplikation für Referers und User Agents
- **Monatliche Partitionierung** — Automatische Partitionserstellung für `redirect_events`
- **Domain-Normalisierung** — `WWW.Example.COM` → `example.com`

### Architektur

```
Redirect-Handler
    │
    ├── try_send(RedirectEvent) ──► [tokio::mpsc-Kanal]
    │   (non-blocking,                    │
    │    fire-and-forget)                 ▼
    │                              Hintergrund-Task
    │                              (Batch nach Größe/Zeit)
    │                                     │
    │                                     ▼
    │                                [RabbitMQ-Queue]
    │                                     │
    │                                     ▼
    │                              Event-Consumer
    │                              (separates Binary/Container)
    │                                     │
    │                                     ▼
    │                              [PostgreSQL-Analytik]
    │                              (monatlich partitioniert)
```

### Schnellstart

```bash
# In config.yaml aktivieren
events:
  enabled: true
  rabbitmq:
    url: amqp://guest:guest@localhost:5672/%2f

# Oder über Umgebungsvariablen
REDIRECTOR__EVENTS__ENABLED=true
RABBITMQ_URL=amqp://guest:guest@localhost:5672/%2f

# Consumer starten
RABBITMQ_URL=amqp://... DATABASE_URL=postgres://... cargo run --bin event_consumer
```

### Docker Compose mit Events

```yaml
services:
  redirector:
    build: .
    environment:
      - REDIRECTOR__EVENTS__ENABLED=true
    depends_on: [redis, rabbitmq]

  event_consumer:
    build: .
    command: ["./event_consumer"]
    environment:
      - RABBITMQ_URL=amqp://guest:guest@rabbitmq:5672/%2f
      - DATABASE_URL=postgres://postgres:postgres@analytics-db:5432/analytics
      - GEOIP_DB_PATH=/data/GeoLite2-City.mmdb  # optional
    depends_on: [rabbitmq, analytics-db]

  rabbitmq:
    image: rabbitmq:4-management-alpine
    ports: ["5672:5672", "15672:15672"]

  analytics-db:
    image: postgres:16-alpine
    environment:
      POSTGRES_DB: analytics
```

### Wichtige Designentscheidungen

- **Blockiert Redirects nie**: `try_send()` auf beschränktem Kanal, verwirft Events bei Überlauf
- **Typensicheres Event-Batching**: `EventBatch` ist ein Rust-Enum mit `event_type`-Tag
- **Snowflake-Batch-IDs**: Benutzerdefinierte Epoche 2025-01-01, ~69 Jahre eindeutiger IDs
- **Elegante Verschlechterung**: Wenn RabbitMQ ausfällt, funktionieren Redirects weiter; Events werden mit Metriken verworfen

## Metriken

Der Dienst exponiert umfassende Prometheus-Metriken unter `/metrics` (benötigt Basic Auth):

### Service-Metriken
```
redirector_up 1
redirector_build_info{version="0.1.0"} 1
redirector_uptime_seconds 3600.5
```

### Request-Metriken
```
redirect_requests_total 150000
not_found_requests_total 50
request_duration_seconds{quantile="0.5"} 0.040
request_duration_seconds{quantile="0.99"} 0.081
```

### Cache-Metriken
```
cache_hits_total 140000
cache_misses_total 10000
cache_get_duration_seconds{quantile="0.5"} 0.002
cache_set_duration_seconds{quantile="0.5"} 0.002
```

### Datenbank-Metriken
```
db_queries_total 10000
db_hits_total 9950
db_misses_total 50
db_errors_total 0
db_query_duration_seconds{quantile="0.5"} 0.035
db_rate_limit_exceeded_total 0
circuit_breaker_rejections_total 0
```

### Rate Limiting
```
rate_limit_exceeded_total 0
```

### Events (wenn aktiviert)
```
events_published 50000
events_dropped 0
events_publish_errors 0
events_serialize_errors 0
rabbitmq_connected 1
```

## Lizenz

MIT-Lizenz - siehe [LICENSE](../LICENSE) für Details.

## Mitwirken

Beiträge willkommen! Bitte:

1. Repository forken
2. Feature-Branch erstellen
3. Pull Request einreichen

Geschützter master-Branch erfordert PR-Review.
