# redirector

> **Wysokowydajny skracacz URL i usługa przekierowań** zbudowany z Rust, Axum, Redis i PostgreSQL. Zawiera bezpieczne strony przejściowe, panel administracyjny w czasie rzeczywistym i obserwowalność klasy enterprise.

[English](../README.md) | [Русский](README.ru.md) | [中文](README.zh.md) | [हिंदी](README.hi.md) | [Español](README.es.md) | [Português](README.pt.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | **Polski** | [Nederlands](README.nl.md) | [Italiano](README.it.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) | [עברית](README.he.md) | [Bahasa Indonesia](README.id.md) | [Tiếng Việt](README.vi.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![Docker Image Size](https://ghcr-badge.egpl.dev/brilliant-almazov/redirector/size)](https://github.com/brilliant-almazov/redirector/pkgs/container/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

**Słowa kluczowe**: skracacz URL, skracacz linków, usługa przekierowań, usługa web Rust, framework Axum, cache Redis, PostgreSQL, metryki Prometheus, hashids, krótkie linki, strony przejściowe, bezpieczne przekierowania, wysoka wydajność, mikroserwis

Bezpieczna usługa przekierowań URL ze stronami przejściowymi i krótkimi linkami opartymi na hashid. Idealna dla narzędzi wewnętrznych, zarządzania linkami korporacyjnymi i markowych usług skracania URL.

### Wydajność

| Scenariusz | RPS | Średnie opóźnienie | Opóźnienie P99 |
|------------|-----|-------------------|----------------|
| 100% Cache Hit | **7 800+** | ~14ms | ~50ms |
| Cache Miss (10K URLs) | **2 300+** | ~44ms | ~81ms |

**Warunki testu**: wrk -t4 -c100 -d30s, PostgreSQL 15, Dragonfly (Redis), macOS M1 (Docker)

> ⚠️ Wyniki pochodzą z Docker na macOS z narzutem VM. Natywne wdrożenie na Linux powinno być **3-5x szybsze**.

## Problem

Udostępnianie długich adresów URL jest niewygodne. Skracacze URL istnieją, ale często przekierowują natychmiast, co może stanowić zagrożenie bezpieczeństwa. Użytkownicy powinni widzieć, dokąd zmierzają, zanim zostaną przekierowani.

**redirector** zapewnia bezpieczne przekierowania z:
- Stroną przejściową pokazującą docelowy URL przed przekierowaniem
- Licznikiem odliczającym dla świadomości użytkownika
- Pięknymi, markowanymi stronami

## Funkcje

- 🔗 **URL Hashid** - Krótkie, unikalne, niesekwencyjne ID (np. `/r/abc123`)
- ⏱️ **Strona przejściowa** - Licznik odliczający pokazuje docelowy URL przed przekierowaniem
- ⚡ **Cache Redis** - Szybkie wyszukiwania z konfigurowalnym TTL
- 🛡️ **Circuit breaker** - Ochrona bazy danych przed awariami kaskadowymi
- 🚦 **Limitowanie prędkości** - Globalne i na poziomie bazy danych limity prędkości
- 📊 **Metryki Prometheus** - Pełna obserwowalność z ochroną Basic Auth
- 🎨 **Piękne strony** - Czyste strony 404 i indeksu z 3 motywami
- 🔑 **Wiele soli** - Wsparcie dla rotacji soli hashid dla migracji
- 📱 **Panel administracyjny** - Monitoring metryk w czasie rzeczywistym przez SSE

## Zrzuty ekranu

| Jasny | Ciemny | Ciepły |
|-------|--------|--------|
| ![Dashboard Jasny](screenshots/dashboard-light.png) | ![Dashboard Ciemny](screenshots/dashboard-dark.png) | ![Dashboard Ciepły](screenshots/dashboard-warm.png) |
| ![Logowanie Jasny](screenshots/login-light.png) | ![Logowanie Ciemny](screenshots/login-dark.png) | ![Logowanie Ciepły](screenshots/login-warm.png) |
| ![404 Jasny](screenshots/not-found-light.png) | ![404 Ciemny](screenshots/not-found-dark.png) | ![404 Ciepły](screenshots/not-found-warm.png) |

| Strona główna | Przejściowa |
|---------------|-------------|
| ![Strona główna](screenshots/index.png) | ![Przejściowa](screenshots/interstitial.png) |

## Stos technologiczny

- **Język**: Rust (async z Tokio)
- **Framework web**: Axum
- **Cache**: Kompatybilny z Redis (Redis, Dragonfly, Valkey, KeyDB itp.)
- **Baza danych**: PostgreSQL (wymienna warstwa przechowywania)
- **Metryki**: Prometheus + metrics-rs
- **Hashowanie haseł**: Argon2

> **Uwaga**: Warstwy przechowywania i cache są abstrakcyjne i mogą być zastąpione dowolnym kompatybilnym źródłem danych. Obecnie w aktywnym rozwoju.

## Szybki start

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

## Jak to działa

1. Użytkownik odwiedza `/r/{hashid}` (np. `/r/abc123`)
2. Usługa dekoduje hashid do numerycznego ID
3. Sprawdza cache Redis dla URL
4. W przypadku braku w cache, odpytuje PostgreSQL
5. Zapisuje wynik w cache Redis
6. Wyświetla stronę przejściową z odliczaniem
7. Po odliczaniu przekierowuje do docelowego URL

## Endpointy

| Endpoint | Auth | Opis |
|----------|------|------|
| `GET /` | Nie | Strona główna |
| `GET /r/{hashid}` | Nie | Przekierowanie ze stroną przejściową |
| `GET /d/{hashid}` | Nie | Demo przekierowanie (syntetyczne testowanie obciążenia) |
| `GET /health` | Nie | Health check |
| `GET /metrics` | Basic | Metryki Prometheus |
| `GET /admin` | Session | Logowanie do panelu admina |
| `GET /admin/dashboard` | Session | Panel administracyjny |

## Panel administracyjny

Usługa zawiera opcjonalny panel administracyjny do monitorowania metryk w czasie rzeczywistym.

### Konfiguracja

1. **Wygeneruj hash hasła:**

```bash
cargo run --bin hash_password
# Wprowadź hasło, lub:
cargo run --bin hash_password -- "your-password"
```

2. **Dodaj do config.yaml:**

```yaml
admin:
  enabled: true
  session_ttl_hours: 24
  users:
    - username: admin
      password_hash: "$argon2id$v=19$m=19456,t=2,p=1$..."  # z kroku 1
```

3. **Dostęp do panelu:**

Otwórz `http://localhost:8080/admin` i zaloguj się swoimi danymi.

### Funkcje

- Wykresy RPS i opóźnień w czasie rzeczywistym
- Metryki systemu (CPU, pamięć, uptime)
- Monitoring współczynnika trafień cache
- Lista ostatnich przekierowań
- Symulacja obciążenia do testów
- Trzy motywy: Jasny, Ciemny, Ciepły

## Licencja

Licencja MIT - szczegóły w [LICENSE](../LICENSE).

## Współpraca

Wkład jest mile widziany! Proszę:

1. Zrób fork repozytorium
2. Stwórz branch funkcjonalności
3. Wyślij Pull Request

Chroniony branch master wymaga przeglądu PR.
