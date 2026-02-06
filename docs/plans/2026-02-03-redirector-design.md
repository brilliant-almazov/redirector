# Redirector — Дизайн-документ

## Обзор

Сервис безопасных редиректов. Принимает короткие ссылки (hashid), показывает interstitial страницу с таймером, редиректит на целевой URL. Скрывает основной домен от пользователя.

**Стек:** Rust, Axum, Askama, Redis, PostgreSQL

## API

| Путь | Метод | Описание |
|------|-------|----------|
| `/` | GET | Минимальная заглушка |
| `/{hashid}` | GET | Interstitial → редирект |
| `/metrics` | GET | Prometheus метрики (Basic Auth) |

## Флоу редиректа

```
GET /abc123
    ↓
Декодируем hashid (перебор солей сверху вниз)
    ↓
Ищем в Redis: url:{id}
    ↓ (miss)
SELECT из основной базы, кэшируем
    ↓
Показываем interstitial с таймером
    ↓
302 Redirect → target URL
```

## Конфигурация

```yaml
server:
  host: "0.0.0.0"
  port: 8080

hashids:
  salts:
    - "current_salt_2024"
    - "old_salt_2023"
  min_length: 6

redis:
  url: "redis://localhost:6379"
  cache_ttl_seconds: 86400

database:
  url: "postgres://user:pass@host:5432/db"
  pool:
    max_connections: 3
    connect_timeout_seconds: 3
  rate_limit:
    max_requests_per_second: 50
  circuit_breaker:
    failure_threshold: 3
    reset_timeout_seconds: 60

interstitial:
  delay_seconds: 5

metrics:
  basic_auth:
    username: "prometheus"
    password: "secret"

rate_limit:
  requests_per_second: 1000
  burst: 100
```

Переменные окружения переопределяют YAML: `SERVER_PORT`, `REDIS_URL`, `DATABASE_URL` и т.д.

## Схема базы данных (основная)

```sql
-- Домены
CREATE TABLE dictionary.domains (
  id BIGINT PRIMARY KEY,
  hash CHAR(32) UNIQUE NOT NULL,
  name VARCHAR(4096) NOT NULL,
  created_by BIGINT NOT NULL,
  created_at TIMESTAMP NOT NULL
);

-- URL-ы
CREATE TABLE dictionary.urls (
  id BIGINT PRIMARY KEY,
  domain_id BIGINT NOT NULL REFERENCES dictionary.domains(id),
  hash CHAR(32) UNIQUE NOT NULL,
  name VARCHAR(4096) NOT NULL,
  created_by BIGINT NOT NULL,
  created_at TIMESTAMP NOT NULL
);
```

Полный URL = `domains.name || urls.name`

## Структура проекта

```
redirector/
├── Cargo.toml
├── Cargo.lock
├── config.yaml.example
├── README.md
├── LICENSE
├── CONTRIBUTING.md
├── Dockerfile
├── docker-compose.yaml
│
├── .github/
│   └── workflows/
│       ├── ci.yaml            # build, test, lint
│       └── coverage.yaml      # code coverage
│
├── docs/
│   └── plans/
│       └── 2026-02-03-redirector-design.md
│
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── config.rs
│   ├── error.rs
│   │
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── redirect.rs        # GET /{hashid}
│   │   ├── index.rs           # GET /
│   │   └── metrics.rs         # GET /metrics
│   │
│   ├── services/
│   │   ├── mod.rs
│   │   ├── hashid.rs          # декодирование
│   │   ├── cache.rs           # Redis
│   │   └── url_resolver.rs    # логика получения URL
│   │
│   ├── db/
│   │   ├── mod.rs
│   │   └── main_storage.rs    # основная база
│   │
│   └── middleware/
│       ├── mod.rs
│       ├── rate_limit.rs
│       └── basic_auth.rs
│
├── templates/
│   ├── index.html
│   ├── interstitial.html
│   └── not_found.html
│
└── tests/
    ├── integration/
    │   └── ...
    └── ...
```

## Страницы

### Interstitial
- Сообщение "Вы будете перенаправлены на {domain}"
- Обратный отсчёт (время из конфига)
- Кнопка "Перейти сейчас"
- Автоматический редирект после таймера

### 404
- Красивая страница "Ссылка не найдена"

### Главная (/)
- Минимальная заглушка

## План итераций

### Итерация 1 — MVP
- [ ] Инициализация Rust проекта (Cargo.toml)
- [ ] Axum + Askama setup
- [ ] Конфиг (YAML + env vars)
- [ ] Декодирование hashid (несколько солей)
- [ ] Redis кэш
- [ ] Подключение к основному Postgres
- [ ] Rate limit, circuit breaker для защиты основной базы
- [ ] Handlers: `/`, `/{hashid}`, `/metrics`
- [ ] Шаблоны: interstitial, 404, index
- [ ] Basic Auth для `/metrics`
- [ ] Тесты (unit + integration), цель 100% coverage
- [ ] GitHub Actions (CI: build, test, lint, coverage)
- [ ] Dockerfile, docker-compose
- [ ] README, LICENSE (MIT/Apache-2.0), CONTRIBUTING

### Итерация 2 — Свой Postgres
- [ ] Отдельный Postgres для сервиса
- [ ] API endpoint для приёма данных от основной системы (push)
- [ ] Соли в базе вместо конфига
- [ ] Hot reload солей (кэш в Redis)
- [ ] Админка для управления солями

### Итерация 3 — Аналитика
- [ ] Сбор статистики (referrer, clicks, timestamp)
- [ ] Отложенная запись (батчинг)
- [ ] UI для графиков

### Итерация 4 — Улучшения
- [ ] OpenTelemetry
- [ ] Whitelist доменов для мгновенного редиректа
- [ ] Hot reload конфига

## Нефункциональные требования

- Защита основной базы от паразитной нагрузки (маленький пул, rate limit, circuit breaker)
- Redis обрабатывает 99%+ запросов
- Compile-time шаблоны (Askama) для максимальной производительности
- Open source проект
