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
