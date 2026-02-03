# redirector

[English](../README.md) | [Русский](README.ru.md) | [中文](README.zh.md) | [हिंदी](README.hi.md) | [Español](README.es.md) | [Português](README.pt.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Polski](README.pl.md) | [Nederlands](README.nl.md) | [Italiano](README.it.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) | [Bahasa Indonesia](README.id.md) | **Tiếng Việt** | [Svenska](README.sv.md) | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

Dịch vụ chuyển hướng URL an toàn với trang trung gian và liên kết ngắn dựa trên hashid.

## Vấn đề

Chia sẻ URL dài rất bất tiện. Các công cụ rút gọn URL tồn tại nhưng thường chuyển hướng ngay lập tức, có thể là rủi ro bảo mật. Người dùng nên thấy họ đang đi đâu trước khi được chuyển hướng.

**redirector** cung cấp chuyển hướng an toàn với:
- Trang trung gian hiển thị URL đích trước khi chuyển hướng
- Bộ đếm ngược để người dùng nhận biết
- Các trang đẹp, có thương hiệu

## Tính năng

- 🔗 **URL Hashid** - ID ngắn, duy nhất, không tuần tự (ví dụ: `/r/abc123`)
- ⏱️ **Trang trung gian** - Đếm ngược hiển thị URL đích trước khi chuyển hướng
- ⚡ **Bộ nhớ đệm Redis** - Tra cứu nhanh với TTL có thể cấu hình
- 🛡️ **Circuit breaker** - Bảo vệ cơ sở dữ liệu khỏi lỗi dây chuyền
- 🚦 **Giới hạn tốc độ** - Giới hạn toàn cục và cấp cơ sở dữ liệu
- 📊 **Metrics Prometheus** - Khả năng quan sát đầy đủ với bảo vệ Basic Auth
- 🎨 **Trang đẹp** - Các trang 404 và trang chủ gọn gàng
- 🔑 **Nhiều salt** - Hỗ trợ xoay salt hashid để di chuyển

## Bắt đầu Nhanh

### Docker

```bash
docker run -p 8080:8080 \
  -v $(pwd)/config.yaml:/config.yaml \
  ghcr.io/brilliant-almazov/redirector:latest
```

## Cách Hoạt động

1. Người dùng truy cập `/r/{hashid}` (ví dụ: `/r/abc123`)
2. Dịch vụ giải mã hashid thành ID số
3. Kiểm tra bộ nhớ đệm Redis cho URL
4. Nếu cache miss, truy vấn PostgreSQL
5. Lưu kết quả vào Redis
6. Hiển thị trang trung gian với đếm ngược
7. Sau khi đếm ngược, chuyển hướng đến URL đích

## Giấy phép

Giấy phép MIT - xem [LICENSE](../LICENSE) để biết chi tiết.
