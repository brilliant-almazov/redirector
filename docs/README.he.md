# redirector

> **שירות קיצור כתובות URL והפניות בעל ביצועים גבוהים** בנוי עם Rust, Axum, Redis ו-PostgreSQL. כולל דפי ביניים מאובטחים, לוח בקרה בזמן אמת וניטור ברמה ארגונית.

[English](../README.md) | [Русский](README.ru.md) | [中文](README.zh.md) | [हिंदी](README.hi.md) | [Español](README.es.md) | [Português](README.pt.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Polski](README.pl.md) | [Nederlands](README.nl.md) | [Italiano](README.it.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) | **עברית** | [Bahasa Indonesia](README.id.md) | [Tiếng Việt](README.vi.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![Docker Image Size](https://ghcr-badge.egpl.dev/brilliant-almazov/redirector/size)](https://github.com/brilliant-almazov/redirector/pkgs/container/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

**מילות מפתח**: מקצר כתובות, קיצור URL, שירות הפניות, שירות רשת Rust, מסגרת Axum, מטמון Redis, PostgreSQL, מדדי Prometheus, hashids, קישורים קצרים, דפי ביניים, הפניות מאובטחות, ביצועים גבוהים, מיקרו-שירות

שירות הפניות URL מאובטח עם דפי ביניים וקישורים קצרים מבוססי hashid. מושלם לכלים פנימיים, ניהול קישורים ארגוני ושירותי URL ממותגים.

### ביצועים

| תרחיש | RPS | השהייה ממוצעת | השהייה P99 |
|-------|-----|---------------|------------|
| 100% Cache Hit | **7,800+** | ~14ms | ~50ms |
| Cache Miss (10K URLs) | **2,300+** | ~44ms | ~81ms |

**תנאי בדיקה**: wrk -t4 -c100 -d30s, PostgreSQL 15, Dragonfly (Redis), macOS M1 (Docker)

> ⚠️ התוצאות מ-Docker על macOS עם תקורת וירטואליזציה. בריצה native על Linux צפוי להיות **3-5 פעמים מהיר יותר**.

## הבעיה

שיתוף כתובות URL ארוכות אינו נוח. מקצרי כתובות קיימים אך לרוב מפנים מיד, מה שעלול להוות סיכון אבטחה. משתמשים צריכים לראות לאן הם הולכים לפני ההפניה.

**redirector** מספק הפניות בטוחות עם:
- דף ביניים המציג את ה-URL היעד לפני ההפניה
- טיימר ספירה לאחור למודעות המשתמש
- דפים יפים וממותגים

## תכונות

- 🔗 **כתובות Hashid** - מזהים קצרים, ייחודיים ולא רציפים (לדוגמה, `/r/abc123`)
- ⏱️ **דף ביניים** - טיימר ספירה לאחור מציג URL יעד לפני הפניה
- ⚡ **מטמון Redis** - חיפושים מהירים עם TTL הניתן להגדרה
- 🛡️ **Circuit breaker** - הגנה על מסד הנתונים מפני כשלים מדורגים
- 🚦 **הגבלת קצב** - הגבלות קצב גלובליות וברמת מסד הנתונים
- 📊 **מדדי Prometheus** - תצפיתנות מלאה עם הגנת Basic Auth
- 🎨 **דפים יפים** - דפי 404 ודף ראשי נקיים עם 4 ערכות נושא
- 🔑 **מלחים מרובים** - תמיכה בסיבוב מלח hashid להעברה
- 📱 **לוח בקרה** - ניטור מדדים בזמן אמת עם SSE
- 📤 **ניתוח אירועים** - פרסום אירועים אופציונלי ל-RabbitMQ עם צרכן PostgreSQL

## צילומי מסך

| בהיר | כהה | אפור | חם |
|------|-----|------|-----|
| ![לוח בקרה בהיר](screenshots/dashboard-light.png) | ![לוח בקרה כהה](screenshots/dashboard-dark.png) | ![לוח בקרה אפור](screenshots/dashboard-gray.png) | ![לוח בקרה חם](screenshots/dashboard-warm.png) |
| ![התחברות בהיר](screenshots/login-light.png) | ![התחברות כהה](screenshots/login-dark.png) | ![התחברות אפור](screenshots/login-gray.png) | ![התחברות חם](screenshots/login-warm.png) |
| ![Index Light](screenshots/index-light.png) | ![Index Dark](screenshots/index-dark.png) | ![Index Gray](screenshots/index-gray.png) | ![Index Warm](screenshots/index-warm.png) |
| ![Interstitial Light](screenshots/interstitial-light.png) | ![Interstitial Dark](screenshots/interstitial-dark.png) | ![Interstitial Gray](screenshots/interstitial-gray.png) | ![Interstitial Warm](screenshots/interstitial-warm.png) |
| ![404 Light](screenshots/404-light.png) | ![404 Dark](screenshots/404-dark.png) | ![404 Gray](screenshots/404-gray.png) | ![404 Warm](screenshots/404-warm.png) |

### חלון בדיקת עומס

| בהיר | כהה | אפור | חם |
|------|-----|------|-----|
| ![חלון בהיר](screenshots/dashboard-modal-light.png) | ![חלון כהה](screenshots/dashboard-modal-dark.png) | ![חלון אפור](screenshots/dashboard-modal-gray.png) | ![חלון חם](screenshots/dashboard-modal-warm.png) |

## מחסנית טכנולוגית

- **שפה**: Rust (אסינכרוני עם Tokio)
- **מסגרת רשת**: Axum
- **מטמון**: תואם Redis (Redis, Dragonfly, Valkey, KeyDB ועוד)
- **מסד נתונים**: PostgreSQL (שכבת אחסון מופשטת)
- **מדדים**: Prometheus + metrics-rs
- **תור הודעות**: RabbitMQ (אופציונלי, לניתוח אירועים)
- **גיבוב סיסמאות**: Argon2

> **הערה**: שכבות האחסון והמטמון מופשטות וניתנות להחלפה בכל מקור נתונים תואם. כעת בפיתוח פעיל.

## התחלה מהירה

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

## הגדרות

צרו `config.yaml`:

```yaml
server:
  host: "0.0.0.0"
  port: 8080

hashids:
  salts:
    - ${HASHID_SALT}          # מלח ראשי
    - ${HASHID_SALT_OLD}      # אופציונלי: מלח ישן להעברה
  min_length: 6

redis:
  url: ${REDIS_URL}
  cache_ttl_seconds: 86400    # 24 שעות

database:
  url: ${DATABASE_URL}
  pool:
    max_connections: 5
    connect_timeout_seconds: 3
  rate_limit:
    max_requests_per_second: 50
  circuit_breaker:
    failure_threshold: 3
    reset_timeout_seconds: 60
  query:
    table: "dictionary.urls"    # שם הטבלה שלך
    id_column: "id"             # עמודת ID
    url_column: "name"          # עמודת URL

interstitial:
  delay_seconds: 5            # ספירה לאחור לפני הפניה

metrics:
  basic_auth:
    username: prometheus
    password: ${METRICS_PASSWORD}

rate_limit:
  requests_per_second: 1000
  burst: 100
```

### אפשרויות הגדרה

#### שרת

| אפשרות | ברירת מחדל | תיאור |
|---------|------------|-------|
| `host` | `0.0.0.0` | כתובת חיבור |
| `port` | `8080` | פורט HTTP |

#### Hashids

| אפשרות | ברירת מחדל | תיאור |
|---------|------------|-------|
| `salts` | *חובה* | רשימת מלחי hashid (ראשון = ראשי) |
| `min_length` | `6` | אורך מינימלי של hashid |

#### Redis

| אפשרות | ברירת מחדל | תיאור |
|---------|------------|-------|
| `url` | *חובה* | URL חיבור Redis |
| `cache_ttl_seconds` | `86400` | TTL מטמון בשניות |

#### מסד נתונים

| אפשרות | ברירת מחדל | תיאור |
|---------|------------|-------|
| `url` | *חובה* | URL חיבור PostgreSQL |
| `pool.max_connections` | `3` | גודל מאגר חיבורים |
| `pool.connect_timeout_seconds` | `3` | זמן קצוב לחיבור |
| `rate_limit.max_requests_per_second` | `50` | הגבלת קצב מסד נתונים |
| `circuit_breaker.failure_threshold` | `3` | כשלונות לפני פתיחה |
| `circuit_breaker.reset_timeout_seconds` | `60` | זמן קצוב לאיפוס מפסק |

#### הגבלת קצב (גלובלית)

| אפשרות | ברירת מחדל | תיאור |
|---------|------------|-------|
| `requests_per_second` | `1000` | הגבלת קצב גלובלית |
| `burst` | `100` | קיבולת פרץ |

### משתני סביבה

ישנן **שלוש דרכים** להגדרת השירות, לפי סדר עדיפות (הגבוה ביותר ראשון):

| עדיפות | שיטה | מקרה שימוש |
|---------|-------|------------|
| 1 | משתני `REDIRECTOR__*` | דריסת ערכים בודדים |
| 2 | משתני PaaS סטנדרטיים (`DATABASE_URL` וכו') | פלטפורמות PaaS (Railway, Heroku, Render) |
| 3 | קובץ הגדרות (`config.yaml` או `CONFIG_BASE64`) | הגדרות בסיס |

#### משתנים מיוחדים

| משתנה | ברירת מחדל | תיאור |
|-------|------------|-------|
| `CONFIG_PATH` | `config.yaml` | נתיב לקובץ הגדרות YAML |
| `CONFIG_BASE64` | — | הגדרות YAML מקודדות ב-Base64 (עדיפות על `CONFIG_PATH`) |

#### משתני סביבה סטנדרטיים של PaaS

אלה מזוהים ומיושמים אוטומטית. רוב פלטפורמות PaaS מגדירות אותם עבורך:

| משתנה | נתיב בהגדרות | דוגמה |
|-------|-------------|-------|
| `DATABASE_URL` | `database.url` | `postgres://user:pass@host:5432/db` |
| `REDIS_URL` | `redis.url` | `redis://host:6379` |
| `PORT` | `server.port` | `3000` |

> **כלל עדיפות**: אם גם `DATABASE_URL` וגם `REDIRECTOR__DATABASE__URL` מוגדרים, הגרסה עם קידומת `REDIRECTOR__` מנצחת.

#### משתנים עם קידומת (`REDIRECTOR__*`)

כל ערך הגדרה ניתן לדריסה באמצעות קידומת `REDIRECTOR__` עם `__` (קו תחתון כפול) כמפריד קינון:

```
נתיב הגדרות YAML           →  משתנה סביבה
─────────────────────────────────────────────────────
server.port               →  REDIRECTOR__SERVER__PORT
server.host               →  REDIRECTOR__SERVER__HOST
database.url              →  REDIRECTOR__DATABASE__URL
database.pool.max_connections → REDIRECTOR__DATABASE__POOL__MAX_CONNECTIONS
redis.url                 →  REDIRECTOR__REDIS__URL
redis.cache_ttl_seconds   →  REDIRECTOR__REDIS__CACHE_TTL_SECONDS
interstitial.delay_seconds → REDIRECTOR__INTERSTITIAL__DELAY_SECONDS
metrics.basic_auth.username → REDIRECTOR__METRICS__BASIC_AUTH__USERNAME
metrics.basic_auth.password → REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD
rate_limit.requests_per_second → REDIRECTOR__RATE_LIMIT__REQUESTS_PER_SECOND
rate_limit.burst          →  REDIRECTOR__RATE_LIMIT__BURST
admin.enabled             →  REDIRECTOR__ADMIN__ENABLED
admin.session_ttl_hours   →  REDIRECTOR__ADMIN__SESSION_TTL_HOURS
```

#### דוגמאות לפי פלטפורמת פריסה

**Railway / Render / Fly.io** (PaaS עם מסדי נתונים מנוהלים):

```bash
# אלה מוגדרים בדרך כלל אוטומטית על ידי הפלטפורמה:
DATABASE_URL=postgres://user:pass@host:5432/db
REDIS_URL=redis://host:6379
PORT=3000

# הגדר את ההגדרות שלך דרך base64:
CONFIG_BASE64=c2VydmVyOgogIGhvc3Q6IC...

# או דרוס ערכים בודדים:
REDIRECTOR__HASHIDS__SALTS__0=my-secret-salt
REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD=strong-password
```

**Docker / Docker Compose**:

```yaml
services:
  redirector:
    image: ghcr.io/brilliant-almazov/redirector:latest
    ports:
      - "8080:8080"
    environment:
      DATABASE_URL: "postgres://user:pass@postgres:5432/redirector"
      REDIS_URL: "redis://redis:6379"
      CONFIG_BASE64: "${CONFIG_BASE64}"
      # או דרוס ערכים בודדים מעל קובץ ההגדרות:
      REDIRECTOR__RATE_LIMIT__REQUESTS_PER_SECOND: "2000"
      REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD: "${METRICS_PASSWORD}"
    volumes:
      - ./config.yaml:/app/config.yaml  # אופציונלי עם CONFIG_BASE64
    depends_on:
      - postgres
      - redis
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
            - name: REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD
              valueFrom:
                secretKeyRef:
                  name: redirector-secrets
                  key: metrics-password
            - name: CONFIG_BASE64
              valueFrom:
                configMapKeyRef:
                  name: redirector-config
                  key: config-base64
```

**Docker רגיל**:

```bash
docker run -p 8080:8080 \
  -e DATABASE_URL="postgres://user:pass@host:5432/db" \
  -e REDIS_URL="redis://host:6379" \
  -e CONFIG_BASE64="$(cat config.yaml | base64)" \
  ghcr.io/brilliant-almazov/redirector:latest
```

**הגדרה מינימלית (רק משתני סביבה, ללא קובץ הגדרות)**:

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

#### הגדרת Base64

עבור סביבות שבהן עיגון קבצי הגדרות אינו מעשי (PaaS, serverless, CI/CD), העבירו את כל ההגדרות כמחרוזת מקודדת ב-Base64:

```bash
# קידוד
cat config.yaml | base64

# פענוח (לאימות)
echo "$CONFIG_BASE64" | base64 -d
```

`CONFIG_BASE64` מקבל עדיפות על `CONFIG_PATH`. דריסות משתני סביבה (`REDIRECTOR__*` ומשתני PaaS) מיושמות **מעל** ההגדרות המפוענחות.

## איך זה עובד

1. המשתמש מבקר ב-`/r/{hashid}` (לדוגמה, `/r/abc123`)
2. השירות מפענח את ה-hashid למזהה מספרי
3. בודק את מטמון Redis עבור URL
4. בהחמצת מטמון, שואל את PostgreSQL
5. שומר את התוצאה במטמון Redis
6. מציג דף ביניים עם ספירה לאחור
7. לאחר הספירה, מפנה ל-URL היעד

## נקודות קצה

| נקודת קצה | אימות | תיאור |
|-----------|-------|-------|
| `GET /` | לא | דף ראשי |
| `GET /r/{hashid}` | לא | הפניה עם דף ביניים |
| `GET /d/{hashid}` | לא | הפניית דמו (בדיקת עומס סינתטית) |
| `GET /health` | לא | בדיקת תקינות |
| `GET /metrics` | Basic | מדדי Prometheus |
| `GET /admin` | Session | כניסה ללוח בקרה |
| `GET /admin/dashboard` | Session | לוח בקרה |

## לוח בקרה

השירות כולל לוח בקרה אופציונלי לניטור מדדים בזמן אמת.

### הגדרה

1. **צור גיבוב סיסמה:**

```bash
cargo run --bin hash_password
# הזן סיסמה או:
cargo run --bin hash_password -- "your-password"
```

2. **הוסף ל-config.yaml:**

```yaml
admin:
  enabled: true
  session_ttl_hours: 24
  users:
    - username: admin
      password_hash: "$argon2id$v=19$m=19456,t=2,p=1$..."  # משלב 1
```

3. **גש ללוח הבקרה:**

פתח `http://localhost:8080/admin` והתחבר עם הפרטים שלך.

### תכונות

- גרפי RPS והשהייה בזמן אמת
- מדדי מערכת (CPU, זיכרון, זמן פעילות)
- ניטור שיעור פגיעות מטמון
- רשימת הפניות אחרונות
- סימולציית עומס לבדיקות
- שלוש ערכות נושא: בהיר, כהה, חם

## ניתוח אירועים

קו ייצור אופציונלי לפרסום אירועים לאנליטיקה של הפניות. כאשר מופעל, כל אירוע הפניה מפורסם ל-RabbitMQ ונצרך על ידי בינארי נפרד הכותב ל-PostgreSQL עם העשרה עשירה.

> **תיעוד מלא**: [docs/EVENT_ANALYTICS.md](../docs/EVENT_ANALYTICS.md)

### תכונות

- **פרסום ללא השהיה** — השהיית ההפניה אינה מושפעת מהזמינות של התור
- **עיבוד בקבוצות** — אירועים מקובצים לפי גודל (100) או זמן (שנייה אחת)
- **ניתוח User-Agent** — דפדפן, גרסה, מערכת הפעלה, סוג התקן דרך woothee
- **העשרת GeoIP** — מדינה ועיר מ-IP (MaxMind mmdb עם עדכון דינמי)
- **ביטול שכפול הפניות** — ביטול כפילויות מבוסס MD5 לפניות ו-User Agents
- **חלוקה חודשית** — יצירה אוטומטית של מחיצות עבור `redirect_events`
- **תקנון תחום** — `WWW.Example.COM` → `example.com`

### ארכיטקטורה

```
מטפל הפניות
    │
    ├── try_send(RedirectEvent) ──► [tokio::mpsc channel]
    │   (אי-חוסם,                        │
    │    ללא השהיה)                       ▼
    │                              משימה רקע
    │                              (קיבוץ לפי גודל/זמן)
    │                                     │
    │                                     ▼
    │                                [תור RabbitMQ]
    │                                     │
    │                                     ▼
    │                              צרכן אירועים
    │                              (בינארי/מיכל נפרד)
    │                                     │
    │                                     ▼
    │                              [אנליטיקה PostgreSQL]
    │                              (מחיצה חודשית)
```

### התחלה מהירה

```bash
# הפעל ב-config.yaml
events:
  enabled: true
  rabbitmq:
    url: amqp://guest:guest@localhost:5672/%2f

# או דרך משתני סביבה
REDIRECTOR__EVENTS__ENABLED=true
RABBITMQ_URL=amqp://guest:guest@localhost:5672/%2f

# הפעל את הצרכן
RABBITMQ_URL=amqp://... DATABASE_URL=postgres://... cargo run --bin event_consumer
```

### Docker Compose עם אירועים

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
      - GEOIP_DB_PATH=/data/GeoLite2-City.mmdb  # אופציונלי
    depends_on: [rabbitmq, analytics-db]

  rabbitmq:
    image: rabbitmq:4-management-alpine
    ports: ["5672:5672", "15672:15672"]

  analytics-db:
    image: postgres:16-alpine
    environment:
      POSTGRES_DB: analytics
```

### החלטות עיצוב עיקריות

- **לעולם אל תחסום הפניות**: `try_send()` על ערוץ מוגבל, מפילה אירועים אם מלא
- **קבוצות אירועים בטיפוס בטוח**: `EventBatch` הוא enum של Rust מתויג לפי `event_type`
- **מזהי קבוצה Snowflake**: Epoch מותאם אישית 2025-01-01, ~69 שנים של מזהים ייחודיים
- **הדרדרות כושר** — אם RabbitMQ כבוי, הפניות ממשיכות; אירועים נופלים עם מדדים

### מדדי אירועים

```
events_published 50000
events_dropped 0
events_publish_errors 0
events_serialize_errors 0
rabbitmq_connected 1
```

## רישיון

רישיון MIT - ראה [LICENSE](../LICENSE) לפרטים.

## תרומה

תרומות מתקבלות בברכה! אנא:

1. בצע Fork לרפוזיטורי
2. צור ענף feature
3. שלח Pull Request

ענף master מוגן ודורש בדיקת PR.
