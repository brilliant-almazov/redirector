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
