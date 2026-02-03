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
