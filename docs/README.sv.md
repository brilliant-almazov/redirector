# redirector

[English](../README.md) | [Русский](README.ru.md) | [中文](README.zh.md) | [हिंदी](README.hi.md) | [Español](README.es.md) | [Português](README.pt.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Polski](README.pl.md) | [Nederlands](README.nl.md) | [Italiano](README.it.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) | [Bahasa Indonesia](README.id.md) | [Tiếng Việt](README.vi.md) | **Svenska** | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

Säker URL-omdirigeringstjänst med mellansidor och hashid-baserade kortlänkar.

## Problem

Att dela långa URL:er är opraktiskt. URL-förkortare finns men omdirigerar ofta omedelbart, vilket kan vara en säkerhetsrisk. Användare bör se vart de är på väg innan de omdirigeras.

**redirector** erbjuder säkra omdirigeringar med:
- Mellansida som visar mål-URL före omdirigering
- Nedräkningstimer för användarmedvetenhet
- Vackra, varumärkta sidor

## Funktioner

- 🔗 **Hashid-URL:er** - Korta, unika, icke-sekventiella ID:n (t.ex. `/r/abc123`)
- ⏱️ **Mellansida** - Nedräkning visar mål-URL före omdirigering
- ⚡ **Redis-cachning** - Snabba sökningar med konfigurerbar TTL
- 🛡️ **Circuit breaker** - Databasskydd mot kaskadfel
- 🚦 **Hastighetsbegränsning** - Globala och databasnivå-gränser
- 📊 **Prometheus-mätvärden** - Full observerbarhet med Basic Auth-skydd
- 🎨 **Vackra sidor** - Rena 404- och indexsidor
- 🔑 **Flera salter** - Hashid-saltrotationsstöd för migrering

## Snabbstart

### Docker

```bash
docker run -p 8080:8080 \
  -v $(pwd)/config.yaml:/config.yaml \
  ghcr.io/brilliant-almazov/redirector:latest
```

## Slutpunkter

| Slutpunkt | Auth | Beskrivning |
|-----------|------|-------------|
| `GET /` | Nej | Startsida |
| `GET /r/{hashid}` | Nej | Omdirigering med mellansida |
| `GET /health` | Nej | Hälsokontroll |
| `GET /metrics` | Basic | Prometheus-mätvärden |
| `GET /admin` | Session | Admin-dashboard inloggning |
| `GET /admin/dashboard` | Session | Admin-dashboard |

## Admin-Dashboard

Tjänsten inkluderar en valfri admin-dashboard för att övervaka live-mätvärden.

### Konfiguration

1. **Generera lösenords-hash:**

```bash
cargo run --bin hash_password
# Ange lösenord vid uppmaning, eller:
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

3. **Öppna dashboard:**

Öppna `http://localhost:8080/admin` och logga in med dina uppgifter.

### Funktioner

- Realtids-RPS och latensdiagram
- Systemmätvärden (CPU, minne, drifttid)
- Övervakning av cache-träffar
- Lista över senaste omdirigeringar
- Belastningssimulering för testning
- Tre teman: Ljust, Mörkt, Varmt

## Hur Det Fungerar

1. Användare besöker `/r/{hashid}` (t.ex. `/r/abc123`)
2. Tjänsten avkodar hashid till numeriskt ID
3. Kontrollerar Redis-cache för URL
4. Vid cache-miss, frågar PostgreSQL
5. Cachar resultatet i Redis
6. Visar mellansida med nedräkning
7. Efter nedräkning, omdirigerar till mål-URL

## Licens

MIT-licens - se [LICENSE](../LICENSE) för detaljer.
