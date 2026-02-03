# redirector

[English](../README.md) | [Русский](README.ru.md) | [中文](README.zh.md) | [हिंदी](README.hi.md) | [Español](README.es.md) | [Português](README.pt.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Polski](README.pl.md) | **Nederlands** | [Italiano](README.it.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) | [Bahasa Indonesia](README.id.md) | [Tiếng Việt](README.vi.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

Veilige URL-omleidingsservice met tussenpagina's en hashid-gebaseerde korte links.

## Probleem

Het delen van lange URL's is onhandig. URL-verkorters bestaan, maar leiden vaak direct door, wat een veiligheidsrisico kan zijn. Gebruikers moeten zien waar ze naartoe gaan voordat ze worden doorgestuurd.

**redirector** biedt veilige omleidingen met:
- Tussenpagina die doel-URL toont voor omleiding
- Afteltimer voor gebruikersbewustzijn
- Mooie, merkgebonden pagina's

## Functies

- 🔗 **Hashid URL's** - Korte, unieke, niet-sequentiële ID's (bijv. `/r/abc123`)
- ⏱️ **Tussenpagina** - Aftelling toont doel-URL voor omleiding
- ⚡ **Redis caching** - Snelle opzoekingen met configureerbare TTL
- 🛡️ **Circuit breaker** - Databasebescherming tegen cascade-fouten
- 🚦 **Rate limiting** - Globale en database-niveau limieten
- 📊 **Prometheus metrics** - Volledige observeerbaarheid met Basic Auth-bescherming
- 🎨 **Mooie pagina's** - Schone 404- en indexpagina's
- 🔑 **Meerdere salts** - Hashid salt-rotatie ondersteuning voor migratie

## Schermafbeeldingen

| Licht | Donker | Warm |
|-------|--------|------|
| ![Dashboard Licht](screenshots/dashboard-light.png) | ![Dashboard Donker](screenshots/dashboard-dark.png) | ![Dashboard Warm](screenshots/dashboard-warm.png) |
| ![Login Licht](screenshots/login-light.png) | ![Login Donker](screenshots/login-dark.png) | ![Login Warm](screenshots/login-warm.png) |
| ![404 Licht](screenshots/not-found-light.png) | ![404 Donker](screenshots/not-found-dark.png) | ![404 Warm](screenshots/not-found-warm.png) |

| Indexpagina | Tussenpagina |
|-------------|--------------|
| ![Index](screenshots/index.png) | ![Tussenpagina](screenshots/interstitial.png) |

## Technologie Stack

- **Taal**: Rust (async met Tokio)
- **Web Framework**: Axum
- **Cache**: Redis-compatible (Redis, Dragonfly, Valkey, KeyDB)
- **Database**: PostgreSQL (pluggable opslaglaag)
- **Metrics**: Prometheus + metrics-rs
- **Wachtwoord Hashing**: Argon2

> **Opmerking**: PostgreSQL wordt gebruikt als standaard opslagbackend. De opslaglaag is geabstraheerd en kan worden vervangen door elke gegevensbron. Momenteel in actieve ontwikkeling.

## Snel Starten

### Docker

```bash
docker run -p 8080:8080 \
  -v $(pwd)/config.yaml:/config.yaml \
  ghcr.io/brilliant-almazov/redirector:latest
```

## Eindpunten

| Eindpunt | Auth | Beschrijving |
|----------|------|--------------|
| `GET /` | Nee | Startpagina |
| `GET /r/{hashid}` | Nee | Omleiding met tussenpagina |
| `GET /health` | Nee | Gezondheidscontrole |
| `GET /metrics` | Basic | Prometheus metrics |
| `GET /admin` | Sessie | Admin dashboard login |
| `GET /admin/dashboard` | Sessie | Admin dashboard |

## Admin Dashboard

De service bevat een optioneel admin dashboard voor het monitoren van live metrics.

### Instellen

1. **Wachtwoord-hash genereren:**

```bash
cargo run --bin hash_password
# Voer wachtwoord in wanneer gevraagd, of:
cargo run --bin hash_password -- "jouw-wachtwoord"
```

2. **Toevoegen aan config.yaml:**

```yaml
admin:
  enabled: true
  session_ttl_hours: 24
  users:
    - username: admin
      password_hash: "$argon2id$v=19$m=19456,t=2,p=1$..."  # van stap 1
```

3. **Dashboard openen:**

Open `http://localhost:8080/admin` en log in met je gegevens.

### Functies

- Real-time RPS en latentie grafieken
- Systeemmetrics (CPU, geheugen, uptime)
- Cache hit rate monitoring
- Lijst van recente omleidingen
- Belastingssimulatie voor testen
- Drie thema's: Licht, Donker, Warm

## Hoe Het Werkt

1. Gebruiker bezoekt `/r/{hashid}` (bijv. `/r/abc123`)
2. Service decodeert hashid naar numeriek ID
3. Controleert Redis cache voor URL
4. Bij cache miss, query PostgreSQL
5. Cachet resultaat in Redis
6. Toont tussenpagina met aftelling
7. Na aftelling, omleiding naar doel-URL

## Licentie

MIT-licentie - zie [LICENSE](../LICENSE) voor details.
