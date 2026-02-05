# redirector

> **उच्च-प्रदर्शन URL शॉर्टनर और रीडायरेक्ट सेवा** Rust, Axum, Redis और PostgreSQL के साथ बनाई गई। सुरक्षित इंटरस्टिशियल पेज, रियल-टाइम एडमिन डैशबोर्ड और एंटरप्राइज़-ग्रेड ऑब्ज़र्वेबिलिटी के साथ।

[English](../README.md) | [Русский](README.ru.md) | [中文](README.zh.md) | **हिंदी** | [Español](README.es.md) | [Português](README.pt.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Polski](README.pl.md) | [Nederlands](README.nl.md) | [Italiano](README.it.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) | [עברית](README.he.md) | [Bahasa Indonesia](README.id.md) | [Tiếng Việt](README.vi.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![Docker Image Size](https://ghcr-badge.egpl.dev/brilliant-almazov/redirector/size)](https://github.com/brilliant-almazov/redirector/pkgs/container/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

**कीवर्ड**: URL शॉर्टनर, लिंक शॉर्टनर, रीडायरेक्ट सेवा, Rust वेब सेवा, Axum फ्रेमवर्क, Redis कैश, PostgreSQL, Prometheus मेट्रिक्स, hashids, शॉर्ट लिंक, इंटरस्टिशियल पेज, सुरक्षित रीडायरेक्ट, उच्च प्रदर्शन, माइक्रोसर्विस

इंटरस्टिशियल पेज और hashid-आधारित शॉर्ट लिंक के साथ सुरक्षित URL रीडायरेक्ट सेवा। आंतरिक टूल्स, एंटरप्राइज़ लिंक प्रबंधन और ब्रांडेड शॉर्ट URL सेवाओं के लिए उपयुक्त।

### प्रदर्शन

| परिदृश्य | RPS | औसत विलंबता | P99 विलंबता |
|----------|-----|--------------|-------------|
| 100% कैश हिट | **7,800+** | ~14ms | ~50ms |
| कैश मिस (10K URLs) | **2,300+** | ~44ms | ~81ms |

**परीक्षण शर्तें**: wrk -t4 -c100 -d30s, PostgreSQL 15, Dragonfly (Redis), macOS M1 (Docker)

> ⚠️ परिणाम VM ओवरहेड के साथ macOS Docker से हैं। नेटिव Linux डिप्लॉयमेंट **3-5x तेज़** होने की उम्मीद है।

## समस्या

लंबे URL साझा करना असुविधाजनक है। URL शॉर्टनर मौजूद हैं लेकिन अक्सर तुरंत रीडायरेक्ट करते हैं, जो सुरक्षा जोखिम हो सकता है। उपयोगकर्ताओं को रीडायरेक्ट होने से पहले यह देखना चाहिए कि वे कहाँ जा रहे हैं।

**redirector** सुरक्षित रीडायरेक्ट प्रदान करता है:
- रीडायरेक्ट से पहले लक्ष्य URL दिखाने वाला इंटरस्टिशियल पेज
- उपयोगकर्ता जागरूकता के लिए काउंटडाउन टाइमर
- सुंदर, ब्रांडेड पेज

## विशेषताएं

- 🔗 **Hashid URLs** - छोटी, अद्वितीय, गैर-क्रमिक IDs (जैसे `/r/abc123`)
- ⏱️ **इंटरस्टिशियल पेज** - रीडायरेक्ट से पहले लक्ष्य URL दिखाने वाला काउंटडाउन टाइमर
- ⚡ **Redis कैशिंग** - कॉन्फ़िगर करने योग्य TTL के साथ तेज़ लुकअप
- 🛡️ **सर्किट ब्रेकर** - कैस्केडिंग विफलताओं के खिलाफ डेटाबेस सुरक्षा
- 🚦 **रेट लिमिटिंग** - ग्लोबल और डेटाबेस-स्तरीय रेट लिमिट
- 📊 **Prometheus मेट्रिक्स** - Basic Auth सुरक्षा के साथ पूर्ण ऑब्ज़र्वेबिलिटी
- 🎨 **सुंदर पेज** - 4 थीम के साथ साफ़ 404 और इंडेक्स पेज
- 🔑 **मल्टीपल सॉल्ट** - माइग्रेशन के लिए hashid सॉल्ट रोटेशन सपोर्ट
- 📱 **एडमिन डैशबोर्ड** - SSE के साथ रियल-टाइम मेट्रिक्स मॉनिटरिंग
- 📤 **इवेंट एनालिटिक्स** - RabbitMQ इवेंट पब्लिशिंग और PostgreSQL कंज्यूमर (वैकल्पिक)

## स्क्रीनशॉट

| लाइट | डार्क | ग्रे | वार्म |
|------|-------|------|-------|
| ![डैशबोर्ड लाइट](screenshots/dashboard-light.png) | ![डैशबोर्ड डार्क](screenshots/dashboard-dark.png) | ![डैशबोर्ड ग्रे](screenshots/dashboard-gray.png) | ![डैशबोर्ड वार्म](screenshots/dashboard-warm.png) |
| ![लॉगिन लाइट](screenshots/login-light.png) | ![लॉगिन डार्क](screenshots/login-dark.png) | ![लॉगिन ग्रे](screenshots/login-gray.png) | ![लॉगिन वार्म](screenshots/login-warm.png) |
| ![Index Light](screenshots/index-light.png) | ![Index Dark](screenshots/index-dark.png) | ![Index Gray](screenshots/index-gray.png) | ![Index Warm](screenshots/index-warm.png) |
| ![Interstitial Light](screenshots/interstitial-light.png) | ![Interstitial Dark](screenshots/interstitial-dark.png) | ![Interstitial Gray](screenshots/interstitial-gray.png) | ![Interstitial Warm](screenshots/interstitial-warm.png) |
| ![404 Light](screenshots/404-light.png) | ![404 Dark](screenshots/404-dark.png) | ![404 Gray](screenshots/404-gray.png) | ![404 Warm](screenshots/404-warm.png) |

### लोड टेस्ट मोडल

| लाइट | डार्क | ग्रे | वार्म |
|------|-------|------|-------|
| ![मोडल लाइट](screenshots/dashboard-modal-light.png) | ![मोडल डार्क](screenshots/dashboard-modal-dark.png) | ![मोडल ग्रे](screenshots/dashboard-modal-gray.png) | ![मोडल वार्म](screenshots/dashboard-modal-warm.png) |

## टेक्नोलॉजी स्टैक

- **भाषा**: Rust (Tokio के साथ async)
- **वेब फ्रेमवर्क**: Axum
- **कैश**: Redis-compatible (Redis, Dragonfly, Valkey, KeyDB आदि)
- **डेटाबेस**: PostgreSQL (प्लगेबल स्टोरेज लेयर)
- **मेट्रिक्स**: Prometheus + metrics-rs
- **मैसेज क्यू**: RabbitMQ (वैकल्पिक, इवेंट एनालिटिक्स के लिए)
- **पासवर्ड हैशिंग**: Argon2

> **नोट**: स्टोरेज और कैश लेयर्स एब्स्ट्रैक्टेड हैं और किसी भी संगत डेटा स्रोत से बदले जा सकते हैं। वर्तमान में सक्रिय विकास में।

## क्विक स्टार्ट

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

## कॉन्फ़िगरेशन

`config.yaml` बनाएं:

```yaml
server:
  host: "0.0.0.0"
  port: 8080

hashids:
  salts:
    - ${HASHID_SALT}          # प्राथमिक सॉल्ट
    - ${HASHID_SALT_OLD}      # वैकल्पिक: माइग्रेशन के लिए पुराना सॉल्ट
  min_length: 6

redis:
  url: ${REDIS_URL}
  cache_ttl_seconds: 86400    # 24 घंटे

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
    table: "dictionary.urls"    # आपका टेबल नाम
    id_column: "id"             # ID कॉलम
    url_column: "name"          # URL कॉलम

interstitial:
  delay_seconds: 5            # रीडायरेक्ट से पहले काउंटडाउन

metrics:
  basic_auth:
    username: prometheus
    password: ${METRICS_PASSWORD}

rate_limit:
  requests_per_second: 1000
  burst: 100
```

### कॉन्फ़िगरेशन विकल्प

#### सर्वर

| विकल्प | डिफ़ॉल्ट | विवरण |
|--------|----------|-------|
| `host` | `0.0.0.0` | बाइंड एड्रेस |
| `port` | `8080` | HTTP पोर्ट |

#### Hashids

| विकल्प | डिफ़ॉल्ट | विवरण |
|--------|----------|-------|
| `salts` | *आवश्यक* | hashid सॉल्ट्स की सूची (पहला = प्राथमिक) |
| `min_length` | `6` | न्यूनतम hashid लंबाई |

#### Redis

| विकल्प | डिफ़ॉल्ट | विवरण |
|--------|----------|-------|
| `url` | *आवश्यक* | Redis कनेक्शन URL |
| `cache_ttl_seconds` | `86400` | कैश TTL सेकंड में |

#### डेटाबेस

| विकल्प | डिफ़ॉल्ट | विवरण |
|--------|----------|-------|
| `url` | *आवश्यक* | PostgreSQL कनेक्शन URL |
| `pool.max_connections` | `3` | कनेक्शन पूल आकार |
| `pool.connect_timeout_seconds` | `3` | कनेक्शन टाइमआउट |
| `rate_limit.max_requests_per_second` | `50` | DB रेट लिमिट |
| `circuit_breaker.failure_threshold` | `3` | खुलने से पहले विफलताएं |
| `circuit_breaker.reset_timeout_seconds` | `60` | सर्किट रीसेट टाइमआउट |

#### रेट लिमिट (ग्लोबल)

| विकल्प | डिफ़ॉल्ट | विवरण |
|--------|----------|-------|
| `requests_per_second` | `1000` | ग्लोबल रेट लिमिट |
| `burst` | `100` | बर्स्ट क्षमता |

### पर्यावरण चर

सेवा को कॉन्फ़िगर करने के **तीन तरीके** हैं, प्राथमिकता के क्रम में (उच्चतम पहले):

| प्राथमिकता | विधि | उपयोग का मामला |
|------------|-------|-----------------|
| 1 | `REDIRECTOR__*` env वेरिएबल्स | व्यक्तिगत मान ओवरराइड करना |
| 2 | मानक PaaS env वेरिएबल्स (`DATABASE_URL` आदि) | PaaS प्लेटफ़ॉर्म (Railway, Heroku, Render) |
| 3 | कॉन्फ़िग फ़ाइल (`config.yaml` या `CONFIG_BASE64`) | आधार कॉन्फ़िगरेशन |

#### विशेष चर

| चर | डिफ़ॉल्ट | विवरण |
|----|----------|-------|
| `CONFIG_PATH` | `config.yaml` | YAML कॉन्फ़िग फ़ाइल का पथ |
| `CONFIG_BASE64` | — | Base64-एन्कोडेड YAML कॉन्फ़िग (`CONFIG_PATH` पर प्राथमिकता) |

#### मानक PaaS पर्यावरण चर

ये स्वचालित रूप से पहचाने और लागू किए जाते हैं। अधिकांश PaaS प्लेटफ़ॉर्म इन्हें आपके लिए सेट करते हैं:

| चर | कॉन्फ़िग पथ | उदाहरण |
|----|-------------|--------|
| `DATABASE_URL` | `database.url` | `postgres://user:pass@host:5432/db` |
| `REDIS_URL` | `redis.url` | `redis://host:6379` |
| `PORT` | `server.port` | `3000` |

> **प्राथमिकता नियम**: यदि `DATABASE_URL` और `REDIRECTOR__DATABASE__URL` दोनों सेट हैं, तो `REDIRECTOR__` प्रीफ़िक्स्ड संस्करण जीतता है।

#### प्रीफ़िक्स्ड पर्यावरण चर (`REDIRECTOR__*`)

किसी भी कॉन्फ़िग मान को `REDIRECTOR__` प्रीफ़िक्स और `__` (डबल अंडरस्कोर) नेस्टिंग सेपरेटर का उपयोग करके ओवरराइड किया जा सकता है:

```
YAML कॉन्फ़िग पथ            →  पर्यावरण चर
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

#### डिप्लॉयमेंट प्लेटफ़ॉर्म के अनुसार उदाहरण

**Railway / Render / Fly.io** (प्रबंधित डेटाबेस वाला PaaS):

```bash
# ये आमतौर पर प्लेटफ़ॉर्म द्वारा स्वचालित रूप से सेट किए जाते हैं:
DATABASE_URL=postgres://user:pass@host:5432/db
REDIS_URL=redis://host:6379
PORT=3000

# Base64 के माध्यम से कॉन्फ़िग सेट करें:
CONFIG_BASE64=c2VydmVyOgogIGhvc3Q6IC...

# या व्यक्तिगत मान ओवरराइड करें:
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
      # या कॉन्फ़िग फ़ाइल के ऊपर व्यक्तिगत मान ओवरराइड करें:
      REDIRECTOR__RATE_LIMIT__REQUESTS_PER_SECOND: "2000"
      REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD: "${METRICS_PASSWORD}"
    volumes:
      - ./config.yaml:/app/config.yaml  # CONFIG_BASE64 के साथ वैकल्पिक
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

**सादा Docker**:

```bash
docker run -p 8080:8080 \
  -e DATABASE_URL="postgres://user:pass@host:5432/db" \
  -e REDIS_URL="redis://host:6379" \
  -e CONFIG_BASE64="$(cat config.yaml | base64)" \
  ghcr.io/brilliant-almazov/redirector:latest
```

**न्यूनतम सेटअप (केवल env वेरिएबल्स, कोई कॉन्फ़िग फ़ाइल नहीं)**:

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

#### Base64 कॉन्फ़िगरेशन

ऐसे वातावरण के लिए जहाँ कॉन्फ़िग फ़ाइलें माउंट करना व्यावहारिक नहीं है (PaaS, serverless, CI/CD), पूरा कॉन्फ़िग Base64-एन्कोडेड स्ट्रिंग के रूप में पास करें:

```bash
# एन्कोड
cat config.yaml | base64

# डीकोड (सत्यापन के लिए)
echo "$CONFIG_BASE64" | base64 -d
```

`CONFIG_BASE64` की `CONFIG_PATH` पर प्राथमिकता है। पर्यावरण चर ओवरराइड (`REDIRECTOR__*` और PaaS चर) डीकोडेड कॉन्फ़िग के **ऊपर** लागू होते हैं।

## यह कैसे काम करता है

1. उपयोगकर्ता `/r/{hashid}` पर जाता है (जैसे `/r/abc123`)
2. सेवा hashid को न्यूमेरिक ID में डीकोड करती है
3. URL के लिए Redis कैश चेक करती है
4. कैश मिस पर, PostgreSQL क्वेरी करती है
5. परिणाम को Redis में कैश करती है
6. काउंटडाउन के साथ इंटरस्टिशियल पेज दिखाती है
7. काउंटडाउन के बाद, लक्ष्य URL पर रीडायरेक्ट करती है

## एंडपॉइंट्स

| एंडपॉइंट | Auth | विवरण |
|----------|------|-------|
| `GET /` | नहीं | इंडेक्स पेज |
| `GET /r/{hashid}` | नहीं | इंटरस्टिशियल के साथ रीडायरेक्ट |
| `GET /d/{hashid}` | नहीं | डेमो रीडायरेक्ट (सिंथेटिक लोड टेस्टिंग) |
| `GET /health` | नहीं | हेल्थ चेक |
| `GET /metrics` | Basic | Prometheus मेट्रिक्स |
| `GET /admin` | Session | एडमिन डैशबोर्ड लॉगिन |
| `GET /admin/dashboard` | Session | एडमिन डैशबोर्ड |

## एडमिन डैशबोर्ड

सेवा में रियल-टाइम मेट्रिक्स मॉनिटरिंग के लिए एक वैकल्पिक एडमिन डैशबोर्ड शामिल है।

### सेटअप

1. **पासवर्ड हैश जेनरेट करें:**

```bash
cargo run --bin hash_password
# पासवर्ड दर्ज करें, या:
cargo run --bin hash_password -- "your-password"
```

2. **config.yaml में जोड़ें:**

```yaml
admin:
  enabled: true
  session_ttl_hours: 24
  users:
    - username: admin
      password_hash: "$argon2id$v=19$m=19456,t=2,p=1$..."  # स्टेप 1 से
```

3. **डैशबोर्ड एक्सेस करें:**

`http://localhost:8080/admin` खोलें और अपने क्रेडेंशियल्स से लॉगिन करें।

### विशेषताएं

- रियल-टाइम RPS और लेटेंसी चार्ट
- सिस्टम मेट्रिक्स (CPU, मेमोरी, अपटाइम)
- कैश हिट रेट मॉनिटरिंग
- हाल के रीडायरेक्ट्स की सूची
- टेस्टिंग के लिए लोड सिमुलेशन
- तीन थीम: लाइट, डार्क, वार्म

## इवेंट एनालिटिक्स

रीडायरेक्ट एनालिटिक्स के लिए वैकल्पिक इवेंट पब्लिशिंग पाइपलाइन। सक्षम करने पर, हर रीडायरेक्ट इवेंट को RabbitMQ को भेजा जाता है और एक अलग कंज्यूमर द्वारा समृद्ध डेटा के साथ PostgreSQL में लिखा जाता है।

> **पूरा डॉक्यूमेंटेशन**: [docs/EVENT_ANALYTICS.md](EVENT_ANALYTICS.md)

### विशेषताएं

- **Fire-and-forget पब्लिशिंग** — रीडायरेक्ट विलंबता कतार उपलब्धता से प्रभावित नहीं होती
- **बैचिंग** — आकार (100) या समय (1 सेकंड) के आधार पर इवेंट्स को समूहीकृत किया जाता है
- **उपयोगकर्ता-एजेंट पार्सिंग** — woothee के माध्यम से ब्राउज़र, संस्करण, OS, डिवाइस प्रकार
- **GeoIP संवर्धन** — IP से देश और शहर (MaxMind mmdb के साथ लाइव-रीलोड)
- **संदर्भ डिडुप्लिकेशन** — रेफरर्स और उपयोगकर्ता-एजेंट्स के लिए MD5-आधारित डिडुप्लिकेशन
- **मासिक विभाजन** — `redirect_events` के लिए स्वचालित विभाजन निर्माण
- **डोमेन सामान्यीकरण** — `WWW.Example.COM` → `example.com`

### आर्किटेक्चर

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

### क्विक स्टार्ट

**1. config.yaml में सक्षम करें:**

```yaml
events:
  enabled: true
  rabbitmq:
    url: amqp://guest:guest@localhost:5672/%2f
    queue: redirector.events.analytics
  publisher:
    channel_buffer_size: 10000
    batch_size: 100
    flush_interval_ms: 1000
```

**2. पर्यावरण चर के माध्यम से:**

```bash
REDIRECTOR__EVENTS__ENABLED=true
REDIRECTOR__EVENTS__RABBITMQ__URL=amqp://guest:guest@localhost:5672/%2f
```

**3. Event Consumer चलाएं:**

```bash
# Cargo के साथ
RABBITMQ_URL=amqp://guest:guest@localhost:5672/%2f \
DATABASE_URL=postgres://localhost/redirector_analytics \
cargo run --bin event_consumer

# Docker के साथ
docker run -e RABBITMQ_URL=... -e DATABASE_URL=... \
  ghcr.io/brilliant-almazov/redirector:latest \
  /app/event_consumer
```

**4. (वैकल्पिक) GeoIP सक्षम करें:**

```bash
GEOIP_DB_PATH=/path/to/GeoLite2-City.mmdb
```

कंज्यूमर auto-reloads करता है यदि फ़ाइल बदलती है।

### Docker Compose के साथ इवेंट्स

```yaml
services:
  redirector:
    image: ghcr.io/brilliant-almazov/redirector:latest
    environment:
      REDIRECTOR__EVENTS__ENABLED: "true"
      RABBITMQ_URL: "amqp://guest:guest@rabbitmq:5672/%2f"
    depends_on:
      - rabbitmq

  event_consumer:
    image: ghcr.io/brilliant-almazov/redirector:latest
    command: ["/app/event_consumer"]
    environment:
      RABBITMQ_URL: "amqp://guest:guest@rabbitmq:5672/%2f"
      DATABASE_URL: "postgres://postgres:postgres@analytics-db:5432/analytics"
      GEOIP_DB_PATH: "/data/GeoLite2-City.mmdb"
    volumes:
      - ./GeoLite2-City.mmdb:/data/GeoLite2-City.mmdb:ro
    depends_on:
      - rabbitmq
      - analytics-db

  rabbitmq:
    image: rabbitmq:3-management-alpine
    ports:
      - "5672:5672"
      - "15672:15672"

  analytics-db:
    image: postgres:16-alpine
    environment:
      POSTGRES_DB: analytics
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: postgres
    volumes:
      - analytics-data:/var/lib/postgresql/data

volumes:
  analytics-data:
```

### इवेंट मेट्रिक्स

निम्नलिखित Prometheus मेट्रिक्स को ट्रैक किया जाता है:

| मेट्रिक | प्रकार | विवरण |
|---------|--------|--------|
| `events_published` | Counter | सफलतापूर्वक प्रकाशित इवेंट्स |
| `events_dropped` | Counter | छोड़े गए इवेंट्स (buffer full या कोई कनेक्शन नहीं) |
| `events_serialize_errors` | Counter | JSON serialization विफलताएं |
| `rabbitmq_connected` | Gauge | 1 यदि जुड़ा है, अन्यथा 0 |

### उदाहरण क्वेरीज़

**URL प्रति रीडायरेक्ट्स (पिछले 24 घंटे):**

```sql
SELECT url_id, COUNT(*) as redirects
FROM redirect_events
WHERE event_timestamp > NOW() - INTERVAL '24 hours'
GROUP BY url_id
ORDER BY redirects DESC
LIMIT 10;
```

**कैश हिट रेट:**

```sql
SELECT
  source,
  COUNT(*) as count,
  ROUND(100.0 * COUNT(*) / SUM(COUNT(*)) OVER (), 2) as percent
FROM redirect_events
WHERE event_timestamp > NOW() - INTERVAL '1 hour'
GROUP BY source;
```

**शीर्ष रेफरर डोमेन:**

```sql
SELECT rd.domain, COUNT(*) as visits
FROM redirect_events re
JOIN referer_domains rd ON re.referer_domain_id = rd.id
WHERE re.event_timestamp > NOW() - INTERVAL '7 days'
  AND rd.domain != '(unknown)'
GROUP BY rd.domain
ORDER BY visits DESC
LIMIT 20;
```

**भौगोलिक वितरण:**

```sql
SELECT gl.country_code, gl.city, COUNT(*) as visits
FROM redirect_events re
JOIN geo_locations gl ON re.geo_location_id = gl.id
WHERE re.event_timestamp > NOW() - INTERVAL '7 days'
  AND gl.country_code != '--'
GROUP BY gl.country_code, gl.city
ORDER BY visits DESC
LIMIT 50;
```

## लाइसेंस

MIT लाइसेंस - विवरण के लिए [LICENSE](../LICENSE) देखें।

## योगदान

योगदान स्वागत है! कृपया:

1. रिपॉजिटरी को फोर्क करें
2. फीचर ब्रांच बनाएं
3. Pull Request सबमिट करें

संरक्षित master ब्रांच को PR रिव्यू की आवश्यकता है।
