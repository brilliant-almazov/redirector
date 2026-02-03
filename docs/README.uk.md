# redirector

[English](../README.md) | [Русский](README.ru.md) | [中文](README.zh.md) | [हिंदी](README.hi.md) | [Español](README.es.md) | [Português](README.pt.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Polski](README.pl.md) | [Nederlands](README.nl.md) | [Italiano](README.it.md) | [Türkçe](README.tr.md) | **Українська** | [Bahasa Indonesia](README.id.md) | [Tiếng Việt](README.vi.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

Безпечний сервіс перенаправлення URL з проміжними сторінками та короткими посиланнями на основі hashid.

## Проблема

Ділитися довгими URL незручно. Скорочувачі URL існують, але часто перенаправляють одразу, що може бути ризиком безпеки. Користувачі повинні бачити, куди вони йдуть, перед перенаправленням.

**redirector** забезпечує безпечні перенаправлення з:
- Проміжною сторінкою, що показує цільовий URL перед перенаправленням
- Таймером зворотного відліку для обізнаності користувача
- Красивими, брендованими сторінками

## Можливості

- 🔗 **Hashid URL** - Короткі, унікальні, непослідовні ID (напр. `/r/abc123`)
- ⏱️ **Проміжна сторінка** - Зворотний відлік показує цільовий URL перед перенаправленням
- ⚡ **Кешування Redis** - Швидкі запити з налаштовуваним TTL
- 🛡️ **Circuit breaker** - Захист бази даних від каскадних збоїв
- 🚦 **Обмеження швидкості** - Глобальні та на рівні бази даних
- 📊 **Метрики Prometheus** - Повна спостережуваність із захистом Basic Auth
- 🎨 **Красиві сторінки** - Чисті сторінки 404 та індексу
- 🔑 **Кілька солей** - Підтримка ротації солі hashid для міграції

## Швидкий Старт

### Docker

```bash
docker run -p 8080:8080 \
  -v $(pwd)/config.yaml:/config.yaml \
  ghcr.io/brilliant-almazov/redirector:latest
```

## Як Це Працює

1. Користувач відвідує `/r/{hashid}` (напр. `/r/abc123`)
2. Сервіс декодує hashid у числовий ID
3. Перевіряє кеш Redis на наявність URL
4. При промаху кешу запитує PostgreSQL
5. Кешує результат у Redis
6. Показує проміжну сторінку зі зворотним відліком
7. Після відліку перенаправляє на цільовий URL

## Ліцензія

Ліцензія MIT - деталі у [LICENSE](../LICENSE).
