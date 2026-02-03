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

## Giấy phép

Giấy phép MIT - xem [LICENSE](../LICENSE) để biết chi tiết.

## Đóng góp

Đóng góp được hoan nghênh! Vui lòng:

1. Fork repository
2. Tạo nhánh tính năng
3. Gửi Pull Request

Nhánh master được bảo vệ yêu cầu review PR.
