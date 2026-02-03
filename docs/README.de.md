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

## Screenshots

| Hell | Dunkel | Grau | Warm |
|------|--------|------|------|
| ![Dashboard Hell](screenshots/dashboard-light.png) | ![Dashboard Dunkel](screenshots/dashboard-dark.png) | ![Dashboard Grau](screenshots/dashboard-gray.png) | ![Dashboard Warm](screenshots/dashboard-warm.png) |
| ![Login Hell](screenshots/login-light.png) | ![Login Dunkel](screenshots/login-dark.png) | ![Login Grau](screenshots/login-gray.png) | ![Login Warm](screenshots/login-warm.png) |

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

## Lizenz

MIT-Lizenz - siehe [LICENSE](../LICENSE) für Details.

## Mitwirken

Beiträge willkommen! Bitte:

1. Repository forken
2. Feature-Branch erstellen
3. Pull Request einreichen

Geschützter master-Branch erfordert PR-Review.
