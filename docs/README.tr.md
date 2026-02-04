# redirector

> **Yüksek performanslı URL kısaltıcı ve yönlendirme servisi** Rust, Axum, Redis ve PostgreSQL ile geliştirilmiştir. Güvenli ara sayfalar, gerçek zamanlı yönetim paneli ve kurumsal düzeyde gözlemlenebilirlik içerir.

[English](../README.md) | [Русский](README.ru.md) | [中文](README.zh.md) | [हिंदी](README.hi.md) | [Español](README.es.md) | [Português](README.pt.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Polski](README.pl.md) | [Nederlands](README.nl.md) | [Italiano](README.it.md) | **Türkçe** | [Українська](README.uk.md) | [עברית](README.he.md) | [Bahasa Indonesia](README.id.md) | [Tiếng Việt](README.vi.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![Docker Image Size](https://ghcr-badge.egpl.dev/brilliant-almazov/redirector/size)](https://github.com/brilliant-almazov/redirector/pkgs/container/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

**Anahtar Kelimeler**: URL kısaltıcı, bağlantı kısaltıcı, yönlendirme servisi, Rust web servisi, Axum framework, Redis önbellek, PostgreSQL, Prometheus metrikleri, hashids, kısa bağlantılar, ara sayfalar, güvenli yönlendirmeler, yüksek performans, mikroservis

Ara sayfalar ve hashid tabanlı kısa bağlantılarla güvenli URL yönlendirme servisi. Dahili araçlar, kurumsal bağlantı yönetimi ve markalı kısa URL servisleri için idealdir.

### Performans

| Senaryo | RPS | Ort. Gecikme | P99 Gecikme |
|---------|-----|--------------|-------------|
| 100% Cache Hit | **7.800+** | ~14ms | ~50ms |
| Cache Miss (10K URLs) | **2.300+** | ~44ms | ~81ms |

**Test koşulları**: wrk -t4 -c100 -d30s, PostgreSQL 15, Dragonfly (Redis), macOS M1 (Docker)

> ⚠️ Sonuçlar VM overhead'li macOS Docker'dan alınmıştır. Native Linux deployment'ın **3-5x daha hızlı** olması beklenmektedir.

## Problem

Uzun URL'leri paylaşmak zahmetlidir. URL kısaltıcılar mevcut ancak çoğu zaman hemen yönlendirir, bu da güvenlik riski oluşturabilir. Kullanıcılar yönlendirilmeden önce nereye gittiklerini görebilmelidir.

**redirector** güvenli yönlendirmeler sağlar:
- Yönlendirmeden önce hedef URL'yi gösteren ara sayfa
- Kullanıcı farkındalığı için geri sayım zamanlayıcısı
- Güzel, markalı sayfalar

## Özellikler

- 🔗 **Hashid URL'ler** - Kısa, benzersiz, sıralı olmayan ID'ler (örn. `/r/abc123`)
- ⏱️ **Ara sayfa** - Yönlendirmeden önce hedef URL'yi gösteren geri sayım zamanlayıcısı
- ⚡ **Redis önbellekleme** - Yapılandırılabilir TTL ile hızlı aramalar
- 🛡️ **Circuit breaker** - Kademeli arızalara karşı veritabanı koruması
- 🚦 **Hız sınırlama** - Global ve veritabanı düzeyinde hız sınırları
- 📊 **Prometheus metrikleri** - Basic Auth korumalı tam gözlemlenebilirlik
- 🎨 **Güzel sayfalar** - 4 tema ile temiz 404 ve dizin sayfaları
- 🔑 **Çoklu salt** - Geçiş için hashid salt rotasyonu desteği
- 📱 **Yönetim paneli** - SSE ile gerçek zamanlı metrik izleme

## Ekran Görüntüleri

| Açık | Koyu | Gri | Sıcak |
|------|------|-----|-------|
| ![Dashboard Açık](screenshots/dashboard-light.png) | ![Dashboard Koyu](screenshots/dashboard-dark.png) | ![Dashboard Gri](screenshots/dashboard-gray.png) | ![Dashboard Sıcak](screenshots/dashboard-warm.png) |
| ![Giriş Açık](screenshots/login-light.png) | ![Giriş Koyu](screenshots/login-dark.png) | ![Giriş Gri](screenshots/login-gray.png) | ![Giriş Sıcak](screenshots/login-warm.png) |
| ![Index Light](screenshots/index-light.png) | ![Index Dark](screenshots/index-dark.png) | ![Index Gray](screenshots/index-gray.png) | ![Index Warm](screenshots/index-warm.png) |
| ![Interstitial Light](screenshots/interstitial-light.png) | ![Interstitial Dark](screenshots/interstitial-dark.png) | ![Interstitial Gray](screenshots/interstitial-gray.png) | ![Interstitial Warm](screenshots/interstitial-warm.png) |
| ![404 Light](screenshots/404-light.png) | ![404 Dark](screenshots/404-dark.png) | ![404 Gray](screenshots/404-gray.png) | ![404 Warm](screenshots/404-warm.png) |

### Yük Testi Modalı

| Açık | Koyu | Gri | Sıcak |
|------|------|-----|-------|
| ![Modal Açık](screenshots/dashboard-modal-light.png) | ![Modal Koyu](screenshots/dashboard-modal-dark.png) | ![Modal Gri](screenshots/dashboard-modal-gray.png) | ![Modal Sıcak](screenshots/dashboard-modal-warm.png) |

## Teknoloji Yığını

- **Dil**: Rust (Tokio ile async)
- **Web Framework**: Axum
- **Önbellek**: Redis uyumlu (Redis, Dragonfly, Valkey, KeyDB vb.)
- **Veritabanı**: PostgreSQL (değiştirilebilir depolama katmanı)
- **Metrikler**: Prometheus + metrics-rs
- **Şifre Hash**: Argon2

> **Not**: Depolama ve önbellek katmanları soyutlanmıştır ve herhangi bir uyumlu veri kaynağıyla değiştirilebilir. Şu anda aktif geliştirme aşamasındadır.

## Hızlı Başlangıç

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

### Ortam Değişkenleri

Servisi yapılandırmanın **üç yolu** vardır, öncelik sırasına göre (en yüksekten en düşüğe):

| Öncelik | Yöntem | Kullanım Senaryosu |
|---------|--------|---------------------|
| 1 | `REDIRECTOR__*` ortam değişkenleri | Bireysel değerleri geçersiz kılma |
| 2 | Standart PaaS ortam değişkenleri (`DATABASE_URL` vb.) | PaaS platformları (Railway, Heroku, Render) |
| 3 | Yapılandırma dosyası (`config.yaml` veya `CONFIG_BASE64`) | Temel yapılandırma |

#### Özel Değişkenler

| Değişken | Varsayılan | Açıklama |
|----------|-----------|----------|
| `CONFIG_PATH` | `config.yaml` | YAML yapılandırma dosyasının yolu |
| `CONFIG_BASE64` | — | Base64 kodlanmış YAML yapılandırması (`CONFIG_PATH`'e göre önceliklidir) |

#### Standart PaaS Ortam Değişkenleri

Bunlar otomatik olarak tanınır ve uygulanır. Çoğu PaaS platformu bunları sizin için ayarlar:

| Değişken | Yapılandırma Yolu | Örnek |
|----------|-------------------|-------|
| `DATABASE_URL` | `database.url` | `postgres://user:pass@host:5432/db` |
| `REDIS_URL` | `redis.url` | `redis://host:6379` |
| `PORT` | `server.port` | `3000` |
| `HASHIDS_SALTS` | `hashids.salts` | `new-salt,old-salt` (virgülle ayrılmış) |

> **Öncelik kuralı**: Hem `DATABASE_URL` hem de `REDIRECTOR__DATABASE__URL` ayarlanmışsa, `REDIRECTOR__` ön ekli sürüm kazanır. Benzer şekilde, `REDIRECTOR__HASHIDS__SALTS__0`, `HASHIDS_SALTS`'a göre önceliklidir.

#### Ön Ekli Ortam Değişkenleri (`REDIRECTOR__*`)

Herhangi bir yapılandırma değeri `REDIRECTOR__` ön eki ile geçersiz kılınabilir; `__` (çift alt çizgi) iç içe ayırıcı olarak kullanılır. Aşağıda geçersiz kılınabilir tüm değişkenlerin **tam referansı** verilmiştir:

##### Server

| Ortam Değişkeni | Yapılandırma Yolu | Varsayılan | Açıklama |
|-----------------|-------------------|-----------|----------|
| `REDIRECTOR__SERVER__HOST` | `server.host` | `0.0.0.0` | Bağlanma adresi |
| `REDIRECTOR__SERVER__PORT` | `server.port` | `8080` | HTTP portu |

##### Hashids

| Ortam Değişkeni | Yapılandırma Yolu | Varsayılan | Açıklama |
|-----------------|-------------------|-----------|----------|
| `REDIRECTOR__HASHIDS__SALTS__0` | `hashids.salts[0]` | *gerekli* | Birincil hashid tuzu |
| `REDIRECTOR__HASHIDS__SALTS__1` | `hashids.salts[1]` | — | Eski tuz (geçiş için) |
| `REDIRECTOR__HASHIDS__MIN_LENGTH` | `hashids.min_length` | `6` | Minimum hashid uzunluğu |

> **Diziler**: Liste öğeleri `__0`, `__1`, `__2` vb. ile indekslenir. Hashid tuz rotasyonu için `__0`'ı yeni tuz, `__1`'i eski tuz olarak ayarlayın.

##### Redis / Önbellek

| Ortam Değişkeni | Yapılandırma Yolu | Varsayılan | Açıklama |
|-----------------|-------------------|-----------|----------|
| `REDIRECTOR__REDIS__URL` | `redis.url` | *gerekli* | Redis bağlantı URL'si |
| `REDIRECTOR__REDIS__CACHE_TTL_SECONDS` | `redis.cache_ttl_seconds` | `86400` | Önbellek TTL (saniye). `86400` = 24 saat |

##### Veritabanı

| Ortam Değişkeni | Yapılandırma Yolu | Varsayılan | Açıklama |
|-----------------|-------------------|-----------|----------|
| `REDIRECTOR__DATABASE__URL` | `database.url` | *gerekli* | PostgreSQL bağlantı URL'si |
| `REDIRECTOR__DATABASE__POOL__MAX_CONNECTIONS` | `database.pool.max_connections` | `3` | Bağlantı havuzu boyutu |
| `REDIRECTOR__DATABASE__POOL__CONNECT_TIMEOUT_SECONDS` | `database.pool.connect_timeout_seconds` | `3` | Bağlantı zaman aşımı (saniye) |
| `REDIRECTOR__DATABASE__RATE_LIMIT__MAX_REQUESTS_PER_SECOND` | `database.rate_limit.max_requests_per_second` | `50` | Saniyede maks. DB sorgusu |
| `REDIRECTOR__DATABASE__CIRCUIT_BREAKER__FAILURE_THRESHOLD` | `database.circuit_breaker.failure_threshold` | `3` | Devre açılmadan önceki ardışık hata sayısı |
| `REDIRECTOR__DATABASE__CIRCUIT_BREAKER__RESET_TIMEOUT_SECONDS` | `database.circuit_breaker.reset_timeout_seconds` | `60` | Yarı açık yeniden deneme öncesi saniye |
| `REDIRECTOR__DATABASE__QUERY__TABLE` | `database.query.table` | `dictionary.urls` | URL aramaları için tablo adı |
| `REDIRECTOR__DATABASE__QUERY__ID_COLUMN` | `database.query.id_column` | `id` | Sayısal ID için sütun adı |
| `REDIRECTOR__DATABASE__QUERY__URL_COLUMN` | `database.query.url_column` | `name` | Hedef URL için sütun adı |

##### Ara Sayfa

| Ortam Değişkeni | Yapılandırma Yolu | Varsayılan | Açıklama |
|-----------------|-------------------|-----------|----------|
| `REDIRECTOR__INTERSTITIAL__DELAY_SECONDS` | `interstitial.delay_seconds` | `5` | Yönlendirmeden önceki geri sayım |

##### Metrikler

| Ortam Değişkeni | Yapılandırma Yolu | Varsayılan | Açıklama |
|-----------------|-------------------|-----------|----------|
| `REDIRECTOR__METRICS__BASIC_AUTH__USERNAME` | `metrics.basic_auth.username` | *gerekli* | `/metrics` endpoint'i için kullanıcı adı |
| `REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD` | `metrics.basic_auth.password` | *gerekli* | `/metrics` endpoint'i için şifre |

##### Hız Sınırlama (Global)

| Ortam Değişkeni | Yapılandırma Yolu | Varsayılan | Açıklama |
|-----------------|-------------------|-----------|----------|
| `REDIRECTOR__RATE_LIMIT__REQUESTS_PER_SECOND` | `rate_limit.requests_per_second` | `1000` | Saniyede maks. istek |
| `REDIRECTOR__RATE_LIMIT__BURST` | `rate_limit.burst` | `100` | RPS limitinin üzerinde izin verilen patlama |

##### Yönetim Paneli

| Ortam Değişkeni | Yapılandırma Yolu | Varsayılan | Açıklama |
|-----------------|-------------------|-----------|----------|
| `REDIRECTOR__ADMIN__ENABLED` | `admin.enabled` | `false` | Yönetim panelini etkinleştir |
| `REDIRECTOR__ADMIN__SESSION_SECRET` | `admin.session_secret` | `change-me-...` | Oturum imzalama sırrı (min 32 karakter) |
| `REDIRECTOR__ADMIN__SESSION_TTL_HOURS` | `admin.session_ttl_hours` | `24` | Saat cinsinden oturum ömrü |

> **Not**: Admin kullanıcıları (`admin.users`) `username` ve `password_hash` ile karmaşık yapıları nedeniyle ortam değişkenleri ile ayarlanamaz. Bunları yapılandırma dosyasında veya `CONFIG_BASE64` ile tanımlayın.

#### Dağıtım Platformuna Göre Örnekler

**Railway / Render / Fly.io** (yönetilen veritabanlarıyla PaaS):

```bash
# Bunlar genellikle platform tarafından otomatik ayarlanır:
DATABASE_URL=postgres://user:pass@host:5432/db
REDIS_URL=redis://host:6379
PORT=3000

# Yapılandırmanızı base64 ile ayarlayın:
CONFIG_BASE64=c2VydmVyOgogIGhvc3Q6IC...

# Veya bireysel değerleri geçersiz kılın:
REDIRECTOR__HASHIDS__SALTS__0=my-secret-salt
REDIRECTOR__METRICS__BASIC_AUTH__USERNAME=prometheus
REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD=strong-password
REDIRECTOR__ADMIN__ENABLED=true
REDIRECTOR__ADMIN__SESSION_SECRET=random-32-byte-secret-for-sessions
```

**Docker Compose (tüm geçersiz kılmalarla tam örnek)**:

```yaml
services:
  redirector:
    image: ghcr.io/brilliant-almazov/redirector:latest
    ports:
      - "8080:8080"
    environment:
      # --- Bağlantı URL'leri (PaaS tarzı) ---
      DATABASE_URL: "postgres://redirector:${DB_PASSWORD}@postgres:5432/redirector"
      REDIS_URL: "redis://redis:6379"

      # --- Yapılandırma dosyası ---
      CONFIG_BASE64: "${CONFIG_BASE64}"

      # --- Sunucu ---
      REDIRECTOR__SERVER__HOST: "0.0.0.0"
      REDIRECTOR__SERVER__PORT: "8080"

      # --- Hashid tuzları ---
      REDIRECTOR__HASHIDS__SALTS__0: "${HASHID_SALT}"        # birincil tuz
      REDIRECTOR__HASHIDS__SALTS__1: "${HASHID_SALT_OLD}"    # geçiş için eski tuz
      REDIRECTOR__HASHIDS__MIN_LENGTH: "6"

      # --- Redis önbellek ---
      REDIRECTOR__REDIS__CACHE_TTL_SECONDS: "43200"          # 12 saat

      # --- Veritabanı havuzu ve dayanıklılık ---
      REDIRECTOR__DATABASE__POOL__MAX_CONNECTIONS: "5"
      REDIRECTOR__DATABASE__POOL__CONNECT_TIMEOUT_SECONDS: "5"
      REDIRECTOR__DATABASE__RATE_LIMIT__MAX_REQUESTS_PER_SECOND: "100"
      REDIRECTOR__DATABASE__CIRCUIT_BREAKER__FAILURE_THRESHOLD: "5"
      REDIRECTOR__DATABASE__CIRCUIT_BREAKER__RESET_TIMEOUT_SECONDS: "30"

      # --- Özel tablo eşlemesi ---
      REDIRECTOR__DATABASE__QUERY__TABLE: "public.short_urls"
      REDIRECTOR__DATABASE__QUERY__ID_COLUMN: "id"
      REDIRECTOR__DATABASE__QUERY__URL_COLUMN: "target_url"

      # --- Ara sayfa ---
      REDIRECTOR__INTERSTITIAL__DELAY_SECONDS: "3"

      # --- Metrik kimlik doğrulaması ---
      REDIRECTOR__METRICS__BASIC_AUTH__USERNAME: "prometheus"
      REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD: "${METRICS_PASSWORD}"

      # --- Global hız limiti ---
      REDIRECTOR__RATE_LIMIT__REQUESTS_PER_SECOND: "2000"
      REDIRECTOR__RATE_LIMIT__BURST: "200"

      # --- Yönetim paneli ---
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

**Düz Docker (tek komut)**:

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

**Minimal kurulum (yalnızca ortam değişkenleri, yapılandırma dosyası yok)**:

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

#### Ortam Değişkenleri ile Tuz Rotasyonu

Hashid tuzlarını döndürürken, servis tuzları sırayla dener — ilk eşleşme kazanır. Yeni bağlantıların onu kullanması için yeni tuzu önce ayarlayın ve geriye dönük uyumluluk için eski tuzu tutun:

**Seçenek 1: Virgülle ayrılmış tek değişken** (önerilen):

```bash
# Rotasyondan önce
HASHIDS_SALTS=original-salt

# Rotasyondan sonra — yeni tuz önce, mevcut bağlantılar için eski tuz
HASHIDS_SALTS=new-salt,original-salt
```

**Seçenek 2: İndeksli değişkenler**:

```bash
# Rotasyondan önce
REDIRECTOR__HASHIDS__SALTS__0=original-salt

# Rotasyondan sonra
REDIRECTOR__HASHIDS__SALTS__0=new-salt
REDIRECTOR__HASHIDS__SALTS__1=original-salt
```

> **Not**: `REDIRECTOR__HASHIDS__SALTS__0` ayarlanmışsa, `HASHIDS_SALTS` görmezden gelinir.

#### Base64 Yapılandırması

Yapılandırma dosyalarının bağlanmasının pratik olmadığı ortamlar için (PaaS, serverless, CI/CD), tüm yapılandırmayı base64 kodlanmış dize olarak geçirin:

```bash
# Encode
cat config.yaml | base64

# Doğrulamak için decode
echo "$CONFIG_BASE64" | base64 -d
```

`CONFIG_BASE64`, `CONFIG_PATH`'e göre önceliklidir. Ortam değişkeni geçersiz kılmaları (`REDIRECTOR__*` ve PaaS değişkenleri) çözülen yapılandırmanın **üzerine** uygulanır.

## Nasıl Çalışır

1. Kullanıcı `/r/{hashid}` ziyaret eder (örn. `/r/abc123`)
2. Servis hashid'i sayısal ID'ye çözer
3. Redis önbelleğinde URL'yi kontrol eder
4. Önbellek kaçırmasında PostgreSQL'i sorgular
5. Sonucu Redis'te önbelleğe alır
6. Geri sayımlı ara sayfayı gösterir
7. Geri sayımdan sonra hedef URL'ye yönlendirir

## Endpoint'ler

| Endpoint | Yetki | Açıklama |
|----------|-------|----------|
| `GET /` | Hayır | Ana sayfa |
| `GET /r/{hashid}` | Hayır | Ara sayfa ile yönlendirme |
| `GET /d/{hashid}` | Hayır | Demo yönlendirme (sentetik yük testi) |
| `GET /health` | Hayır | Sağlık kontrolü |
| `GET /metrics` | Basic | Prometheus metrikleri |
| `GET /admin` | Session | Yönetim paneli girişi |
| `GET /admin/dashboard` | Session | Yönetim paneli |

## Yönetim Paneli

Servis, gerçek zamanlı metrik izleme için opsiyonel bir yönetim paneli içerir.

### Kurulum

1. **Şifre hash oluştur:**

```bash
cargo run --bin hash_password
# Şifre girin veya:
cargo run --bin hash_password -- "your-password"
```

2. **config.yaml'a ekle:**

```yaml
admin:
  enabled: true
  session_ttl_hours: 24
  users:
    - username: admin
      password_hash: "$argon2id$v=19$m=19456,t=2,p=1$..."  # adım 1'den
```

3. **Panele erişin:**

`http://localhost:8080/admin` açın ve kimlik bilgilerinizle giriş yapın.

### Özellikler

- Gerçek zamanlı RPS ve gecikme grafikleri
- Sistem metrikleri (CPU, bellek, çalışma süresi)
- Önbellek isabet oranı izleme
- Son yönlendirmeler listesi
- Test için yük simülasyonu
- Üç tema: Açık, Koyu, Sıcak

## Lisans

MIT Lisansı - detaylar için [LICENSE](../LICENSE) dosyasına bakın.

## Katkıda Bulunma

Katkılar memnuniyetle karşılanır! Lütfen:

1. Repository'yi fork edin
2. Özellik branch'i oluşturun
3. Pull Request gönderin

Korumalı master branch PR incelemesi gerektirir.
