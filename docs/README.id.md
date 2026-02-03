# redirector

[English](../README.md) | [Русский](README.ru.md) | [中文](README.zh.md) | [हिंदी](README.hi.md) | [Español](README.es.md) | [Português](README.pt.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Polski](README.pl.md) | [Nederlands](README.nl.md) | [Italiano](README.it.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) | **Bahasa Indonesia** | [Tiếng Việt](README.vi.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

Layanan pengalihan URL yang aman dengan halaman antara dan tautan pendek berbasis hashid.

## Masalah

Berbagi URL panjang tidak praktis. Pemendek URL ada tetapi sering langsung mengalihkan, yang bisa menjadi risiko keamanan. Pengguna harus melihat ke mana mereka akan pergi sebelum dialihkan.

**redirector** menyediakan pengalihan yang aman dengan:
- Halaman antara yang menampilkan URL tujuan sebelum pengalihan
- Timer hitung mundur untuk kesadaran pengguna
- Halaman yang indah dan bermerek

## Fitur

- 🔗 **URL Hashid** - ID pendek, unik, non-sekuensial (mis. `/r/abc123`)
- ⏱️ **Halaman antara** - Hitung mundur menampilkan URL tujuan sebelum pengalihan
- ⚡ **Caching Redis** - Pencarian cepat dengan TTL yang dapat dikonfigurasi
- 🛡️ **Circuit breaker** - Perlindungan database dari kegagalan berantai
- 🚦 **Pembatasan laju** - Batas global dan tingkat database
- 📊 **Metrik Prometheus** - Observabilitas penuh dengan perlindungan Basic Auth
- 🎨 **Halaman indah** - Halaman 404 dan indeks yang bersih
- 🔑 **Banyak salt** - Dukungan rotasi salt hashid untuk migrasi

## Mulai Cepat

### Docker

```bash
docker run -p 8080:8080 \
  -v $(pwd)/config.yaml:/config.yaml \
  ghcr.io/brilliant-almazov/redirector:latest
```

## Cara Kerja

1. Pengguna mengunjungi `/r/{hashid}` (mis. `/r/abc123`)
2. Layanan mendekode hashid ke ID numerik
3. Memeriksa cache Redis untuk URL
4. Jika cache miss, kueri PostgreSQL
5. Cache hasil di Redis
6. Menampilkan halaman antara dengan hitung mundur
7. Setelah hitung mundur, alihkan ke URL tujuan

## Lisensi

Lisensi MIT - lihat [LICENSE](../LICENSE) untuk detail.
