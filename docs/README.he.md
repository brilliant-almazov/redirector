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

#### הגדרת Base64

עבור סביבות שבהן אין אפשרות לעגן קבצי תצורה (למשל Railway, serverless):

```bash
# Encode
cat config.yaml | base64

# Run with base64 config
CONFIG_BASE64="c2VydmVyOgogIGhvc3Q6IC..." docker run ghcr.io/brilliant-almazov/redirector:latest
```

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

## רישיון

רישיון MIT - ראה [LICENSE](../LICENSE) לפרטים.

## תרומה

תרומות מתקבלות בברכה! אנא:

1. בצע Fork לרפוזיטורי
2. צור ענף feature
3. שלח Pull Request

ענף master מוגן ודורש בדיקת PR.
