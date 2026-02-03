# redirector

[English](../README.md) | [Русский](README.ru.md) | [中文](README.zh.md) | [हिंदी](README.hi.md) | [Español](README.es.md) | [Português](README.pt.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Polski](README.pl.md) | **Nederlands** | [Italiano](README.it.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) | [Bahasa Indonesia](README.id.md) | [Tiếng Việt](README.vi.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

Veilige URL-omleidingsservice met tussenpagina's en hashid-gebaseerde korte links.

## Probleem

Het delen van lange URL's is onhandig. URL-verkorters bestaan, maar leiden vaak direct door, wat een veiligheidsrisico kan zijn. Gebruikers moeten zien waar ze naartoe gaan voordat ze worden doorgestuurd.

**redirector** biedt veilige omleidingen met:
- Tussenpagina die doel-URL toont voor omleiding
- Afteltimer voor gebruikersbewustzijn
- Mooie, merkgebonden pagina's

## Functies

- 🔗 **Hashid URL's** - Korte, unieke, niet-sequentiële ID's (bijv. `/r/abc123`)
- ⏱️ **Tussenpagina** - Aftelling toont doel-URL voor omleiding
- ⚡ **Redis caching** - Snelle opzoekingen met configureerbare TTL
- 🛡️ **Circuit breaker** - Databasebescherming tegen cascade-fouten
- 🚦 **Rate limiting** - Globale en database-niveau limieten
- 📊 **Prometheus metrics** - Volledige observeerbaarheid met Basic Auth-bescherming
- 🎨 **Mooie pagina's** - Schone 404- en indexpagina's
- 🔑 **Meerdere salts** - Hashid salt-rotatie ondersteuning voor migratie

## Snel Starten

### Docker

```bash
docker run -p 8080:8080 \
  -v $(pwd)/config.yaml:/config.yaml \
  ghcr.io/brilliant-almazov/redirector:latest
```

## Hoe Het Werkt

1. Gebruiker bezoekt `/r/{hashid}` (bijv. `/r/abc123`)
2. Service decodeert hashid naar numeriek ID
3. Controleert Redis cache voor URL
4. Bij cache miss, query PostgreSQL
5. Cachet resultaat in Redis
6. Toont tussenpagina met aftelling
7. Na aftelling, omleiding naar doel-URL

## Licentie

MIT-licentie - zie [LICENSE](../LICENSE) voor details.
