# redirector

[English](../README.md) | [Русский](README.ru.md) | [中文](README.zh.md) | [हिंदी](README.hi.md) | [Español](README.es.md) | [Português](README.pt.md) | [Français](README.fr.md) | **Deutsch** | [日本語](README.ja.md) | [한국어](README.ko.md) | [Polski](README.pl.md) | [Nederlands](README.nl.md) | [Italiano](README.it.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) | [Bahasa Indonesia](README.id.md) | [Tiếng Việt](README.vi.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

Sicherer URL-Weiterleitungsdienst mit Zwischenseiten und hashid-basierten Kurzlinks.

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
- 🎨 **Schöne Seiten** - Saubere 404- und Index-Seiten
- 🔑 **Mehrere Salts** - Hashid-Salt-Rotation für Migration

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

## Wie es funktioniert

1. Benutzer besucht `/r/{hashid}` (z.B. `/r/abc123`)
2. Dienst dekodiert Hashid zu numerischer ID
3. Prüft Redis-Cache auf URL
4. Bei Cache-Miss, Abfrage an PostgreSQL
5. Ergebnis in Redis cachen
6. Zwischenseite mit Countdown anzeigen
7. Nach Countdown zur Ziel-URL weiterleiten

## Lizenz

MIT-Lizenz - siehe [LICENSE](../LICENSE) für Details.
