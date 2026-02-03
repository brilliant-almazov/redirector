# redirector

[English](../README.md) | [Русский](README.ru.md) | [中文](README.zh.md) | [हिंदी](README.hi.md) | [Español](README.es.md) | [Português](README.pt.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Polski](README.pl.md) | [Nederlands](README.nl.md) | [Italiano](README.it.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) | [Bahasa Indonesia](README.id.md) | [Tiếng Việt](README.vi.md) | [Svenska](README.sv.md) | **Suomi**

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

Turvallinen URL-uudelleenohauspalvelu välisivuilla ja hashid-pohjaisilla lyhytlinkeillä.

## Ongelma

Pitkien URL-osoitteiden jakaminen on hankalaa. URL-lyhentimiä on olemassa, mutta ne ohjaavat usein heti, mikä voi olla turvallisuusriski. Käyttäjien pitäisi nähdä minne he ovat menossa ennen uudelleenohjausta.

**redirector** tarjoaa turvallisia uudelleenohjauksia:
- Välisivu näyttää kohde-URL:n ennen uudelleenohjausta
- Lähtölaskenta käyttäjän tietoisuutta varten
- Kauniit, brändätyt sivut

## Ominaisuudet

- 🔗 **Hashid-URL:t** - Lyhyet, ainutlaatuiset, ei-peräkkäiset ID:t (esim. `/r/abc123`)
- ⏱️ **Välisivu** - Lähtölaskenta näyttää kohde-URL:n ennen uudelleenohjausta
- ⚡ **Redis-välimuisti** - Nopeat haut konfiguroitavalla TTL:llä
- 🛡️ **Circuit breaker** - Tietokantasuojaus kaskadivikoja vastaan
- 🚦 **Nopeusrajoitus** - Globaalit ja tietokantatasoiset rajat
- 📊 **Prometheus-mittarit** - Täysi havainnoitavuus Basic Auth -suojauksella
- 🎨 **Kauniit sivut** - Siistit 404- ja indeksisivut
- 🔑 **Useita suoloja** - Hashid-suolarotaatiotuki migraatiota varten

## Pikaopas

### Docker

```bash
docker run -p 8080:8080 \
  -v $(pwd)/config.yaml:/config.yaml \
  ghcr.io/brilliant-almazov/redirector:latest
```

## Miten Se Toimii

1. Käyttäjä vierailee `/r/{hashid}` (esim. `/r/abc123`)
2. Palvelu purkaa hashidin numeeriseksi ID:ksi
3. Tarkistaa Redis-välimuistista URL:n
4. Välimuistin ohi mennessä, kysyy PostgreSQL:stä
5. Tallentaa tuloksen Redis-välimuistiin
6. Näyttää välisivun lähtölaskennan kanssa
7. Lähtölaskennan jälkeen ohjaa kohde-URL:iin

## Lisenssi

MIT-lisenssi - katso [LICENSE](../LICENSE) yksityiskohtia varten.
