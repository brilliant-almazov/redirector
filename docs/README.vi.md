# redirector

> **Dịch vụ rút gọn URL và chuyển hướng hiệu suất cao** được xây dựng với Rust, Axum, Redis và PostgreSQL. Bao gồm trang trung gian an toàn, bảng điều khiển quản trị thời gian thực và khả năng quan sát cấp doanh nghiệp.

[English](../README.md) | [Русский](README.ru.md) | [中文](README.zh.md) | [हिंदी](README.hi.md) | [Español](README.es.md) | [Português](README.pt.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Polski](README.pl.md) | [Nederlands](README.nl.md) | [Italiano](README.it.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) | [עברית](README.he.md) | [Bahasa Indonesia](README.id.md) | **Tiếng Việt** | [Svenska](README.sv.md) | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![Docker Image Size](https://ghcr-badge.egpl.dev/brilliant-almazov/redirector/size)](https://github.com/brilliant-almazov/redirector/pkgs/container/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

**Từ khóa**: rút gọn URL, rút gọn liên kết, dịch vụ chuyển hướng, dịch vụ web Rust, framework Axum, cache Redis, PostgreSQL, metrics Prometheus, hashids, liên kết ngắn, trang trung gian, chuyển hướng an toàn, hiệu suất cao, microservice

Dịch vụ chuyển hướng URL an toàn với trang trung gian và liên kết ngắn dựa trên hashid. Hoàn hảo cho công cụ nội bộ, quản lý liên kết doanh nghiệp và dịch vụ URL ngắn có thương hiệu.

### Hiệu suất

| Kịch bản | RPS | Độ trễ TB | Độ trễ P99 |
|----------|-----|-----------|------------|
| 100% Cache Hit | **7.800+** | ~14ms | ~50ms |
| Cache Miss (10K URLs) | **2.300+** | ~44ms | ~81ms |

**Điều kiện kiểm tra**: wrk -t4 -c100 -d30s, PostgreSQL 15, Dragonfly (Redis), macOS M1 (Docker)

> ⚠️ Kết quả từ Docker trên macOS với overhead VM. Triển khai Linux native dự kiến **nhanh hơn 3-5 lần**.

## Vấn đề

Chia sẻ URL dài rất bất tiện. Các công cụ rút gọn URL tồn tại nhưng thường chuyển hướng ngay lập tức, có thể là rủi ro bảo mật. Người dùng nên thấy họ đang đi đâu trước khi được chuyển hướng.

**redirector** cung cấp chuyển hướng an toàn với:
- Trang trung gian hiển thị URL đích trước khi chuyển hướng
- Đồng hồ đếm ngược để người dùng nhận thức
- Các trang đẹp và có thương hiệu

## Tính năng

- 🔗 **URL Hashid** - ID ngắn, duy nhất, không tuần tự (ví dụ: `/r/abc123`)
- ⏱️ **Trang trung gian** - Đồng hồ đếm ngược hiển thị URL đích trước chuyển hướng
- ⚡ **Caching Redis** - Tra cứu nhanh với TTL có thể cấu hình
- 🛡️ **Circuit breaker** - Bảo vệ cơ sở dữ liệu chống lỗi dây chuyền
- 🚦 **Giới hạn tốc độ** - Giới hạn tốc độ toàn cục và cấp cơ sở dữ liệu
- 📊 **Metrics Prometheus** - Khả năng quan sát đầy đủ với bảo vệ Basic Auth
- 🎨 **Trang đẹp** - Trang 404 và index sạch với 4 chủ đề
- 🔑 **Nhiều salt** - Hỗ trợ xoay vòng salt hashid để di chuyển
- 📱 **Bảng điều khiển admin** - Giám sát metrics thời gian thực với SSE
- 📤 **Phân tích sự kiện** - Xuất bản sự kiện RabbitMQ tùy chọn với consumer PostgreSQL

## Ảnh chụp màn hình

| Sáng | Tối | Xám | Ấm |
|------|-----|-----|-----|
| ![Dashboard Sáng](screenshots/dashboard-light.png) | ![Dashboard Tối](screenshots/dashboard-dark.png) | ![Dashboard Xám](screenshots/dashboard-gray.png) | ![Dashboard Ấm](screenshots/dashboard-warm.png) |
| ![Đăng nhập Sáng](screenshots/login-light.png) | ![Đăng nhập Tối](screenshots/login-dark.png) | ![Đăng nhập Xám](screenshots/login-gray.png) | ![Đăng nhập Ấm](screenshots/login-warm.png) |
| ![Index Light](screenshots/index-light.png) | ![Index Dark](screenshots/index-dark.png) | ![Index Gray](screenshots/index-gray.png) | ![Index Warm](screenshots/index-warm.png) |
| ![Interstitial Light](screenshots/interstitial-light.png) | ![Interstitial Dark](screenshots/interstitial-dark.png) | ![Interstitial Gray](screenshots/interstitial-gray.png) | ![Interstitial Warm](screenshots/interstitial-warm.png) |
| ![404 Light](screenshots/404-light.png) | ![404 Dark](screenshots/404-dark.png) | ![404 Gray](screenshots/404-gray.png) | ![404 Warm](screenshots/404-warm.png) |

### Modal kiểm tra tải

| Sáng | Tối | Xám | Ấm |
|------|-----|-----|-----|
| ![Modal Sáng](screenshots/dashboard-modal-light.png) | ![Modal Tối](screenshots/dashboard-modal-dark.png) | ![Modal Xám](screenshots/dashboard-modal-gray.png) | ![Modal Ấm](screenshots/dashboard-modal-warm.png) |

## Stack công nghệ

- **Ngôn ngữ**: Rust (async với Tokio)
- **Framework Web**: Axum
- **Cache**: Tương thích Redis (Redis, Dragonfly, Valkey, KeyDB, v.v.)
- **Cơ sở dữ liệu**: PostgreSQL (lớp lưu trữ có thể thay thế)
- **Metrics**: Prometheus + metrics-rs
- **Hàng đợi tin nhắn**: RabbitMQ (tùy chọn, cho phân tích sự kiện)
- **Hash mật khẩu**: Argon2

> **Lưu ý**: Các lớp lưu trữ và cache được trừu tượng hóa và có thể thay thế bằng bất kỳ nguồn dữ liệu tương thích nào. Hiện đang trong giai đoạn phát triển tích cực.

## Bắt đầu nhanh

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

### Biến môi trường

Có **ba cách** để cấu hình dịch vụ, được liệt kê theo thứ tự ưu tiên (cao nhất trước):

| Ưu tiên | Phương thức | Trường hợp sử dụng |
|---------|-------------|---------------------|
| 1 | Biến môi trường `REDIRECTOR__*` | Ghi đè các giá trị riêng lẻ |
| 2 | Biến PaaS tiêu chuẩn (`DATABASE_URL`, v.v.) | Nền tảng PaaS (Railway, Heroku, Render) |
| 3 | File cấu hình (`config.yaml` hoặc `CONFIG_BASE64`) | Cấu hình cơ sở |

#### Biến đặc biệt

| Biến | Mặc định | Mô tả |
|------|----------|-------|
| `CONFIG_PATH` | `config.yaml` | Đường dẫn đến file cấu hình YAML |
| `CONFIG_BASE64` | — | Cấu hình YAML mã hóa Base64 (ưu tiên hơn `CONFIG_PATH`) |

#### Biến môi trường PaaS tiêu chuẩn

Các biến này được tự động nhận diện và áp dụng. Hầu hết nền tảng PaaS thiết lập chúng cho bạn:

| Biến | Đường dẫn cấu hình | Ví dụ |
|------|---------------------|-------|
| `DATABASE_URL` | `database.url` | `postgres://user:pass@host:5432/db` |
| `REDIS_URL` | `redis.url` | `redis://host:6379` |
| `PORT` | `server.port` | `3000` |
| `HASHIDS_SALTS` | `hashids.salts` | `new-salt,old-salt` (phân cách bằng dấu phẩy) |

> **Quy tắc ưu tiên**: Nếu cả `DATABASE_URL` và `REDIRECTOR__DATABASE__URL` đều được thiết lập, phiên bản có tiền tố `REDIRECTOR__` sẽ được ưu tiên. Tương tự, `REDIRECTOR__HASHIDS__SALTS__0` có ưu tiên hơn `HASHIDS_SALTS`.

#### Biến môi trường có tiền tố (`REDIRECTOR__*`)

Bất kỳ giá trị cấu hình nào cũng có thể được ghi đè bằng tiền tố `REDIRECTOR__` với `__` (hai dấu gạch dưới) làm dấu phân cách lồng nhau. Dưới đây là **tham chiếu đầy đủ** của tất cả các biến có thể ghi đè:

##### Server

| Biến môi trường | Đường dẫn cấu hình | Mặc định | Mô tả |
|-----------------|---------------------|----------|-------|
| `REDIRECTOR__SERVER__HOST` | `server.host` | `0.0.0.0` | Địa chỉ gắn kết |
| `REDIRECTOR__SERVER__PORT` | `server.port` | `8080` | Cổng HTTP |

##### Hashids

| Biến môi trường | Đường dẫn cấu hình | Mặc định | Mô tả |
|-----------------|---------------------|----------|-------|
| `REDIRECTOR__HASHIDS__SALTS__0` | `hashids.salts[0]` | *bắt buộc* | Salt hashid chính |
| `REDIRECTOR__HASHIDS__SALTS__1` | `hashids.salts[1]` | — | Salt cũ (để di chuyển) |
| `REDIRECTOR__HASHIDS__MIN_LENGTH` | `hashids.min_length` | `6` | Độ dài hashid tối thiểu |

> **Mảng**: Các phần tử danh sách được đánh chỉ mục bằng `__0`, `__1`, `__2`, v.v. Để xoay vòng salt hashid, đặt `__0` cho salt mới và `__1` cho salt cũ.

##### Redis / Cache

| Biến môi trường | Đường dẫn cấu hình | Mặc định | Mô tả |
|-----------------|---------------------|----------|-------|
| `REDIRECTOR__REDIS__URL` | `redis.url` | *bắt buộc* | URL kết nối Redis |
| `REDIRECTOR__REDIS__CACHE_TTL_SECONDS` | `redis.cache_ttl_seconds` | `86400` | TTL cache (giây). `86400` = 24 giờ |

##### Cơ sở dữ liệu

| Biến môi trường | Đường dẫn cấu hình | Mặc định | Mô tả |
|-----------------|---------------------|----------|-------|
| `REDIRECTOR__DATABASE__URL` | `database.url` | *bắt buộc* | URL kết nối PostgreSQL |
| `REDIRECTOR__DATABASE__POOL__MAX_CONNECTIONS` | `database.pool.max_connections` | `3` | Kích thước pool kết nối |
| `REDIRECTOR__DATABASE__POOL__CONNECT_TIMEOUT_SECONDS` | `database.pool.connect_timeout_seconds` | `3` | Thời gian chờ kết nối (giây) |
| `REDIRECTOR__DATABASE__RATE_LIMIT__MAX_REQUESTS_PER_SECOND` | `database.rate_limit.max_requests_per_second` | `50` | Số truy vấn DB tối đa mỗi giây |
| `REDIRECTOR__DATABASE__CIRCUIT_BREAKER__FAILURE_THRESHOLD` | `database.circuit_breaker.failure_threshold` | `3` | Số lỗi liên tiếp trước khi mở mạch |
| `REDIRECTOR__DATABASE__CIRCUIT_BREAKER__RESET_TIMEOUT_SECONDS` | `database.circuit_breaker.reset_timeout_seconds` | `60` | Giây trước khi thử lại bán mở |
| `REDIRECTOR__DATABASE__QUERY__TABLE` | `database.query.table` | `dictionary.urls` | Tên bảng tra cứu URL |
| `REDIRECTOR__DATABASE__QUERY__ID_COLUMN` | `database.query.id_column` | `id` | Tên cột cho ID số |
| `REDIRECTOR__DATABASE__QUERY__URL_COLUMN` | `database.query.url_column` | `name` | Tên cột cho URL đích |

##### Trang trung gian

| Biến môi trường | Đường dẫn cấu hình | Mặc định | Mô tả |
|-----------------|---------------------|----------|-------|
| `REDIRECTOR__INTERSTITIAL__DELAY_SECONDS` | `interstitial.delay_seconds` | `5` | Đếm ngược trước chuyển hướng |

##### Metrics

| Biến môi trường | Đường dẫn cấu hình | Mặc định | Mô tả |
|-----------------|---------------------|----------|-------|
| `REDIRECTOR__METRICS__BASIC_AUTH__USERNAME` | `metrics.basic_auth.username` | *bắt buộc* | Tên người dùng cho endpoint `/metrics` |
| `REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD` | `metrics.basic_auth.password` | *bắt buộc* | Mật khẩu cho endpoint `/metrics` |

##### Giới hạn tốc độ (Toàn cục)

| Biến môi trường | Đường dẫn cấu hình | Mặc định | Mô tả |
|-----------------|---------------------|----------|-------|
| `REDIRECTOR__RATE_LIMIT__REQUESTS_PER_SECOND` | `rate_limit.requests_per_second` | `1000` | Số yêu cầu tối đa mỗi giây |
| `REDIRECTOR__RATE_LIMIT__BURST` | `rate_limit.burst` | `100` | Cho phép burst vượt giới hạn RPS |

##### Bảng điều khiển Admin

| Biến môi trường | Đường dẫn cấu hình | Mặc định | Mô tả |
|-----------------|---------------------|----------|-------|
| `REDIRECTOR__ADMIN__ENABLED` | `admin.enabled` | `false` | Bật bảng điều khiển admin |
| `REDIRECTOR__ADMIN__SESSION_SECRET` | `admin.session_secret` | `change-me-...` | Bí mật ký phiên (tối thiểu 32 ký tự) |
| `REDIRECTOR__ADMIN__SESSION_TTL_HOURS` | `admin.session_ttl_hours` | `24` | Thời gian sống phiên tính bằng giờ |

> **Lưu ý**: Người dùng admin (`admin.users`) với `username` và `password_hash` không thể thiết lập qua biến môi trường do cấu trúc phức tạp. Định nghĩa chúng trong file cấu hình hoặc `CONFIG_BASE64`.

#### Ví dụ theo nền tảng triển khai

**Railway / Render / Fly.io** (PaaS với cơ sở dữ liệu được quản lý):

```bash
# Các biến này thường được nền tảng tự động thiết lập:
DATABASE_URL=postgres://user:pass@host:5432/db
REDIS_URL=redis://host:6379
PORT=3000

# Thiết lập cấu hình qua base64:
CONFIG_BASE64=c2VydmVyOgogIGhvc3Q6IC...

# Hoặc ghi đè các giá trị riêng lẻ:
REDIRECTOR__HASHIDS__SALTS__0=my-secret-salt
REDIRECTOR__METRICS__BASIC_AUTH__USERNAME=prometheus
REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD=strong-password
REDIRECTOR__ADMIN__ENABLED=true
REDIRECTOR__ADMIN__SESSION_SECRET=random-32-byte-secret-for-sessions
```

**Docker Compose (ví dụ đầy đủ với tất cả ghi đè)**:

```yaml
services:
  redirector:
    image: ghcr.io/brilliant-almazov/redirector:latest
    ports:
      - "8080:8080"
    environment:
      # --- URL kết nối (kiểu PaaS) ---
      DATABASE_URL: "postgres://redirector:${DB_PASSWORD}@postgres:5432/redirector"
      REDIS_URL: "redis://redis:6379"

      # --- File cấu hình ---
      CONFIG_BASE64: "${CONFIG_BASE64}"

      # --- Server ---
      REDIRECTOR__SERVER__HOST: "0.0.0.0"
      REDIRECTOR__SERVER__PORT: "8080"

      # --- Salt hashid ---
      REDIRECTOR__HASHIDS__SALTS__0: "${HASHID_SALT}"        # salt chính
      REDIRECTOR__HASHIDS__SALTS__1: "${HASHID_SALT_OLD}"    # salt cũ để di chuyển
      REDIRECTOR__HASHIDS__MIN_LENGTH: "6"

      # --- Cache Redis ---
      REDIRECTOR__REDIS__CACHE_TTL_SECONDS: "43200"          # 12 giờ

      # --- Pool DB và khả năng phục hồi ---
      REDIRECTOR__DATABASE__POOL__MAX_CONNECTIONS: "5"
      REDIRECTOR__DATABASE__POOL__CONNECT_TIMEOUT_SECONDS: "5"
      REDIRECTOR__DATABASE__RATE_LIMIT__MAX_REQUESTS_PER_SECOND: "100"
      REDIRECTOR__DATABASE__CIRCUIT_BREAKER__FAILURE_THRESHOLD: "5"
      REDIRECTOR__DATABASE__CIRCUIT_BREAKER__RESET_TIMEOUT_SECONDS: "30"

      # --- Ánh xạ bảng tùy chỉnh ---
      REDIRECTOR__DATABASE__QUERY__TABLE: "public.short_urls"
      REDIRECTOR__DATABASE__QUERY__ID_COLUMN: "id"
      REDIRECTOR__DATABASE__QUERY__URL_COLUMN: "target_url"

      # --- Trang trung gian ---
      REDIRECTOR__INTERSTITIAL__DELAY_SECONDS: "3"

      # --- Xác thực metrics ---
      REDIRECTOR__METRICS__BASIC_AUTH__USERNAME: "prometheus"
      REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD: "${METRICS_PASSWORD}"

      # --- Giới hạn tốc độ toàn cục ---
      REDIRECTOR__RATE_LIMIT__REQUESTS_PER_SECOND: "2000"
      REDIRECTOR__RATE_LIMIT__BURST: "200"

      # --- Bảng điều khiển admin ---
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

**Docker thuần (một lệnh)**:

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

**Cài đặt tối thiểu (chỉ biến môi trường, không file cấu hình)**:

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

#### Xoay vòng Salt qua biến môi trường

Khi xoay vòng salt hashid, dịch vụ thử các salt theo thứ tự -- kết quả khớp đầu tiên thắng. Đặt salt mới trước để các liên kết mới sử dụng nó, và giữ salt cũ để tương thích ngược:

**Tùy chọn 1: Một biến với dấu phẩy phân cách** (khuyến nghị):

```bash
# Trước khi xoay vòng
HASHIDS_SALTS=original-salt

# Sau khi xoay vòng -- salt mới trước, salt cũ cho các liên kết hiện tại
HASHIDS_SALTS=new-salt,original-salt
```

**Tùy chọn 2: Biến có chỉ mục**:

```bash
# Trước khi xoay vòng
REDIRECTOR__HASHIDS__SALTS__0=original-salt

# Sau khi xoay vòng
REDIRECTOR__HASHIDS__SALTS__0=new-salt
REDIRECTOR__HASHIDS__SALTS__1=original-salt
```

> **Lưu ý**: Nếu `REDIRECTOR__HASHIDS__SALTS__0` được thiết lập, `HASHIDS_SALTS` sẽ bị bỏ qua.

#### Cấu hình Base64

Cho các môi trường không thể mount file cấu hình (PaaS, serverless, CI/CD), truyền toàn bộ cấu hình dưới dạng chuỗi mã hóa base64:

```bash
# Encode
cat config.yaml | base64

# Decode (để xác minh)
echo "$CONFIG_BASE64" | base64 -d
```

`CONFIG_BASE64` có ưu tiên hơn `CONFIG_PATH`. Ghi đè biến môi trường (`REDIRECTOR__*` và biến PaaS) được áp dụng **lên trên** cấu hình đã giải mã.

## Cách hoạt động

1. Người dùng truy cập `/r/{hashid}` (ví dụ: `/r/abc123`)
2. Dịch vụ giải mã hashid thành ID số
3. Kiểm tra cache Redis cho URL
4. Khi cache miss, truy vấn PostgreSQL
5. Cache kết quả vào Redis
6. Hiển thị trang trung gian với đếm ngược
7. Sau đếm ngược, chuyển hướng đến URL đích

## Endpoints

| Endpoint | Auth | Mô tả |
|----------|------|-------|
| `GET /` | Không | Trang chủ |
| `GET /r/{hashid}` | Không | Chuyển hướng với trang trung gian |
| `GET /d/{hashid}` | Không | Chuyển hướng demo (kiểm tra tải tổng hợp) |
| `GET /health` | Không | Kiểm tra sức khỏe |
| `GET /metrics` | Basic | Metrics Prometheus |
| `GET /admin` | Session | Đăng nhập bảng điều khiển admin |
| `GET /admin/dashboard` | Session | Bảng điều khiển admin |

## Bảng điều khiển Admin

Dịch vụ bao gồm bảng điều khiển admin tùy chọn để giám sát metrics thời gian thực.

### Cài đặt

1. **Tạo hash mật khẩu:**

```bash
cargo run --bin hash_password
# Nhập mật khẩu, hoặc:
cargo run --bin hash_password -- "your-password"
```

2. **Thêm vào config.yaml:**

```yaml
admin:
  enabled: true
  session_ttl_hours: 24
  users:
    - username: admin
      password_hash: "$argon2id$v=19$m=19456,t=2,p=1$..."  # từ bước 1
```

3. **Truy cập bảng điều khiển:**

Mở `http://localhost:8080/admin` và đăng nhập bằng thông tin xác thực của bạn.

### Tính năng

- Biểu đồ RPS và độ trễ thời gian thực
- Metrics hệ thống (CPU, bộ nhớ, uptime)
- Giám sát tỷ lệ cache hit
- Danh sách chuyển hướng gần đây
- Mô phỏng tải cho kiểm tra
- Ba chủ đề: Sáng, Tối, Ấm

## Phân tích sự kiện

Đường ống xuất bản sự kiện tùy chọn cho phân tích chuyển hướng. Khi được bật, mỗi sự kiện chuyển hướng được xuất bản lên RabbitMQ và được một nhị phân riêng biệt tiêu thụ để ghi vào PostgreSQL với làm giàu dữ liệu.

> **Tài liệu đầy đủ**: [docs/EVENT_ANALYTICS.md](../EVENT_ANALYTICS.md)

### Tính năng

- **Xuất bản Fire-and-forget** — độ trễ chuyển hướng không bị ảnh hưởng bởi sự khả dụng của hàng đợi
- **Phân loại theo lô** — sự kiện được nhóm theo kích thước (100) hoặc thời gian (1 giây)
- **Phân tích User-Agent** — trình duyệt, phiên bản, hệ điều hành, loại thiết bị qua woothee
- **Làm giàu GeoIP** — quốc gia và thành phố từ IP (MaxMind mmdb với hot-reload)
- **Khử trùng tham chiếu** — MD5-based dedup cho referer và user agent
- **Phân chia hàng tháng** — tạo partition tự động cho `redirect_events`
- **Chuẩn hóa miền** — `WWW.Example.COM` → `example.com`

### Kiến trúc

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

### Bắt đầu nhanh

```bash
# Bật trong config.yaml
events:
  enabled: true
  rabbitmq:
    url: amqp://guest:guest@localhost:5672/%2f

# Hoặc qua biến môi trường
REDIRECTOR__EVENTS__ENABLED=true
RABBITMQ_URL=amqp://guest:guest@localhost:5672/%2f

# Chạy consumer
RABBITMQ_URL=amqp://... DATABASE_URL=postgres://... cargo run --bin event_consumer
```

### Docker Compose với Events

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
      - GEOIP_DB_PATH=/data/GeoLite2-City.mmdb  # optional
    depends_on: [rabbitmq, analytics-db]

  rabbitmq:
    image: rabbitmq:4-management-alpine
    ports: ["5672:5672", "15672:15672"]

  analytics-db:
    image: postgres:16-alpine
    environment:
      POSTGRES_DB: analytics
```

### Quyết định thiết kế chính

- **Không bao giờ chặn chuyển hướng**: `try_send()` trên bounded channel, bỏ sự kiện nếu đầy
- **Sự kiện lô an toàn về kiểu**: `EventBatch` là enum Rust được gắn nhãn bằng `event_type`
- **Snowflake batch IDs**: Epoch tùy chỉnh 2025-01-01, ~69 năm ID duy nhất
- **Suy giảm Graceful**: Nếu RabbitMQ ngừng hoạt động, chuyển hướng tiếp tục; sự kiện bị bỏ với metrics

## Metrics

Dịch vụ hiển thị các metrics Prometheus toàn diện tại `/metrics` (yêu cầu Basic Auth):

### Service Metrics
```
redirector_up 1
redirector_build_info{version="0.1.0"} 1
redirector_uptime_seconds 3600.5
```

### Request Metrics
```
redirect_requests_total 150000
not_found_requests_total 50
request_duration_seconds{quantile="0.5"} 0.040
request_duration_seconds{quantile="0.99"} 0.081
```

### Cache Metrics
```
cache_hits_total 140000
cache_misses_total 10000
cache_get_duration_seconds{quantile="0.5"} 0.002
cache_set_duration_seconds{quantile="0.5"} 0.002
```

### Database Metrics
```
db_queries_total 10000
db_hits_total 9950
db_misses_total 50
db_errors_total 0
```

## Giấy phép

Giấy phép MIT - xem [LICENSE](../LICENSE) để biết chi tiết.

## Đóng góp

Đóng góp được hoan nghênh! Vui lòng:

1. Fork repository
2. Tạo nhánh tính năng
3. Gửi Pull Request

Nhánh master được bảo vệ yêu cầu review PR.
