# redirector

> **Accorciatore di URL e servizio di reindirizzamento ad alte prestazioni** costruito con Rust, Axum, Redis e PostgreSQL. Include pagine interstiziali sicure, dashboard di amministrazione in tempo reale e osservabilità di livello enterprise.

[English](../README.md) | [Русский](README.ru.md) | [中文](README.zh.md) | [हिंदी](README.hi.md) | [Español](README.es.md) | [Português](README.pt.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Polski](README.pl.md) | [Nederlands](README.nl.md) | **Italiano** | [Türkçe](README.tr.md) | [Українська](README.uk.md) | [עברית](README.he.md) | [Bahasa Indonesia](README.id.md) | [Tiếng Việt](README.vi.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![Docker Image Size](https://ghcr-badge.egpl.dev/brilliant-almazov/redirector/size)](https://github.com/brilliant-almazov/redirector/pkgs/container/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

**Parole chiave**: accorciatore URL, accorciatore link, servizio reindirizzamento, servizio web Rust, framework Axum, cache Redis, PostgreSQL, metriche Prometheus, hashids, link brevi, pagine interstiziali, reindirizzamenti sicuri, alte prestazioni, microservizio

Servizio di reindirizzamento URL sicuro con pagine interstiziali e link brevi basati su hashid. Perfetto per strumenti interni, gestione link aziendali e servizi URL brevi personalizzati.

### Prestazioni

| Scenario | RPS | Latenza Media | Latenza P99 |
|----------|-----|---------------|-------------|
| 100% Cache Hit | **7.800+** | ~14ms | ~50ms |
| Cache Miss (10K URLs) | **2.300+** | ~44ms | ~81ms |

**Condizioni di test**: wrk -t4 -c100 -d30s, PostgreSQL 15, Dragonfly (Redis), macOS M1 (Docker)

> ⚠️ I risultati sono da Docker su macOS con overhead VM. Il deployment nativo su Linux dovrebbe essere **3-5x più veloce**.

## Problema

Condividere URL lunghi è scomodo. Gli accorciatori di URL esistono ma spesso reindirizzano immediatamente, il che può essere un rischio per la sicurezza. Gli utenti dovrebbero vedere dove stanno andando prima di essere reindirizzati.

**redirector** fornisce reindirizzamenti sicuri con:
- Pagina interstiziale che mostra l'URL di destinazione prima del reindirizzamento
- Timer con conto alla rovescia per la consapevolezza dell'utente
- Pagine belle e personalizzate

## Funzionalità

- 🔗 **URL Hashid** - ID brevi, unici e non sequenziali (es. `/r/abc123`)
- ⏱️ **Pagina interstiziale** - Timer con conto alla rovescia mostra l'URL di destinazione prima del reindirizzamento
- ⚡ **Cache Redis** - Ricerche veloci con TTL configurabile
- 🛡️ **Circuit breaker** - Protezione del database contro guasti a cascata
- 🚦 **Limitazione velocità** - Limiti di velocità globali e a livello database
- 📊 **Metriche Prometheus** - Osservabilità completa con protezione Basic Auth
- 🎨 **Pagine belle** - Pagine 404 e indice pulite con 4 temi
- 🔑 **Salt multipli** - Supporto rotazione salt hashid per migrazione
- 📱 **Dashboard admin** - Monitoraggio metriche in tempo reale con SSE

## Screenshot

| Chiaro | Scuro | Grigio | Caldo |
|--------|-------|--------|-------|
| ![Dashboard Chiaro](screenshots/dashboard-light.png) | ![Dashboard Scuro](screenshots/dashboard-dark.png) | ![Dashboard Grigio](screenshots/dashboard-gray.png) | ![Dashboard Caldo](screenshots/dashboard-warm.png) |
| ![Login Chiaro](screenshots/login-light.png) | ![Login Scuro](screenshots/login-dark.png) | ![Login Grigio](screenshots/login-gray.png) | ![Login Caldo](screenshots/login-warm.png) |
| ![Index Light](screenshots/index-light.png) | ![Index Dark](screenshots/index-dark.png) | ![Index Gray](screenshots/index-gray.png) | ![Index Warm](screenshots/index-warm.png) |
| ![Interstitial Light](screenshots/interstitial-light.png) | ![Interstitial Dark](screenshots/interstitial-dark.png) | ![Interstitial Gray](screenshots/interstitial-gray.png) | ![Interstitial Warm](screenshots/interstitial-warm.png) |
| ![404 Light](screenshots/404-light.png) | ![404 Dark](screenshots/404-dark.png) | ![404 Gray](screenshots/404-gray.png) | ![404 Warm](screenshots/404-warm.png) |

### Modal test di carico

| Chiaro | Scuro | Grigio | Caldo |
|--------|-------|--------|-------|
| ![Modal Chiaro](screenshots/dashboard-modal-light.png) | ![Modal Scuro](screenshots/dashboard-modal-dark.png) | ![Modal Grigio](screenshots/dashboard-modal-gray.png) | ![Modal Caldo](screenshots/dashboard-modal-warm.png) |

## Stack tecnologico

- **Linguaggio**: Rust (async con Tokio)
- **Framework web**: Axum
- **Cache**: Compatibile Redis (Redis, Dragonfly, Valkey, KeyDB, ecc.)
- **Database**: PostgreSQL (layer di storage intercambiabile)
- **Metriche**: Prometheus + metrics-rs
- **Hash password**: Argon2

> **Nota**: I layer di storage e cache sono astratti e possono essere sostituiti con qualsiasi fonte dati compatibile. Attualmente in sviluppo attivo.

## Avvio rapido

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

## Configurazione

Creare `config.yaml`:

```yaml
server:
  host: "0.0.0.0"
  port: 8080

hashids:
  salts:
    - ${HASHID_SALT}          # Salt primario
    - ${HASHID_SALT_OLD}      # Opzionale: vecchio salt per migrazione
  min_length: 6

redis:
  url: ${REDIS_URL}
  cache_ttl_seconds: 86400    # 24 ore

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
    table: "dictionary.urls"    # Nome della tabella
    id_column: "id"             # Colonna ID
    url_column: "name"          # Colonna URL

interstitial:
  delay_seconds: 5            # Conto alla rovescia prima del reindirizzamento

metrics:
  basic_auth:
    username: prometheus
    password: ${METRICS_PASSWORD}

rate_limit:
  requests_per_second: 1000
  burst: 100
```

### Opzioni di configurazione

#### Server

| Opzione | Predefinito | Descrizione |
|---------|-------------|-------------|
| `host` | `0.0.0.0` | Indirizzo di binding |
| `port` | `8080` | Porta HTTP |

#### Hashids

| Opzione | Predefinito | Descrizione |
|---------|-------------|-------------|
| `salts` | *obbligatorio* | Lista dei salt hashid (primo = primario) |
| `min_length` | `6` | Lunghezza minima hashid |

#### Redis

| Opzione | Predefinito | Descrizione |
|---------|-------------|-------------|
| `url` | *obbligatorio* | URL di connessione Redis |
| `cache_ttl_seconds` | `86400` | TTL cache in secondi |

#### Database

| Opzione | Predefinito | Descrizione |
|---------|-------------|-------------|
| `url` | *obbligatorio* | URL di connessione PostgreSQL |
| `pool.max_connections` | `3` | Dimensione del pool di connessioni |
| `pool.connect_timeout_seconds` | `3` | Timeout di connessione |
| `rate_limit.max_requests_per_second` | `50` | Rate limit DB |
| `circuit_breaker.failure_threshold` | `3` | Errori prima dell'apertura |
| `circuit_breaker.reset_timeout_seconds` | `60` | Timeout reset circuit |

#### Rate Limit (Globale)

| Opzione | Predefinito | Descrizione |
|---------|-------------|-------------|
| `requests_per_second` | `1000` | Rate limit globale |
| `burst` | `100` | Capacità burst |

### Variabili d'ambiente

Ci sono **tre modi** per configurare il servizio, elencati per priorità (più alta prima):

| Priorità | Metodo | Caso d'uso |
|----------|--------|------------|
| 1 | Variabili `REDIRECTOR__*` | Sovrascrivere valori individuali |
| 2 | Variabili PaaS standard (`DATABASE_URL`, ecc.) | Piattaforme PaaS (Railway, Heroku, Render) |
| 3 | File di configurazione (`config.yaml` o `CONFIG_BASE64`) | Configurazione base |

#### Variabili speciali

| Variabile | Predefinito | Descrizione |
|-----------|-------------|-------------|
| `CONFIG_PATH` | `config.yaml` | Percorso al file di configurazione YAML |
| `CONFIG_BASE64` | — | Configurazione YAML codificata in Base64 (priorità su `CONFIG_PATH`) |

#### Variabili d'ambiente PaaS standard

Queste vengono riconosciute e applicate automaticamente. La maggior parte delle piattaforme PaaS le imposta per te:

| Variabile | Percorso configurazione | Esempio |
|-----------|------------------------|---------|
| `DATABASE_URL` | `database.url` | `postgres://user:pass@host:5432/db` |
| `REDIS_URL` | `redis.url` | `redis://host:6379` |
| `PORT` | `server.port` | `3000` |

> **Regola di priorità**: Se sia `DATABASE_URL` che `REDIRECTOR__DATABASE__URL` sono impostati, la versione con prefisso `REDIRECTOR__` ha la precedenza.

#### Variabili con prefisso (`REDIRECTOR__*`)

Qualsiasi valore di configurazione può essere sovrascritto usando il prefisso `REDIRECTOR__` con `__` (doppio underscore) come separatore di annidamento:

```
Percorso YAML configurazione →  Variabile d'ambiente
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

#### Esempi per piattaforma di deployment

**Railway / Render / Fly.io** (PaaS con database gestiti):

```bash
# Queste vengono solitamente impostate automaticamente dalla piattaforma:
DATABASE_URL=postgres://user:pass@host:5432/db
REDIS_URL=redis://host:6379
PORT=3000

# Imposta la configurazione via base64:
CONFIG_BASE64=c2VydmVyOgogIGhvc3Q6IC...

# O sovrascrivi valori individuali:
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
      # O sovrascrivi valori individuali sopra il file di configurazione:
      REDIRECTOR__RATE_LIMIT__REQUESTS_PER_SECOND: "2000"
      REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD: "${METRICS_PASSWORD}"
    volumes:
      - ./config.yaml:/app/config.yaml  # opzionale con CONFIG_BASE64
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

**Docker semplice**:

```bash
docker run -p 8080:8080 \
  -e DATABASE_URL="postgres://user:pass@host:5432/db" \
  -e REDIS_URL="redis://host:6379" \
  -e CONFIG_BASE64="$(cat config.yaml | base64)" \
  ghcr.io/brilliant-almazov/redirector:latest
```

**Setup minimale (solo variabili d'ambiente, nessun file di configurazione)**:

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

#### Configurazione Base64

Per ambienti dove il montaggio di file di configurazione non è pratico (PaaS, serverless, CI/CD), passare l'intera configurazione come stringa codificata in Base64:

```bash
# Codifica
cat config.yaml | base64

# Decodifica (per verifica)
echo "$CONFIG_BASE64" | base64 -d
```

`CONFIG_BASE64` ha priorità su `CONFIG_PATH`. Le sovrascritture delle variabili d'ambiente (`REDIRECTOR__*` e variabili PaaS) vengono applicate **sopra** la configurazione decodificata.

## Come funziona

1. L'utente visita `/r/{hashid}` (es. `/r/abc123`)
2. Il servizio decodifica l'hashid in ID numerico
3. Controlla la cache Redis per l'URL
4. In caso di cache miss, interroga PostgreSQL
5. Memorizza il risultato nella cache Redis
6. Mostra la pagina interstiziale con conto alla rovescia
7. Dopo il conto alla rovescia, reindirizza all'URL di destinazione

## Endpoint

| Endpoint | Auth | Descrizione |
|----------|------|-------------|
| `GET /` | No | Pagina indice |
| `GET /r/{hashid}` | No | Reindirizzamento con pagina interstiziale |
| `GET /d/{hashid}` | No | Reindirizzamento demo (test di carico sintetico) |
| `GET /health` | No | Health check |
| `GET /metrics` | Basic | Metriche Prometheus |
| `GET /admin` | Session | Login dashboard admin |
| `GET /admin/dashboard` | Session | Dashboard admin |

## Dashboard admin

Il servizio include una dashboard admin opzionale per monitorare le metriche in tempo reale.

### Configurazione

1. **Genera hash password:**

```bash
cargo run --bin hash_password
# Inserisci password, oppure:
cargo run --bin hash_password -- "your-password"
```

2. **Aggiungi a config.yaml:**

```yaml
admin:
  enabled: true
  session_ttl_hours: 24
  users:
    - username: admin
      password_hash: "$argon2id$v=19$m=19456,t=2,p=1$..."  # dal passaggio 1
```

3. **Accedi alla dashboard:**

Apri `http://localhost:8080/admin` e accedi con le tue credenziali.

### Funzionalità

- Grafici RPS e latenza in tempo reale
- Metriche di sistema (CPU, memoria, uptime)
- Monitoraggio tasso cache hit
- Lista reindirizzamenti recenti
- Simulazione carico per test
- Tre temi: Chiaro, Scuro, Caldo

## Licenza

Licenza MIT - vedi [LICENSE](../LICENSE) per dettagli.

## Contribuire

I contributi sono benvenuti! Per favore:

1. Fai fork del repository
2. Crea un branch per la funzionalità
3. Invia una Pull Request

Il branch master protetto richiede revisione PR.
