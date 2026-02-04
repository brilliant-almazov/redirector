# redirector

> **Високопродуктивний сервіс скорочення посилань та редиректів** на Rust, Axum, Redis та PostgreSQL. Безпечні проміжні сторінки, панель моніторингу в реальному часі та спостережуваність корпоративного рівня.

[English](../README.md) | [Русский](README.ru.md) | [中文](README.zh.md) | [हिंदी](README.hi.md) | [Español](README.es.md) | [Português](README.pt.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Polski](README.pl.md) | [Nederlands](README.nl.md) | [Italiano](README.it.md) | [Türkçe](README.tr.md) | **Українська** | [עברית](README.he.md) | [Bahasa Indonesia](README.id.md) | [Tiếng Việt](README.vi.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![Docker Image Size](https://ghcr-badge.egpl.dev/brilliant-almazov/redirector/size)](https://github.com/brilliant-almazov/redirector/pkgs/container/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

**Ключові слова**: скорочувач посилань, URL shortener, сервіс редиректів, Rust веб-сервіс, Axum фреймворк, Redis кеш, PostgreSQL, Prometheus метрики, hashids, короткі посилання, проміжні сторінки, безпечні редиректи, висока продуктивність, мікросервіс

Безпечний сервіс редиректів з проміжними сторінками та короткими посиланнями на основі hashid. Ідеально підходить для внутрішніх інструментів, корпоративного управління посиланнями та брендованих URL-сервісів.

### Продуктивність

| Сценарій | RPS | Сер. затримка | P99 затримка |
|----------|-----|---------------|--------------|
| 100% Cache Hit | **7 800+** | ~14ms | ~50ms |
| Cache Miss (10K URLs) | **2 300+** | ~44ms | ~81ms |

**Умови тесту**: wrk -t4 -c100 -d30s, PostgreSQL 15, Dragonfly (Redis), macOS M1 (Docker)

> ⚠️ Результати отримані в Docker на macOS з overhead віртуалізації. На native Linux очікується **в 3-5 разів швидше**.

## Проблема

Ділитися довгими URL незручно. Скорочувачі посилань існують, але часто перенаправляють одразу, що може бути ризиком безпеки. Користувачі повинні бачити, куди вони йдуть, перед перенаправленням.

**redirector** забезпечує безпечні редиректи:
- Проміжна сторінка показує цільовий URL перед редиректом
- Зворотний відлік для обізнаності користувача
- Красиві, брендовані сторінки

## Можливості

- 🔗 **Hashid URL** - Короткі, унікальні, непослідовні ID (наприклад, `/r/abc123`)
- ⏱️ **Проміжна сторінка** - Зворотний відлік показує цільовий URL перед редиректом
- ⚡ **Кешування Redis** - Швидкі запити з налаштовуваним TTL
- 🛡️ **Circuit breaker** - Захист БД від каскадних збоїв
- 🚦 **Rate limiting** - Глобальний та на рівні БД
- 📊 **Prometheus метрики** - Повна спостережуваність з Basic Auth захистом
- 🎨 **Красиві сторінки** - Чисті сторінки 404 та індексу з 4 темами
- 🔑 **Декілька солей** - Підтримка ротації солі hashid для міграції
- 📱 **Адмін-панель** - Моніторинг метрик в реальному часі через SSE

## Скріншоти

| Світла | Темна | Сіра | Тепла |
|--------|-------|------|-------|
| ![Дашборд світла](screenshots/dashboard-light.png) | ![Дашборд темна](screenshots/dashboard-dark.png) | ![Дашборд сіра](screenshots/dashboard-gray.png) | ![Дашборд тепла](screenshots/dashboard-warm.png) |
| ![Логін світла](screenshots/login-light.png) | ![Логін темна](screenshots/login-dark.png) | ![Логін сіра](screenshots/login-gray.png) | ![Логін тепла](screenshots/login-warm.png) |
| ![Index Light](screenshots/index-light.png) | ![Index Dark](screenshots/index-dark.png) | ![Index Gray](screenshots/index-gray.png) | ![Index Warm](screenshots/index-warm.png) |
| ![Interstitial Light](screenshots/interstitial-light.png) | ![Interstitial Dark](screenshots/interstitial-dark.png) | ![Interstitial Gray](screenshots/interstitial-gray.png) | ![Interstitial Warm](screenshots/interstitial-warm.png) |
| ![404 Light](screenshots/404-light.png) | ![404 Dark](screenshots/404-dark.png) | ![404 Gray](screenshots/404-gray.png) | ![404 Warm](screenshots/404-warm.png) |

### Модалка навантажувального тестування

| Світла | Темна | Сіра | Тепла |
|--------|-------|------|-------|
| ![Модалка світла](screenshots/dashboard-modal-light.png) | ![Модалка темна](screenshots/dashboard-modal-dark.png) | ![Модалка сіра](screenshots/dashboard-modal-gray.png) | ![Модалка тепла](screenshots/dashboard-modal-warm.png) |

## Стек технологій

- **Мова**: Rust (асинхронний з Tokio)
- **Веб-фреймворк**: Axum
- **Кеш**: Redis-compatible (Redis, Dragonfly, Valkey, KeyDB та ін.)
- **База даних**: PostgreSQL (абстрактний шар зберігання)
- **Метрики**: Prometheus + metrics-rs
- **Хешування паролів**: Argon2

> **Примітка**: Шари зберігання та кешування абстраговані і можуть бути замінені будь-яким сумісним джерелом даних. Проєкт в активній розробці.

## Швидкий старт

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

### Змінні оточення

Існує **три способи** налаштування сервісу, перелічені за пріоритетом (від найвищого):

| Пріоритет | Метод | Випадок використання |
|-----------|-------|----------------------|
| 1 | Змінні оточення `REDIRECTOR__*` | Перевизначення окремих значень |
| 2 | Стандартні PaaS змінні (`DATABASE_URL` тощо) | PaaS платформи (Railway, Heroku, Render) |
| 3 | Файл конфігурації (`config.yaml` або `CONFIG_BASE64`) | Базова конфігурація |

#### Спеціальні змінні

| Змінна | За замовчуванням | Опис |
|--------|-----------------|------|
| `CONFIG_PATH` | `config.yaml` | Шлях до YAML файлу конфігурації |
| `CONFIG_BASE64` | — | YAML конфігурація в кодуванні Base64 (має пріоритет над `CONFIG_PATH`) |

#### Стандартні PaaS змінні оточення

Вони автоматично розпізнаються та застосовуються. Більшість PaaS платформ встановлює їх за вас:

| Змінна | Шлях конфігурації | Приклад |
|--------|-------------------|---------|
| `DATABASE_URL` | `database.url` | `postgres://user:pass@host:5432/db` |
| `REDIS_URL` | `redis.url` | `redis://host:6379` |
| `PORT` | `server.port` | `3000` |
| `HASHIDS_SALTS` | `hashids.salts` | `new-salt,old-salt` (через кому) |

> **Правило пріоритету**: Якщо встановлені обидва `DATABASE_URL` та `REDIRECTOR__DATABASE__URL`, версія з префіксом `REDIRECTOR__` має перевагу. Аналогічно, `REDIRECTOR__HASHIDS__SALTS__0` має пріоритет над `HASHIDS_SALTS`.

#### Змінні з префіксом (`REDIRECTOR__*`)

Будь-яке значення конфігурації може бути перевизначене за допомогою префікса `REDIRECTOR__` з `__` (подвійне підкреслення) як роздільником вкладеності. Нижче наведено **повний довідник** усіх змінних:

##### Server

| Змінна оточення | Шлях конфігурації | За замовчуванням | Опис |
|-----------------|-------------------|-----------------|------|
| `REDIRECTOR__SERVER__HOST` | `server.host` | `0.0.0.0` | Адреса прив'язки |
| `REDIRECTOR__SERVER__PORT` | `server.port` | `8080` | HTTP порт |

##### Hashids

| Змінна оточення | Шлях конфігурації | За замовчуванням | Опис |
|-----------------|-------------------|-----------------|------|
| `REDIRECTOR__HASHIDS__SALTS__0` | `hashids.salts[0]` | *обов'язково* | Основна сіль hashid |
| `REDIRECTOR__HASHIDS__SALTS__1` | `hashids.salts[1]` | — | Стара сіль (для міграції) |
| `REDIRECTOR__HASHIDS__MIN_LENGTH` | `hashids.min_length` | `6` | Мінімальна довжина hashid |

> **Масиви**: Елементи списку індексуються через `__0`, `__1`, `__2` тощо. Для ротації солі hashid встановіть `__0` для нової солі та `__1` для старої.

##### Redis / Кеш

| Змінна оточення | Шлях конфігурації | За замовчуванням | Опис |
|-----------------|-------------------|-----------------|------|
| `REDIRECTOR__REDIS__URL` | `redis.url` | *обов'язково* | URL підключення Redis |
| `REDIRECTOR__REDIS__CACHE_TTL_SECONDS` | `redis.cache_ttl_seconds` | `86400` | TTL кешу (секунди). `86400` = 24 години |

##### База даних

| Змінна оточення | Шлях конфігурації | За замовчуванням | Опис |
|-----------------|-------------------|-----------------|------|
| `REDIRECTOR__DATABASE__URL` | `database.url` | *обов'язково* | URL підключення PostgreSQL |
| `REDIRECTOR__DATABASE__POOL__MAX_CONNECTIONS` | `database.pool.max_connections` | `3` | Розмір пулу з'єднань |
| `REDIRECTOR__DATABASE__POOL__CONNECT_TIMEOUT_SECONDS` | `database.pool.connect_timeout_seconds` | `3` | Таймаут з'єднання (секунди) |
| `REDIRECTOR__DATABASE__RATE_LIMIT__MAX_REQUESTS_PER_SECOND` | `database.rate_limit.max_requests_per_second` | `50` | Макс. запитів до БД на секунду |
| `REDIRECTOR__DATABASE__CIRCUIT_BREAKER__FAILURE_THRESHOLD` | `database.circuit_breaker.failure_threshold` | `3` | Послідовних збоїв до відкриття контуру |
| `REDIRECTOR__DATABASE__CIRCUIT_BREAKER__RESET_TIMEOUT_SECONDS` | `database.circuit_breaker.reset_timeout_seconds` | `60` | Секунд до напіввідкритої спроби |
| `REDIRECTOR__DATABASE__QUERY__TABLE` | `database.query.table` | `dictionary.urls` | Назва таблиці для пошуку URL |
| `REDIRECTOR__DATABASE__QUERY__ID_COLUMN` | `database.query.id_column` | `id` | Назва стовпця для числового ID |
| `REDIRECTOR__DATABASE__QUERY__URL_COLUMN` | `database.query.url_column` | `name` | Назва стовпця для цільового URL |

##### Проміжна сторінка

| Змінна оточення | Шлях конфігурації | За замовчуванням | Опис |
|-----------------|-------------------|-----------------|------|
| `REDIRECTOR__INTERSTITIAL__DELAY_SECONDS` | `interstitial.delay_seconds` | `5` | Зворотний відлік перед редиректом |

##### Метрики

| Змінна оточення | Шлях конфігурації | За замовчуванням | Опис |
|-----------------|-------------------|-----------------|------|
| `REDIRECTOR__METRICS__BASIC_AUTH__USERNAME` | `metrics.basic_auth.username` | *обов'язково* | Ім'я користувача для ендпоінта `/metrics` |
| `REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD` | `metrics.basic_auth.password` | *обов'язково* | Пароль для ендпоінта `/metrics` |

##### Rate Limiting (Глобальний)

| Змінна оточення | Шлях конфігурації | За замовчуванням | Опис |
|-----------------|-------------------|-----------------|------|
| `REDIRECTOR__RATE_LIMIT__REQUESTS_PER_SECOND` | `rate_limit.requests_per_second` | `1000` | Макс. запитів на секунду |
| `REDIRECTOR__RATE_LIMIT__BURST` | `rate_limit.burst` | `100` | Допустимий сплеск понад ліміт RPS |

##### Адмін-панель

| Змінна оточення | Шлях конфігурації | За замовчуванням | Опис |
|-----------------|-------------------|-----------------|------|
| `REDIRECTOR__ADMIN__ENABLED` | `admin.enabled` | `false` | Увімкнути адмін-панель |
| `REDIRECTOR__ADMIN__SESSION_SECRET` | `admin.session_secret` | `change-me-...` | Секрет підпису сесії (мін. 32 символи) |
| `REDIRECTOR__ADMIN__SESSION_TTL_HOURS` | `admin.session_ttl_hours` | `24` | Час життя сесії в годинах |

> **Примітка**: Адмін-користувачі (`admin.users`) з `username` та `password_hash` не можуть бути встановлені через змінні оточення через їх складну структуру. Визначте їх у файлі конфігурації або `CONFIG_BASE64`.

#### Приклади за платформами розгортання

**Railway / Render / Fly.io** (PaaS з керованими базами даних):

```bash
# Зазвичай встановлюються автоматично платформою:
DATABASE_URL=postgres://user:pass@host:5432/db
REDIS_URL=redis://host:6379
PORT=3000

# Задайте конфігурацію через base64:
CONFIG_BASE64=c2VydmVyOgogIGhvc3Q6IC...

# Або перевизначте окремі значення:
REDIRECTOR__HASHIDS__SALTS__0=my-secret-salt
REDIRECTOR__METRICS__BASIC_AUTH__USERNAME=prometheus
REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD=strong-password
REDIRECTOR__ADMIN__ENABLED=true
REDIRECTOR__ADMIN__SESSION_SECRET=random-32-byte-secret-for-sessions
```

**Docker Compose (повний приклад з усіма перевизначеннями)**:

```yaml
services:
  redirector:
    image: ghcr.io/brilliant-almazov/redirector:latest
    ports:
      - "8080:8080"
    environment:
      # --- URL підключень (PaaS-стиль) ---
      DATABASE_URL: "postgres://redirector:${DB_PASSWORD}@postgres:5432/redirector"
      REDIS_URL: "redis://redis:6379"

      # --- Файл конфігурації ---
      CONFIG_BASE64: "${CONFIG_BASE64}"

      # --- Сервер ---
      REDIRECTOR__SERVER__HOST: "0.0.0.0"
      REDIRECTOR__SERVER__PORT: "8080"

      # --- Солі hashid ---
      REDIRECTOR__HASHIDS__SALTS__0: "${HASHID_SALT}"        # основна сіль
      REDIRECTOR__HASHIDS__SALTS__1: "${HASHID_SALT_OLD}"    # стара сіль для міграції
      REDIRECTOR__HASHIDS__MIN_LENGTH: "6"

      # --- Кеш Redis ---
      REDIRECTOR__REDIS__CACHE_TTL_SECONDS: "43200"          # 12 годин

      # --- Пул БД та стійкість ---
      REDIRECTOR__DATABASE__POOL__MAX_CONNECTIONS: "5"
      REDIRECTOR__DATABASE__POOL__CONNECT_TIMEOUT_SECONDS: "5"
      REDIRECTOR__DATABASE__RATE_LIMIT__MAX_REQUESTS_PER_SECOND: "100"
      REDIRECTOR__DATABASE__CIRCUIT_BREAKER__FAILURE_THRESHOLD: "5"
      REDIRECTOR__DATABASE__CIRCUIT_BREAKER__RESET_TIMEOUT_SECONDS: "30"

      # --- Власне відображення таблиці ---
      REDIRECTOR__DATABASE__QUERY__TABLE: "public.short_urls"
      REDIRECTOR__DATABASE__QUERY__ID_COLUMN: "id"
      REDIRECTOR__DATABASE__QUERY__URL_COLUMN: "target_url"

      # --- Проміжна сторінка ---
      REDIRECTOR__INTERSTITIAL__DELAY_SECONDS: "3"

      # --- Автентифікація метрик ---
      REDIRECTOR__METRICS__BASIC_AUTH__USERNAME: "prometheus"
      REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD: "${METRICS_PASSWORD}"

      # --- Глобальний ліміт швидкості ---
      REDIRECTOR__RATE_LIMIT__REQUESTS_PER_SECOND: "2000"
      REDIRECTOR__RATE_LIMIT__BURST: "200"

      # --- Адмін-панель ---
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

**Звичайний Docker (одна команда)**:

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

**Мінімальне налаштування (тільки змінні оточення, без файлу конфігурації)**:

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

#### Ротація солі через змінні оточення

При ротації солей hashid сервіс пробує солі по порядку — перший збіг перемагає. Встановіть нову сіль першою, щоб нові посилання її використовували, та збережіть стару для зворотної сумісності:

**Варіант 1: Одна змінна з роздільником-комою** (рекомендовано):

```bash
# До ротації
HASHIDS_SALTS=original-salt

# Після ротації — нова сіль першою, стара для існуючих посилань
HASHIDS_SALTS=new-salt,original-salt
```

**Варіант 2: Індексовані змінні**:

```bash
# До ротації
REDIRECTOR__HASHIDS__SALTS__0=original-salt

# Після ротації
REDIRECTOR__HASHIDS__SALTS__0=new-salt
REDIRECTOR__HASHIDS__SALTS__1=original-salt
```

> **Примітка**: Якщо встановлено `REDIRECTOR__HASHIDS__SALTS__0`, `HASHIDS_SALTS` ігнорується.

#### Конфігурація Base64

Для середовищ, де монтування конфігураційних файлів непрактичне (PaaS, serverless, CI/CD), передайте всю конфігурацію як рядок у кодуванні base64:

```bash
# Encode
cat config.yaml | base64

# Decode (для перевірки)
echo "$CONFIG_BASE64" | base64 -d
```

`CONFIG_BASE64` має пріоритет над `CONFIG_PATH`. Перевизначення змінних оточення (`REDIRECTOR__*` та PaaS змінні) застосовуються **поверх** декодованої конфігурації.

## Як це працює

1. Користувач відвідує `/r/{hashid}` (наприклад, `/r/abc123`)
2. Сервіс декодує hashid в числовий ID
3. Перевіряє кеш Redis на наявність URL
4. При промаху кешу — запит до PostgreSQL
5. Кешує результат в Redis
6. Показує проміжну сторінку з відліком
7. Після відліку редиректить на цільовий URL

## Ендпоінти

| Ендпоінт | Авторизація | Опис |
|----------|-------------|------|
| `GET /` | Ні | Головна сторінка |
| `GET /r/{hashid}` | Ні | Редирект з проміжною сторінкою |
| `GET /d/{hashid}` | Ні | Демо-редирект (синтетичне навантажувальне тестування) |
| `GET /health` | Ні | Перевірка здоров'я |
| `GET /metrics` | Basic | Prometheus метрики |
| `GET /admin` | Session | Вхід в адмін-панель |
| `GET /admin/dashboard` | Session | Адмін-панель |

## Адмін-панель

Сервіс включає опціональну адмін-панель для моніторингу метрик в реальному часі.

### Налаштування

1. **Згенеруйте хеш пароля:**

```bash
cargo run --bin hash_password
# Введіть пароль або:
cargo run --bin hash_password -- "ваш-пароль"
```

2. **Додайте в config.yaml:**

```yaml
admin:
  enabled: true
  session_ttl_hours: 24
  users:
    - username: admin
      password_hash: "$argon2id$v=19$m=19456,t=2,p=1$..."  # з кроку 1
```

3. **Відкрийте панель:**

Перейдіть на `http://localhost:8080/admin` та увійдіть з вашими обліковими даними.

### Можливості

- Графіки RPS та затримки в реальному часі
- Системні метрики (CPU, пам'ять, uptime)
- Моніторинг cache hit rate
- Список останніх редиректів
- Симуляція навантаження для тестування
- Три теми: Світла, Темна, Тепла

## Ліцензія

MIT License - деталі в [LICENSE](../LICENSE).

## Участь у розробці

Внесок вітається! Будь ласка:

1. Форкніть репозиторій
2. Створіть feature-гілку
3. Надішліть Pull Request

Захищена master-гілка вимагає рев'ю PR.
