# redirector

> **Layanan pemendek URL dan pengalihan berkinerja tinggi** dibangun dengan Rust, Axum, Redis, dan PostgreSQL. Dilengkapi halaman antara yang aman, dasbor admin real-time, dan observabilitas tingkat enterprise.

[English](../README.md) | [Русский](README.ru.md) | [中文](README.zh.md) | [हिंदी](README.hi.md) | [Español](README.es.md) | [Português](README.pt.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Polski](README.pl.md) | [Nederlands](README.nl.md) | [Italiano](README.it.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) | [עברית](README.he.md) | **Bahasa Indonesia** | [Tiếng Việt](README.vi.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![Docker Image Size](https://ghcr-badge.egpl.dev/brilliant-almazov/redirector/size)](https://github.com/brilliant-almazov/redirector/pkgs/container/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

**Kata Kunci**: pemendek URL, pemendek tautan, layanan pengalihan, layanan web Rust, framework Axum, cache Redis, PostgreSQL, metrik Prometheus, hashids, tautan pendek, halaman antara, pengalihan aman, kinerja tinggi, microservice

Layanan pengalihan URL yang aman dengan halaman antara dan tautan pendek berbasis hashid. Sempurna untuk alat internal, manajemen tautan perusahaan, dan layanan URL pendek bermerek.

### Kinerja

| Skenario | RPS | Latensi Rata-rata | Latensi P99 |
|----------|-----|-------------------|-------------|
| 100% Cache Hit | **7.800+** | ~14ms | ~50ms |
| Cache Miss (10K URLs) | **2.300+** | ~44ms | ~81ms |

**Kondisi pengujian**: wrk -t4 -c100 -d30s, PostgreSQL 15, Dragonfly (Redis), macOS M1 (Docker)

> ⚠️ Hasil dari Docker di macOS dengan overhead VM. Deployment Linux native diharapkan **3-5x lebih cepat**.

## Masalah

Berbagi URL panjang tidak praktis. Pemendek URL ada tetapi sering langsung mengalihkan, yang bisa menjadi risiko keamanan. Pengguna harus melihat ke mana mereka akan pergi sebelum dialihkan.

**redirector** menyediakan pengalihan yang aman dengan:
- Halaman antara yang menampilkan URL tujuan sebelum pengalihan
- Timer hitung mundur untuk kesadaran pengguna
- Halaman yang indah dan bermerek

## Fitur

- 🔗 **URL Hashid** - ID pendek, unik, dan tidak berurutan (mis. `/r/abc123`)
- ⏱️ **Halaman antara** - Timer hitung mundur menampilkan URL tujuan sebelum pengalihan
- ⚡ **Caching Redis** - Pencarian cepat dengan TTL yang dapat dikonfigurasi
- 🛡️ **Circuit breaker** - Perlindungan database terhadap kegagalan berantai
- 🚦 **Rate limiting** - Batas rate global dan tingkat database
- 📊 **Metrik Prometheus** - Observabilitas penuh dengan perlindungan Basic Auth
- 🎨 **Halaman indah** - Halaman 404 dan indeks yang bersih dengan 4 tema
- 🔑 **Multiple salt** - Dukungan rotasi salt hashid untuk migrasi
- 📱 **Dasbor admin** - Pemantauan metrik real-time dengan SSE

## Tangkapan Layar

| Terang | Gelap | Abu-abu | Hangat |
|--------|-------|---------|--------|
| ![Dashboard Terang](screenshots/dashboard-light.png) | ![Dashboard Gelap](screenshots/dashboard-dark.png) | ![Dashboard Abu-abu](screenshots/dashboard-gray.png) | ![Dashboard Hangat](screenshots/dashboard-warm.png) |
| ![Login Terang](screenshots/login-light.png) | ![Login Gelap](screenshots/login-dark.png) | ![Login Abu-abu](screenshots/login-gray.png) | ![Login Hangat](screenshots/login-warm.png) |
| ![Index Light](screenshots/index-light.png) | ![Index Dark](screenshots/index-dark.png) | ![Index Gray](screenshots/index-gray.png) | ![Index Warm](screenshots/index-warm.png) |
| ![Interstitial Light](screenshots/interstitial-light.png) | ![Interstitial Dark](screenshots/interstitial-dark.png) | ![Interstitial Gray](screenshots/interstitial-gray.png) | ![Interstitial Warm](screenshots/interstitial-warm.png) |
| ![404 Light](screenshots/404-light.png) | ![404 Dark](screenshots/404-dark.png) | ![404 Gray](screenshots/404-gray.png) | ![404 Warm](screenshots/404-warm.png) |

### Modal Uji Beban

| Terang | Gelap | Abu-abu | Hangat |
|--------|-------|---------|--------|
| ![Modal Terang](screenshots/dashboard-modal-light.png) | ![Modal Gelap](screenshots/dashboard-modal-dark.png) | ![Modal Abu-abu](screenshots/dashboard-modal-gray.png) | ![Modal Hangat](screenshots/dashboard-modal-warm.png) |

## Stack Teknologi

- **Bahasa**: Rust (async dengan Tokio)
- **Framework Web**: Axum
- **Cache**: Kompatibel Redis (Redis, Dragonfly, Valkey, KeyDB, dll.)
- **Database**: PostgreSQL (lapisan penyimpanan yang dapat ditukar)
- **Metrik**: Prometheus + metrics-rs
- **Hashing Password**: Argon2

> **Catatan**: Lapisan penyimpanan dan cache diabstraksikan dan dapat diganti dengan sumber data yang kompatibel. Saat ini dalam pengembangan aktif.

## Mulai Cepat

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

## Konfigurasi

Buat `config.yaml`:

```yaml
server:
  host: "0.0.0.0"
  port: 8080

hashids:
  salts:
    - ${HASHID_SALT}          # Salt utama
    - ${HASHID_SALT_OLD}      # Opsional: salt lama untuk migrasi
  min_length: 6

redis:
  url: ${REDIS_URL}
  cache_ttl_seconds: 86400    # 24 jam

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
    table: "dictionary.urls"    # Nama tabel Anda
    id_column: "id"             # Kolom ID
    url_column: "name"          # Kolom URL

interstitial:
  delay_seconds: 5            # Hitung mundur sebelum pengalihan

metrics:
  basic_auth:
    username: prometheus
    password: ${METRICS_PASSWORD}

rate_limit:
  requests_per_second: 1000
  burst: 100
```

### Opsi Konfigurasi

#### Server

| Opsi | Default | Deskripsi |
|------|---------|-----------|
| `host` | `0.0.0.0` | Alamat bind |
| `port` | `8080` | Port HTTP |

#### Hashids

| Opsi | Default | Deskripsi |
|------|---------|-----------|
| `salts` | *wajib* | Daftar salt hashid (pertama = utama) |
| `min_length` | `6` | Panjang minimum hashid |

#### Redis

| Opsi | Default | Deskripsi |
|------|---------|-----------|
| `url` | *wajib* | URL koneksi Redis |
| `cache_ttl_seconds` | `86400` | TTL cache dalam detik |

#### Database

| Opsi | Default | Deskripsi |
|------|---------|-----------|
| `url` | *wajib* | URL koneksi PostgreSQL |
| `pool.max_connections` | `3` | Ukuran connection pool |
| `pool.connect_timeout_seconds` | `3` | Timeout koneksi |
| `rate_limit.max_requests_per_second` | `50` | Rate limit DB |
| `circuit_breaker.failure_threshold` | `3` | Kegagalan sebelum terbuka |
| `circuit_breaker.reset_timeout_seconds` | `60` | Timeout reset circuit |

#### Rate Limit (Global)

| Opsi | Default | Deskripsi |
|------|---------|-----------|
| `requests_per_second` | `1000` | Rate limit global |
| `burst` | `100` | Kapasitas burst |

### Variabel Lingkungan

Ada **tiga cara** untuk mengkonfigurasi layanan, diurutkan berdasarkan prioritas (tertinggi pertama):

| Prioritas | Metode | Kasus Penggunaan |
|-----------|--------|------------------|
| 1 | Variabel `REDIRECTOR__*` | Menimpa nilai individual |
| 2 | Variabel PaaS standar (`DATABASE_URL`, dll.) | Platform PaaS (Railway, Heroku, Render) |
| 3 | File konfigurasi (`config.yaml` atau `CONFIG_BASE64`) | Konfigurasi dasar |

#### Variabel Khusus

| Variabel | Default | Deskripsi |
|----------|---------|-----------|
| `CONFIG_PATH` | `config.yaml` | Path ke file konfigurasi YAML |
| `CONFIG_BASE64` | — | Konfigurasi YAML yang di-encode Base64 (prioritas atas `CONFIG_PATH`) |

#### Variabel Lingkungan PaaS Standar

Ini secara otomatis dikenali dan diterapkan. Kebanyakan platform PaaS mengaturnya untuk Anda:

| Variabel | Path Konfigurasi | Contoh |
|----------|-----------------|--------|
| `DATABASE_URL` | `database.url` | `postgres://user:pass@host:5432/db` |
| `REDIS_URL` | `redis.url` | `redis://host:6379` |
| `PORT` | `server.port` | `3000` |

> **Aturan prioritas**: Jika `DATABASE_URL` dan `REDIRECTOR__DATABASE__URL` keduanya diatur, versi dengan prefiks `REDIRECTOR__` yang menang.

#### Variabel dengan Prefiks (`REDIRECTOR__*`)

Setiap nilai konfigurasi dapat ditimpa menggunakan prefiks `REDIRECTOR__` dengan `__` (garis bawah ganda) sebagai pemisah bersarang:

```
Path konfigurasi YAML      →  Variabel lingkungan
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

#### Contoh berdasarkan Platform Deployment

**Railway / Render / Fly.io** (PaaS dengan database terkelola):

```bash
# Ini biasanya diatur secara otomatis oleh platform:
DATABASE_URL=postgres://user:pass@host:5432/db
REDIS_URL=redis://host:6379
PORT=3000

# Atur konfigurasi via base64:
CONFIG_BASE64=c2VydmVyOgogIGhvc3Q6IC...

# Atau timpa nilai individual:
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
      # Atau timpa nilai individual di atas file konfigurasi:
      REDIRECTOR__RATE_LIMIT__REQUESTS_PER_SECOND: "2000"
      REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD: "${METRICS_PASSWORD}"
    volumes:
      - ./config.yaml:/app/config.yaml  # opsional jika menggunakan CONFIG_BASE64
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

**Docker biasa**:

```bash
docker run -p 8080:8080 \
  -e DATABASE_URL="postgres://user:pass@host:5432/db" \
  -e REDIS_URL="redis://host:6379" \
  -e CONFIG_BASE64="$(cat config.yaml | base64)" \
  ghcr.io/brilliant-almazov/redirector:latest
```

**Setup minimal (hanya variabel env, tanpa file konfigurasi)**:

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

#### Konfigurasi Base64

Untuk lingkungan di mana mounting file konfigurasi tidak praktis (PaaS, serverless, CI/CD), kirim seluruh konfigurasi sebagai string yang di-encode Base64:

```bash
# Encode
cat config.yaml | base64

# Decode (untuk verifikasi)
echo "$CONFIG_BASE64" | base64 -d
```

`CONFIG_BASE64` memiliki prioritas atas `CONFIG_PATH`. Override variabel lingkungan (`REDIRECTOR__*` dan variabel PaaS) diterapkan **di atas** konfigurasi yang di-decode.

## Cara Kerja

1. Pengguna mengunjungi `/r/{hashid}` (mis. `/r/abc123`)
2. Layanan mendekode hashid ke ID numerik
3. Memeriksa cache Redis untuk URL
4. Pada cache miss, query ke PostgreSQL
5. Menyimpan hasil di cache Redis
6. Menampilkan halaman antara dengan hitung mundur
7. Setelah hitung mundur, mengalihkan ke URL tujuan

## Endpoint

| Endpoint | Auth | Deskripsi |
|----------|------|-----------|
| `GET /` | Tidak | Halaman indeks |
| `GET /r/{hashid}` | Tidak | Pengalihan dengan halaman antara |
| `GET /d/{hashid}` | Tidak | Demo pengalihan (pengujian beban sintetis) |
| `GET /health` | Tidak | Health check |
| `GET /metrics` | Basic | Metrik Prometheus |
| `GET /admin` | Session | Login dasbor admin |
| `GET /admin/dashboard` | Session | Dasbor admin |

## Dasbor Admin

Layanan menyertakan dasbor admin opsional untuk pemantauan metrik real-time.

### Pengaturan

1. **Buat hash password:**

```bash
cargo run --bin hash_password
# Masukkan password, atau:
cargo run --bin hash_password -- "your-password"
```

2. **Tambahkan ke config.yaml:**

```yaml
admin:
  enabled: true
  session_ttl_hours: 24
  users:
    - username: admin
      password_hash: "$argon2id$v=19$m=19456,t=2,p=1$..."  # dari langkah 1
```

3. **Akses dasbor:**

Buka `http://localhost:8080/admin` dan login dengan kredensial Anda.

### Fitur

- Grafik RPS dan latensi real-time
- Metrik sistem (CPU, memori, uptime)
- Pemantauan cache hit rate
- Daftar pengalihan terbaru
- Simulasi beban untuk pengujian
- Tiga tema: Terang, Gelap, Hangat

## Lisensi

Lisensi MIT - lihat [LICENSE](../LICENSE) untuk detail.

## Kontribusi

Kontribusi diterima! Silakan:

1. Fork repositori
2. Buat branch fitur
3. Kirim Pull Request

Branch master yang dilindungi memerlukan review PR.
