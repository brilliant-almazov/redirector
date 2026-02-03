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
- 🎨 **Pagine belle** - Pagine 404 e indice pulite con 3 temi
- 🔑 **Salt multipli** - Supporto rotazione salt hashid per migrazione
- 📱 **Dashboard admin** - Monitoraggio metriche in tempo reale con SSE

## Screenshot

| Chiaro | Scuro | Caldo |
|--------|-------|-------|
| ![Dashboard Chiaro](screenshots/dashboard-light.png) | ![Dashboard Scuro](screenshots/dashboard-dark.png) | ![Dashboard Caldo](screenshots/dashboard-warm.png) |
| ![Login Chiaro](screenshots/login-light.png) | ![Login Scuro](screenshots/login-dark.png) | ![Login Caldo](screenshots/login-warm.png) |
| ![404 Chiaro](screenshots/not-found-light.png) | ![404 Scuro](screenshots/not-found-dark.png) | ![404 Caldo](screenshots/not-found-warm.png) |

| Pagina indice | Interstiziale |
|---------------|---------------|
| ![Indice](screenshots/index.png) | ![Interstiziale](screenshots/interstitial.png) |

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
