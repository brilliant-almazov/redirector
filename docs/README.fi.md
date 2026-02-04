# redirector

> **Korkean suorituskyvyn URL-lyhentäjä ja uudelleenohjauspalvelu** rakennettu Rustilla, Axumilla, Rediksellä ja PostgreSQL:llä. Sisältää turvalliset välisivut, reaaliaikaisen ylläpitopaneelin ja yritystason havainnoitavuuden.

[English](../README.md) | [Русский](README.ru.md) | [中文](README.zh.md) | [हिंदी](README.hi.md) | [Español](README.es.md) | [Português](README.pt.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Polski](README.pl.md) | [Nederlands](README.nl.md) | [Italiano](README.it.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) | [עברית](README.he.md) | [Bahasa Indonesia](README.id.md) | [Tiếng Việt](README.vi.md) | [Svenska](README.sv.md) | **Suomi**

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![Docker Image Size](https://ghcr-badge.egpl.dev/brilliant-almazov/redirector/size)](https://github.com/brilliant-almazov/redirector/pkgs/container/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

**Avainsanat**: URL-lyhentäjä, linkin lyhentäjä, uudelleenohjauspalvelu, Rust-verkkopalvelu, Axum-kehys, Redis-välimuisti, PostgreSQL, Prometheus-mittarit, hashids, lyhyet linkit, välisivut, turvalliset uudelleenohjaukset, korkea suorituskyky, mikropalvelu

Turvallinen URL-uudelleenohjauspalvelu välisivuilla ja hashid-pohjaisilla lyhyillä linkeillä. Täydellinen sisäisiin työkaluihin, yrityksen linkkien hallintaan ja brändättyihin lyhyt-URL-palveluihin.

### Suorituskyky

| Skenaario | RPS | Keskim. viive | P99 viive |
|-----------|-----|---------------|-----------|
| 100% Cache Hit | **7 800+** | ~14ms | ~50ms |
| Cache Miss (10K URLs) | **2 300+** | ~44ms | ~81ms |

**Testiolosuhteet**: wrk -t4 -c100 -d30s, PostgreSQL 15, Dragonfly (Redis), macOS M1 (Docker)

> ⚠️ Tulokset Docker-ympäristöstä macOS:llä VM-ylikuormituksella. Natiivi Linux-käyttöönotto on odotettavissa olevan **3-5x nopeampi**.

## Ongelma

Pitkien URL-osoitteiden jakaminen on epäkäytännöllistä. URL-lyhentäjiä on olemassa, mutta ne usein uudelleenohjaavat välittömästi, mikä voi olla turvallisuusriski. Käyttäjien tulisi nähdä minne he ovat menossa ennen uudelleenohjausta.

**redirector** tarjoaa turvalliset uudelleenohjaukset:
- Välisivu näyttää kohde-URL:n ennen uudelleenohjausta
- Lähtölaskenta-ajastin käyttäjän tietoisuudeksi
- Kauniit, brändätyt sivut

## Ominaisuudet

- 🔗 **Hashid-URL:t** - Lyhyet, yksilölliset, ei-peräkkäiset tunnisteet (esim. `/r/abc123`)
- ⏱️ **Välisivu** - Lähtölaskenta-ajastin näyttää kohde-URL:n ennen uudelleenohjausta
- ⚡ **Redis-välimuisti** - Nopeat haut konfiguroitavalla TTL:llä
- 🛡️ **Circuit breaker** - Tietokantasuojaus kaskadivirheiden varalta
- 🚦 **Nopeusrajoitus** - Sekä globaali että tietokantatasoinen nopeusrajoitus
- 📊 **Prometheus-mittarit** - Täysi havainnoitavuus Basic Auth -suojauksella
- 🎨 **Kauniit sivut** - Siistit 404- ja hakemistosivut 4 teemalla
- 🔑 **Useita suoloja** - Hashid-suolan kiertotuuki migraatiota varten
- 📱 **Ylläpitopaneeli** - Reaaliaikainen mittareiden seuranta SSE:llä

## Kuvakaappaukset

| Vaalea | Tumma | Harmaa | Lämmin |
|--------|-------|--------|--------|
| ![Dashboard Vaalea](screenshots/dashboard-light.png) | ![Dashboard Tumma](screenshots/dashboard-dark.png) | ![Dashboard Harmaa](screenshots/dashboard-gray.png) | ![Dashboard Lämmin](screenshots/dashboard-warm.png) |
| ![Kirjautuminen Vaalea](screenshots/login-light.png) | ![Kirjautuminen Tumma](screenshots/login-dark.png) | ![Kirjautuminen Harmaa](screenshots/login-gray.png) | ![Kirjautuminen Lämmin](screenshots/login-warm.png) |
| ![Index Light](screenshots/index-light.png) | ![Index Dark](screenshots/index-dark.png) | ![Index Gray](screenshots/index-gray.png) | ![Index Warm](screenshots/index-warm.png) |
| ![Interstitial Light](screenshots/interstitial-light.png) | ![Interstitial Dark](screenshots/interstitial-dark.png) | ![Interstitial Gray](screenshots/interstitial-gray.png) | ![Interstitial Warm](screenshots/interstitial-warm.png) |
| ![404 Light](screenshots/404-light.png) | ![404 Dark](screenshots/404-dark.png) | ![404 Gray](screenshots/404-gray.png) | ![404 Warm](screenshots/404-warm.png) |

### Kuormitustestin modaali

| Vaalea | Tumma | Harmaa | Lämmin |
|--------|-------|--------|--------|
| ![Modaali Vaalea](screenshots/dashboard-modal-light.png) | ![Modaali Tumma](screenshots/dashboard-modal-dark.png) | ![Modaali Harmaa](screenshots/dashboard-modal-gray.png) | ![Modaali Lämmin](screenshots/dashboard-modal-warm.png) |

## Teknologiapino

- **Kieli**: Rust (async Tokiolla)
- **Web-kehys**: Axum
- **Välimuisti**: Redis-yhteensopiva (Redis, Dragonfly, Valkey, KeyDB jne.)
- **Tietokanta**: PostgreSQL (vaihdettava tallennuskerros)
- **Mittarit**: Prometheus + metrics-rs
- **Salasanan hajautus**: Argon2

> **Huomautus**: Tallennus- ja välimuistikerrokset on abstrahoitu ja ne voidaan korvata millä tahansa yhteensopivalla tietolähteellä. Tällä hetkellä aktiivisessa kehityksessä.

## Pikaopas

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

#### Base64-konfiguraatio

Ympäristöihin, joissa konfiguraatiotiedostojen liittäminen ei ole mahdollista (esim. serverless, PaaS):

```bash
# Encode
cat config.yaml | base64

# Run with base64 config
CONFIG_BASE64="c2VydmVyOgogIGhvc3Q6IC..." docker run ghcr.io/brilliant-almazov/redirector:latest
```

## Miten se toimii

1. Käyttäjä vierailee `/r/{hashid}` (esim. `/r/abc123`)
2. Palvelu purkaa hashidin numeeriseksi tunnukseksi
3. Tarkistaa Redis-välimuistista URL:n
4. Välimuistihäviössä kyselee PostgreSQL:ää
5. Välimuistittaa tuloksen Redikseen
6. Näyttää välisivun lähtölaskennalla
7. Lähtölaskennan jälkeen uudelleenohjaa kohde-URL:iin

## Päätepisteet

| Päätepiste | Auth | Kuvaus |
|------------|------|--------|
| `GET /` | Ei | Hakemistosivu |
| `GET /r/{hashid}` | Ei | Uudelleenohjaus välisivulla |
| `GET /d/{hashid}` | Ei | Demo-uudelleenohjaus (synteettinen kuormitustestaus) |
| `GET /health` | Ei | Terveystarkistus |
| `GET /metrics` | Basic | Prometheus-mittarit |
| `GET /admin` | Session | Ylläpitopaneelin kirjautuminen |
| `GET /admin/dashboard` | Session | Ylläpitopaneeli |

## Ylläpitopaneeli

Palvelu sisältää valinnaisen ylläpitopaneelin reaaliaikaiseen mittareiden seurantaan.

### Asennus

1. **Luo salasanan hajautus:**

```bash
cargo run --bin hash_password
# Syötä salasana, tai:
cargo run --bin hash_password -- "salasanasi"
```

2. **Lisää config.yaml:iin:**

```yaml
admin:
  enabled: true
  session_ttl_hours: 24
  users:
    - username: admin
      password_hash: "$argon2id$v=19$m=19456,t=2,p=1$..."  # vaiheesta 1
```

3. **Avaa paneeli:**

Mene osoitteeseen `http://localhost:8080/admin` ja kirjaudu tunnuksillasi.

### Ominaisuudet

- Reaaliaikaiset RPS- ja viivekaaviot
- Järjestelmämittarit (CPU, muisti, käyttöaika)
- Välimuistin osumaprosentin seuranta
- Viimeaikaisten uudelleenohjausten lista
- Kuormitussimulaatio testaukseen
- Kolme teemaa: Vaalea, Tumma, Lämmin

## Lisenssi

MIT-lisenssi - katso [LICENSE](../LICENSE) yksityiskohdat.

## Osallistuminen

Osallistumiset ovat tervetulleita! Ole hyvä ja:

1. Forkkaa repositorio
2. Luo ominaisuushaara
3. Lähetä Pull Request

Suojattu master-haara vaatii PR-arvioinnin.
