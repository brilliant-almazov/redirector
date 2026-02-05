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
- 🎨 **Piękne strony** - Czyste strony 404 i indeksu z 4 motywami
- 🔑 **Wiele soli** - Wsparcie dla rotacji soli hashid dla migracji
- 📱 **Panel administracyjny** - Monitoring metryk w czasie rzeczywistym przez SSE
- 📤 **Analityka zdarzeń** - Opcjonalne publikowanie zdarzeń do RabbitMQ z konsumentem PostgreSQL

## Zrzuty ekranu

| Jasny | Ciemny | Szary | Ciepły |
|-------|--------|-------|--------|
| ![Dashboard Jasny](screenshots/dashboard-light.png) | ![Dashboard Ciemny](screenshots/dashboard-dark.png) | ![Dashboard Szary](screenshots/dashboard-gray.png) | ![Dashboard Ciepły](screenshots/dashboard-warm.png) |
| ![Logowanie Jasny](screenshots/login-light.png) | ![Logowanie Ciemny](screenshots/login-dark.png) | ![Logowanie Szary](screenshots/login-gray.png) | ![Logowanie Ciepły](screenshots/login-warm.png) |
| ![Index Light](screenshots/index-light.png) | ![Index Dark](screenshots/index-dark.png) | ![Index Gray](screenshots/index-gray.png) | ![Index Warm](screenshots/index-warm.png) |
| ![Interstitial Light](screenshots/interstitial-light.png) | ![Interstitial Dark](screenshots/interstitial-dark.png) | ![Interstitial Gray](screenshots/interstitial-gray.png) | ![Interstitial Warm](screenshots/interstitial-warm.png) |
| ![404 Light](screenshots/404-light.png) | ![404 Dark](screenshots/404-dark.png) | ![404 Gray](screenshots/404-gray.png) | ![404 Warm](screenshots/404-warm.png) |

### Modal testu obciążenia

| Jasny | Ciemny | Szary | Ciepły |
|-------|--------|-------|--------|
| ![Modal Jasny](screenshots/dashboard-modal-light.png) | ![Modal Ciemny](screenshots/dashboard-modal-dark.png) | ![Modal Szary](screenshots/dashboard-modal-gray.png) | ![Modal Ciepły](screenshots/dashboard-modal-warm.png) |

## Stos technologiczny

- **Język**: Rust (async z Tokio)
- **Framework web**: Axum
- **Cache**: Kompatybilny z Redis (Redis, Dragonfly, Valkey, KeyDB itp.)
- **Baza danych**: PostgreSQL (wymienna warstwa przechowywania)
- **Metryki**: Prometheus + metrics-rs
- **Kolejka wiadomości**: RabbitMQ (opcjonalnie, dla analityki zdarzeń)
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

### Zmienne środowiskowe

Istnieją **trzy sposoby** konfiguracji usługi, uszeregowane według priorytetu (najwyższy pierwszy):

| Priorytet | Metoda | Zastosowanie |
|-----------|--------|--------------|
| 1 | Zmienne środowiskowe `REDIRECTOR__*` | Nadpisywanie pojedynczych wartości |
| 2 | Standardowe zmienne PaaS (`DATABASE_URL`, itp.) | Platformy PaaS (Railway, Heroku, Render) |
| 3 | Plik konfiguracyjny (`config.yaml` lub `CONFIG_BASE64`) | Konfiguracja bazowa |

#### Zmienne specjalne

| Zmienna | Domyślnie | Opis |
|---------|-----------|------|
| `CONFIG_PATH` | `config.yaml` | Ścieżka do pliku konfiguracyjnego YAML |
| `CONFIG_BASE64` | — | Konfiguracja YAML zakodowana w Base64 (ma priorytet nad `CONFIG_PATH`) |

#### Standardowe zmienne środowiskowe PaaS

Są automatycznie rozpoznawane i stosowane. Większość platform PaaS ustawia je automatycznie:

| Zmienna | Ścieżka konfiguracji | Przykład |
|---------|----------------------|----------|
| `DATABASE_URL` | `database.url` | `postgres://user:pass@host:5432/db` |
| `REDIS_URL` | `redis.url` | `redis://host:6379` |
| `PORT` | `server.port` | `3000` |
| `HASHIDS_SALTS` | `hashids.salts` | `new-salt,old-salt` (rozdzielone przecinkami) |

> **Reguła priorytetu**: Jeśli ustawione są zarówno `DATABASE_URL`, jak i `REDIRECTOR__DATABASE__URL`, wygrywa wersja z prefiksem `REDIRECTOR__`. Analogicznie, `REDIRECTOR__HASHIDS__SALTS__0` ma priorytet nad `HASHIDS_SALTS`.

#### Zmienne środowiskowe z prefiksem (`REDIRECTOR__*`)

Każdą wartość konfiguracji można nadpisać za pomocą prefiksu `REDIRECTOR__` z `__` (podwójny podkreślnik) jako separatorem zagnieżdżenia. Poniżej znajduje się **pełna referencja** wszystkich nadpisywalnych zmiennych:

##### Server

| Zmienna środowiskowa | Ścieżka konfiguracji | Domyślnie | Opis |
|----------------------|----------------------|-----------|------|
| `REDIRECTOR__SERVER__HOST` | `server.host` | `0.0.0.0` | Adres nasłuchiwania |
| `REDIRECTOR__SERVER__PORT` | `server.port` | `8080` | Port HTTP |

##### Hashids

| Zmienna środowiskowa | Ścieżka konfiguracji | Domyślnie | Opis |
|----------------------|----------------------|-----------|------|
| `REDIRECTOR__HASHIDS__SALTS__0` | `hashids.salts[0]` | *wymagane* | Podstawowy salt hashid |
| `REDIRECTOR__HASHIDS__SALTS__1` | `hashids.salts[1]` | — | Stary salt (do migracji) |
| `REDIRECTOR__HASHIDS__MIN_LENGTH` | `hashids.min_length` | `6` | Minimalna długość hashid |

> **Tablice**: Elementy listy są indeksowane za pomocą `__0`, `__1`, `__2`, itp. Przy rotacji soli hashid ustaw `__0` dla nowej soli i `__1` dla starej.

##### Redis / Cache

| Zmienna środowiskowa | Ścieżka konfiguracji | Domyślnie | Opis |
|----------------------|----------------------|-----------|------|
| `REDIRECTOR__REDIS__URL` | `redis.url` | *wymagane* | URL połączenia z Redis |
| `REDIRECTOR__REDIS__CACHE_TTL_SECONDS` | `redis.cache_ttl_seconds` | `86400` | TTL cache (sekundy). `86400` = 24h |

##### Baza danych

| Zmienna środowiskowa | Ścieżka konfiguracji | Domyślnie | Opis |
|----------------------|----------------------|-----------|------|
| `REDIRECTOR__DATABASE__URL` | `database.url` | *wymagane* | URL połączenia z PostgreSQL |
| `REDIRECTOR__DATABASE__POOL__MAX_CONNECTIONS` | `database.pool.max_connections` | `3` | Rozmiar puli połączeń |
| `REDIRECTOR__DATABASE__POOL__CONNECT_TIMEOUT_SECONDS` | `database.pool.connect_timeout_seconds` | `3` | Limit czasu połączenia (sekundy) |
| `REDIRECTOR__DATABASE__RATE_LIMIT__MAX_REQUESTS_PER_SECOND` | `database.rate_limit.max_requests_per_second` | `50` | Maks. zapytań do bazy na sekundę |
| `REDIRECTOR__DATABASE__CIRCUIT_BREAKER__FAILURE_THRESHOLD` | `database.circuit_breaker.failure_threshold` | `3` | Kolejne awarie przed otwarciem obwodu |
| `REDIRECTOR__DATABASE__CIRCUIT_BREAKER__RESET_TIMEOUT_SECONDS` | `database.circuit_breaker.reset_timeout_seconds` | `60` | Sekundy przed ponowną próbą (half-open) |
| `REDIRECTOR__DATABASE__QUERY__TABLE` | `database.query.table` | `dictionary.urls` | Nazwa tabeli dla wyszukiwania URL |
| `REDIRECTOR__DATABASE__QUERY__ID_COLUMN` | `database.query.id_column` | `id` | Nazwa kolumny dla numerycznego ID |
| `REDIRECTOR__DATABASE__QUERY__URL_COLUMN` | `database.query.url_column` | `name` | Nazwa kolumny dla docelowego URL |

##### Strona przejściowa

| Zmienna środowiskowa | Ścieżka konfiguracji | Domyślnie | Opis |
|----------------------|----------------------|-----------|------|
| `REDIRECTOR__INTERSTITIAL__DELAY_SECONDS` | `interstitial.delay_seconds` | `5` | Odliczanie przed przekierowaniem |

##### Metryki

| Zmienna środowiskowa | Ścieżka konfiguracji | Domyślnie | Opis |
|----------------------|----------------------|-----------|------|
| `REDIRECTOR__METRICS__BASIC_AUTH__USERNAME` | `metrics.basic_auth.username` | *wymagane* | Nazwa użytkownika dla endpointu `/metrics` |
| `REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD` | `metrics.basic_auth.password` | *wymagane* | Hasło dla endpointu `/metrics` |

##### Limitowanie prędkości (globalne)

| Zmienna środowiskowa | Ścieżka konfiguracji | Domyślnie | Opis |
|----------------------|----------------------|-----------|------|
| `REDIRECTOR__RATE_LIMIT__REQUESTS_PER_SECOND` | `rate_limit.requests_per_second` | `1000` | Maks. żądań na sekundę |
| `REDIRECTOR__RATE_LIMIT__BURST` | `rate_limit.burst` | `100` | Pojemność burst powyżej limitu RPS |

##### Panel administracyjny

| Zmienna środowiskowa | Ścieżka konfiguracji | Domyślnie | Opis |
|----------------------|----------------------|-----------|------|
| `REDIRECTOR__ADMIN__ENABLED` | `admin.enabled` | `false` | Włącz panel administracyjny |
| `REDIRECTOR__ADMIN__SESSION_SECRET` | `admin.session_secret` | `change-me-...` | Sekret podpisywania sesji (min. 32 znaki) |
| `REDIRECTOR__ADMIN__SESSION_TTL_HOURS` | `admin.session_ttl_hours` | `24` | Czas życia sesji w godzinach |

> **Uwaga**: Użytkownicy administratora (`admin.users`) z `username` i `password_hash` nie mogą być ustawiani przez zmienne środowiskowe ze względu na ich złożoną strukturę. Zdefiniuj ich w pliku konfiguracyjnym lub `CONFIG_BASE64`.

#### Przykłady według platformy wdrożeniowej

**Railway / Render / Fly.io** (PaaS z zarządzanymi bazami danych):

```bash
# Te są zazwyczaj ustawiane automatycznie przez platformę:
DATABASE_URL=postgres://user:pass@host:5432/db
REDIS_URL=redis://host:6379
PORT=3000

# Ustaw konfigurację przez base64:
CONFIG_BASE64=c2VydmVyOgogIGhvc3Q6IC...

# Lub nadpisz poszczególne wartości:
REDIRECTOR__HASHIDS__SALTS__0=my-secret-salt
REDIRECTOR__METRICS__BASIC_AUTH__USERNAME=prometheus
REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD=strong-password
REDIRECTOR__ADMIN__ENABLED=true
REDIRECTOR__ADMIN__SESSION_SECRET=random-32-byte-secret-for-sessions
```

**Docker Compose (pełny przykład ze wszystkimi nadpisaniami)**:

```yaml
services:
  redirector:
    image: ghcr.io/brilliant-almazov/redirector:latest
    ports:
      - "8080:8080"
    environment:
      # --- URL-e połączeń (styl PaaS) ---
      DATABASE_URL: "postgres://redirector:${DB_PASSWORD}@postgres:5432/redirector"
      REDIS_URL: "redis://redis:6379"

      # --- Plik konfiguracyjny ---
      CONFIG_BASE64: "${CONFIG_BASE64}"

      # --- Server ---
      REDIRECTOR__SERVER__HOST: "0.0.0.0"
      REDIRECTOR__SERVER__PORT: "8080"

      # --- Sole hashid ---
      REDIRECTOR__HASHIDS__SALTS__0: "${HASHID_SALT}"        # podstawowy salt
      REDIRECTOR__HASHIDS__SALTS__1: "${HASHID_SALT_OLD}"    # stary salt do migracji
      REDIRECTOR__HASHIDS__MIN_LENGTH: "6"

      # --- Cache Redis ---
      REDIRECTOR__REDIS__CACHE_TTL_SECONDS: "43200"          # 12 godzin

      # --- Pula bazy danych i odporność ---
      REDIRECTOR__DATABASE__POOL__MAX_CONNECTIONS: "5"
      REDIRECTOR__DATABASE__POOL__CONNECT_TIMEOUT_SECONDS: "5"
      REDIRECTOR__DATABASE__RATE_LIMIT__MAX_REQUESTS_PER_SECOND: "100"
      REDIRECTOR__DATABASE__CIRCUIT_BREAKER__FAILURE_THRESHOLD: "5"
      REDIRECTOR__DATABASE__CIRCUIT_BREAKER__RESET_TIMEOUT_SECONDS: "30"

      # --- Niestandardowe mapowanie tabeli ---
      REDIRECTOR__DATABASE__QUERY__TABLE: "public.short_urls"
      REDIRECTOR__DATABASE__QUERY__ID_COLUMN: "id"
      REDIRECTOR__DATABASE__QUERY__URL_COLUMN: "target_url"

      # --- Strona przejściowa ---
      REDIRECTOR__INTERSTITIAL__DELAY_SECONDS: "3"

      # --- Uwierzytelnianie metryk ---
      REDIRECTOR__METRICS__BASIC_AUTH__USERNAME: "prometheus"
      REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD: "${METRICS_PASSWORD}"

      # --- Globalne limitowanie prędkości ---
      REDIRECTOR__RATE_LIMIT__REQUESTS_PER_SECOND: "2000"
      REDIRECTOR__RATE_LIMIT__BURST: "200"

      # --- Panel administracyjny ---
      REDIRECTOR__ADMIN__ENABLED: "true"
      REDIRECTOR__ADMIN__SESSION_SECRET: "${SESSION_SECRET}"
      REDIRECTOR__ADMIN__SESSION_TTL_HOURS: "8"
    depends_on:
      - postgres
      - redis

  postgres:
    image: postgres:16-alpine
    environment:
      POSTGRES_USER: redirector
      POSTGRES_PASSWORD: ${DB_PASSWORD}
      POSTGRES_DB: redirector

  redis:
    image: redis:7-alpine
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
            - name: REDIRECTOR__HASHIDS__SALTS__0
              valueFrom:
                secretKeyRef:
                  name: redirector-secrets
                  key: hashid-salt
            - name: REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD
              valueFrom:
                secretKeyRef:
                  name: redirector-secrets
                  key: metrics-password
            - name: REDIRECTOR__ADMIN__SESSION_SECRET
              valueFrom:
                secretKeyRef:
                  name: redirector-secrets
                  key: session-secret
            - name: CONFIG_BASE64
              valueFrom:
                configMapKeyRef:
                  name: redirector-config
                  key: config-base64
```

**Zwykły Docker (pojedyncze polecenie)**:

```bash
docker run -p 8080:8080 \
  -e DATABASE_URL="postgres://user:pass@host:5432/db" \
  -e REDIS_URL="redis://host:6379" \
  -e REDIRECTOR__HASHIDS__SALTS__0="my-secret-salt" \
  -e REDIRECTOR__METRICS__BASIC_AUTH__USERNAME="prometheus" \
  -e REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD="strong-password" \
  -e REDIRECTOR__INTERSTITIAL__DELAY_SECONDS="3" \
  -e CONFIG_BASE64="$(cat config.yaml | base64)" \
  ghcr.io/brilliant-almazov/redirector:latest
```

**Minimalna konfiguracja (tylko zmienne środowiskowe, bez pliku konfiguracyjnego)**:

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

#### Rotacja soli przez zmienne środowiskowe

Podczas rotacji soli hashid usługa próbuje sole w kolejności — pierwsze dopasowanie wygrywa. Ustaw nową sól jako pierwszą, aby nowe linki jej używały, i zachowaj starą sól dla kompatybilności wstecznej:

**Opcja 1: Pojedyncza zmienna z separatorem przecinkowym** (zalecane):

```bash
# Przed rotacją
HASHIDS_SALTS=original-salt

# Po rotacji — nowa sól pierwsza, stara sól dla istniejących linków
HASHIDS_SALTS=new-salt,original-salt
```

**Opcja 2: Zmienne indeksowane**:

```bash
# Przed rotacją
REDIRECTOR__HASHIDS__SALTS__0=original-salt

# Po rotacji
REDIRECTOR__HASHIDS__SALTS__0=new-salt
REDIRECTOR__HASHIDS__SALTS__1=original-salt
```

> **Uwaga**: Jeśli ustawiono `REDIRECTOR__HASHIDS__SALTS__0`, `HASHIDS_SALTS` jest ignorowane.

#### Konfiguracja Base64

Dla środowisk, w których montowanie plików konfiguracyjnych nie jest praktyczne (PaaS, serverless, CI/CD), przekaż całą konfigurację jako ciąg zakodowany w base64:

```bash
# Encode
cat config.yaml | base64

# Dekodowanie (do weryfikacji)
echo "$CONFIG_BASE64" | base64 -d
```

`CONFIG_BASE64` ma priorytet nad `CONFIG_PATH`. Nadpisania zmiennych środowiskowych (`REDIRECTOR__*` i zmienne PaaS) są stosowane **na wierzchu** zdekodowanej konfiguracji.

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

## Analityka Zdarzeń

Opcjonalny potok publikacji zdarzeń do analizy przekierowań. Po włączeniu każde zdarzenie przekierowania jest publikowane do RabbitMQ i konsumowane przez oddzielny binarny plik, który zapisuje dane do PostgreSQL z bogatym wzbogaceniem.

> **Pełna dokumentacja**: [docs/EVENT_ANALYTICS.md](../EVENT_ANALYTICS.md)

### Funkcje

- **Publikacja bez czekania** — latencja przekierowania nie jest zależna od dostępności kolejki
- **Batching** — zdarzenia grupowane po rozmiarze (100) lub czasie (1 sekunda)
- **Parsowanie User-Agent** — przeglądarka, wersja, OS, typ urządzenia poprzez woothee
- **Wzbogacenie GeoIP** — kraj i miasto z adresu IP (MaxMind mmdb z hot-reload)
- **Deduplikacja odwołań** — deduplikacja oparta na MD5 dla refererów i user-agentów
- **Partycjonowanie miesięczne** — automatyczne tworzenie partycji dla `redirect_events`
- **Normalizacja domeny** — `WWW.Example.COM` → `example.com`

### Architektura

```
Redirect Handler
    │
    ├── try_send(RedirectEvent) ──► [tokio::mpsc channel]
    │   (non-blocking,                    │
    │    fire-and-forget)                 ▼
    │                              Background Task
    │                              (batch by size/time)
    │                                     │
    │                                     ▼
    │                                [RabbitMQ Queue]
    │                                     │
    │                                     ▼
    │                              Event Consumer
    │                              (separate binary/container)
    │                                     │
    │                                     ▼
    │                              [PostgreSQL Analytics]
    │                              (monthly partitioned)
```

### Szybki start

```bash
# Włącz w config.yaml
events:
  enabled: true
  rabbitmq:
    url: amqp://guest:guest@localhost:5672/%2f

# Lub poprzez zmienną środowiskową
REDIRECTOR__EVENTS__ENABLED=true
RABBITMQ_URL=amqp://guest:guest@localhost:5672/%2f

# Uruchom konsumenta
RABBITMQ_URL=amqp://... DATABASE_URL=postgres://... cargo run --bin event_consumer
```

### Docker Compose z zdarzeniami

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
      - GEOIP_DB_PATH=/data/GeoLite2-City.mmdb  # opcjonalnie
    depends_on: [rabbitmq, analytics-db]

  rabbitmq:
    image: rabbitmq:4-management-alpine
    ports: ["5672:5672", "15672:15672"]

  analytics-db:
    image: postgres:16-alpine
    environment:
      POSTGRES_DB: analytics
```

### Kluczowe decyzje projektowe

- **Nigdy nie blokuje przekierowań**: `try_send()` na ograniczonym kanale, porzuca zdarzenia jeśli jest pełny
- **Bezpieczne pod względem typów partie zdarzeń**: `EventBatch` to enum Rusta otagowany przez `event_type`
- **Identyfikatory partii Snowflake**: Niestandardowa epoka 2025-01-01, ~69 lat unikalnych ID
- **Wdzięczna degradacja**: Jeśli RabbitMQ jest niedostępny, przekierowania są kontynuowane; zdarzenia są usuwane z metrykami

## Metryki

Usługa udostępnia kompleksowe metryki Prometheus pod adresem `/metrics` (wymaga Basic Auth):

### Metryki usługi
```
redirector_up 1
redirector_build_info{version="0.1.0"} 1
redirector_uptime_seconds 3600.5
```

### Metryki żądań
```
redirect_requests_total 150000
not_found_requests_total 50
request_duration_seconds{quantile="0.5"} 0.040
request_duration_seconds{quantile="0.99"} 0.081
```

### Metryki cache
```
cache_hits_total 140000
cache_misses_total 10000
cache_get_duration_seconds{quantile="0.5"} 0.002
cache_set_duration_seconds{quantile="0.5"} 0.002
```

### Metryki bazy danych
```
db_queries_total 10000
db_hits_total 9950
db_misses_total 50
db_errors_total 0
db_query_duration_seconds{quantile="0.5"} 0.035
db_rate_limit_exceeded_total 0
circuit_breaker_rejections_total 0
```

### Limitowanie prędkości
```
rate_limit_exceeded_total 0
```

### Zdarzenia (gdy włączone)
```
events_published 50000
events_dropped 0
events_publish_errors 0
events_serialize_errors 0
rabbitmq_connected 1
```

## Licencja

Licencja MIT - szczegóły w [LICENSE](../LICENSE).

## Współpraca

Wkład jest mile widziany! Proszę:

1. Zrób fork repozytorium
2. Stwórz branch funkcjonalności
3. Wyślij Pull Request

Chroniony branch master wymaga przeglądu PR.
