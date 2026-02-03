# redirector

[English](../README.md) | [Русский](README.ru.md) | [中文](README.zh.md) | [हिंदी](README.hi.md) | [Español](README.es.md) | [Português](README.pt.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | **Polski** | [Nederlands](README.nl.md) | [Italiano](README.it.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) | [Bahasa Indonesia](README.id.md) | [Tiếng Việt](README.vi.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

Bezpieczna usługa przekierowań URL ze stronami przejściowymi i krótkimi linkami opartymi na hashid.

## Problem

Udostępnianie długich adresów URL jest niewygodne. Skracacze URL istnieją, ale często przekierowują natychmiast, co może stanowić zagrożenie bezpieczeństwa. Użytkownicy powinni widzieć, dokąd zmierzają, zanim zostaną przekierowani.

**redirector** zapewnia bezpieczne przekierowania z:
- Stroną przejściową pokazującą docelowy URL przed przekierowaniem
- Odliczaniem dla świadomości użytkownika
- Pięknymi, brandowanymi stronami

## Funkcje

- 🔗 **URL-e Hashid** - Krótkie, unikalne, niesekwencyjne ID (np. `/r/abc123`)
- ⏱️ **Strona przejściowa** - Odliczanie pokazuje docelowy URL przed przekierowaniem
- ⚡ **Cache Redis** - Szybkie wyszukiwania z konfigurowalnym TTL
- 🛡️ **Circuit breaker** - Ochrona bazy danych przed awariami kaskadowymi
- 🚦 **Ograniczanie szybkości** - Globalne i na poziomie bazy danych
- 📊 **Metryki Prometheus** - Pełna obserwowalność z ochroną Basic Auth
- 🎨 **Piękne strony** - Czyste strony 404 i indeksu
- 🔑 **Wiele soli** - Wsparcie rotacji soli hashid dla migracji

## Szybki Start

### Docker

```bash
docker run -p 8080:8080 \
  -v $(pwd)/config.yaml:/config.yaml \
  ghcr.io/brilliant-almazov/redirector:latest
```

## Endpointy

| Endpoint | Auth | Opis |
|----------|------|------|
| `GET /` | Nie | Strona główna |
| `GET /r/{hashid}` | Nie | Przekierowanie ze stroną przejściową |
| `GET /health` | Nie | Sprawdzenie stanu |
| `GET /metrics` | Basic | Metryki Prometheus |
| `GET /admin` | Sesja | Logowanie do panelu admina |
| `GET /admin/dashboard` | Sesja | Panel administracyjny |

## Panel Administracyjny

Usługa zawiera opcjonalny panel administracyjny do monitorowania metryk w czasie rzeczywistym.

### Konfiguracja

1. **Wygeneruj hash hasła:**

```bash
cargo run --bin hash_password
# Wprowadź hasło po wyświetleniu monitu, lub:
cargo run --bin hash_password -- "twoje-hasło"
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

3. **Uzyskaj dostęp do panelu:**

Otwórz `http://localhost:8080/admin` i zaloguj się swoimi danymi.

### Funkcje

- Wykresy RPS i opóźnień w czasie rzeczywistym
- Metryki systemu (CPU, pamięć, uptime)
- Monitorowanie współczynnika trafień cache
- Lista ostatnich przekierowań
- Symulacja obciążenia do testów
- Trzy motywy: Jasny, Ciemny, Ciepły

## Jak to Działa

1. Użytkownik odwiedza `/r/{hashid}` (np. `/r/abc123`)
2. Usługa dekoduje hashid do numerycznego ID
3. Sprawdza cache Redis dla URL
4. Przy braku w cache, odpytuje PostgreSQL
5. Zapisuje wynik w cache Redis
6. Pokazuje stronę przejściową z odliczaniem
7. Po odliczaniu przekierowuje na docelowy URL

## Licencja

Licencja MIT - szczegóły w [LICENSE](../LICENSE).
