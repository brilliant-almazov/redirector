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
- 📤 **Tapahtumaanalytiikka** - Valinnainen RabbitMQ-tapahtumajulkaisu PostgreSQL-kuluttajalla

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
- **Viestijono**: RabbitMQ (valinnainen, tapahtumaanalytiikkaan)
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

## Konfiguraatio

Luo `config.yaml`:

```yaml
server:
  host: "0.0.0.0"
  port: 8080

hashids:
  salts:
    - ${HASHID_SALT}          # Ensisijainen suola
    - ${HASHID_SALT_OLD}      # Valinnainen: vanha suola migraatiota varten
  min_length: 6

redis:
  url: ${REDIS_URL}
  cache_ttl_seconds: 86400    # 24 tuntia

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
    table: "dictionary.urls"    # Taulun nimi
    id_column: "id"             # ID-sarake
    url_column: "name"          # URL-sarake

interstitial:
  delay_seconds: 5            # Lähtölaskenta ennen uudelleenohjausta

metrics:
  basic_auth:
    username: prometheus
    password: ${METRICS_PASSWORD}

rate_limit:
  requests_per_second: 1000
  burst: 100
```

### Konfiguraatiovaihtoehdot

#### Palvelin

| Vaihtoehto | Oletus | Kuvaus |
|------------|--------|--------|
| `host` | `0.0.0.0` | Sidonta-osoite |
| `port` | `8080` | HTTP-portti |

#### Hashids

| Vaihtoehto | Oletus | Kuvaus |
|------------|--------|--------|
| `salts` | *pakollinen* | Lista hashid-suoloista (ensimmäinen = ensisijainen) |
| `min_length` | `6` | Hashidin vähimmäispituus |

#### Redis

| Vaihtoehto | Oletus | Kuvaus |
|------------|--------|--------|
| `url` | *pakollinen* | Redis-yhteys-URL |
| `cache_ttl_seconds` | `86400` | Välimuistin TTL sekunteina |

#### Tietokanta

| Vaihtoehto | Oletus | Kuvaus |
|------------|--------|--------|
| `url` | *pakollinen* | PostgreSQL-yhteys-URL |
| `pool.max_connections` | `3` | Yhteyspoolinkoko |
| `pool.connect_timeout_seconds` | `3` | Yhteysaikakatkaisu |
| `rate_limit.max_requests_per_second` | `50` | Tietokannan nopeusrajoitus |
| `circuit_breaker.failure_threshold` | `3` | Virhettä ennen avaamista |
| `circuit_breaker.reset_timeout_seconds` | `60` | Piirin nollausaikakatkaisu |

#### Nopeusrajoitus (globaali)

| Vaihtoehto | Oletus | Kuvaus |
|------------|--------|--------|
| `requests_per_second` | `1000` | Globaali nopeusrajoitus |
| `burst` | `100` | Purskeen kapasiteetti |

### Ympäristömuuttujat

Palvelun konfigurointiin on **kolme tapaa**, prioriteettijärjestyksessä (korkein ensin):

| Prioriteetti | Tapa | Käyttötapaus |
|--------------|------|--------------|
| 1 | `REDIRECTOR__*` ympäristömuuttujat | Yksittäisten arvojen ylikirjoitus |
| 2 | Standardit PaaS-muuttujat (`DATABASE_URL` jne.) | PaaS-alustat (Railway, Heroku, Render) |
| 3 | Konfiguraatiotiedosto (`config.yaml` tai `CONFIG_BASE64`) | Peruskonfiguraatio |

#### Erikoismuuttujat

| Muuttuja | Oletus | Kuvaus |
|----------|--------|--------|
| `CONFIG_PATH` | `config.yaml` | Polku YAML-konfiguraatiotiedostoon |
| `CONFIG_BASE64` | — | Base64-koodattu YAML-konfiguraatio (etusija `CONFIG_PATH`:iin nähden) |

#### Standardit PaaS-ympäristömuuttujat

Nämä tunnistetaan ja sovelletaan automaattisesti. Useimmat PaaS-alustat asettavat ne puolestasi:

| Muuttuja | Konfiguraatiopolku | Esimerkki |
|----------|---------------------|-----------|
| `DATABASE_URL` | `database.url` | `postgres://user:pass@host:5432/db` |
| `REDIS_URL` | `redis.url` | `redis://host:6379` |
| `PORT` | `server.port` | `3000` |

> **Prioriteettisääntö**: Jos sekä `DATABASE_URL` että `REDIRECTOR__DATABASE__URL` on asetettu, `REDIRECTOR__`-etuliitteinen versio voittaa.

#### Etuliitteelliset ympäristömuuttujat (`REDIRECTOR__*`)

Mikä tahansa konfiguraatioarvo voidaan ylikirjoittaa käyttämällä `REDIRECTOR__`-etuliitettä ja `__` (kaksinkertainen alaviiva) sisäkkäisyyden erottimena:

```
YAML-konfiguraatiopolku     →  Ympäristömuuttuja
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

#### Esimerkit käyttöönottualustan mukaan

**Railway / Render / Fly.io** (PaaS hallituilla tietokannoilla):

```bash
# Nämä asetetaan yleensä automaattisesti alustan toimesta:
DATABASE_URL=postgres://user:pass@host:5432/db
REDIS_URL=redis://host:6379
PORT=3000

# Aseta konfiguraatio base64:n kautta:
CONFIG_BASE64=c2VydmVyOgogIGhvc3Q6IC...

# Tai ylikirjoita yksittäisiä arvoja:
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
      # Tai ylikirjoita yksittäisiä arvoja konfiguraatiotiedoston päälle:
      REDIRECTOR__RATE_LIMIT__REQUESTS_PER_SECOND: "2000"
      REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD: "${METRICS_PASSWORD}"
    volumes:
      - ./config.yaml:/app/config.yaml  # valinnainen CONFIG_BASE64:n kanssa
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

**Pelkkä Docker**:

```bash
docker run -p 8080:8080 \
  -e DATABASE_URL="postgres://user:pass@host:5432/db" \
  -e REDIS_URL="redis://host:6379" \
  -e CONFIG_BASE64="$(cat config.yaml | base64)" \
  ghcr.io/brilliant-almazov/redirector:latest
```

**Minimaalinen asennus (vain ympäristömuuttujat, ei konfiguraatiotiedostoa)**:

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

#### Base64-konfiguraatio

Ympäristöihin, joissa konfiguraatiotiedostojen liittäminen ei ole käytännöllistä (PaaS, serverless, CI/CD), välitä koko konfiguraatio Base64-koodattuna merkkijonona:

```bash
# Koodaa
cat config.yaml | base64

# Purkaa (tarkistusta varten)
echo "$CONFIG_BASE64" | base64 -d
```

`CONFIG_BASE64` on etusijalla `CONFIG_PATH`:iin nähden. Ympäristömuuttujien ylikirjoitukset (`REDIRECTOR__*` ja PaaS-muuttujat) sovelletaan puretun konfiguraation **päälle**.

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

## Tapahtuma-analytiikka

Valinnainen tapahtumajulkaisun putkilinja uudelleenohjauksien analytiikkaa varten. Kun funktio on käytössä, jokainen uudelleenohjaustapahtumaoletus julkaistaan RabbitMQ:hun ja kulutetaan erillisen binäärin kautta, joka kirjoittaa PostgreSQL:ään rikastettujen tietojen kanssa.

> **Täydellinen dokumentaatio**: [docs/EVENT_ANALYTICS.md](EVENT_ANALYTICS.md)

### Ominaisuudet

- **Fire-and-forget-julkaisu** — uudelleenohjauksen latenssi ei kärsi jonon saatavuudesta
- **Erätöyntö** — tapahtumat ryhmitelty koon (100) tai ajan (1 sekunti) mukaan
- **User-Agent-jäsennys** — selain, versio, käyttöjärjestelmä, laitteen tyyppi wootheen kautta
- **GeoIP-rikastaminen** — maa ja kaupunki IP-osoitteesta (MaxMind mmdb ja kuuman lataamisen kanssa)
- **Viitteen poistaminen kaksinkertaistumisesta** — MD5-pohjainen deduplikaatio viitteille ja käyttäjän agenteille
- **Kuukausittain osiointi** — automaattinen osiointien luominen `redirect_events`-taulukossa
- **Verkkotunnuksen normalisointi** — `WWW.Example.COM` → `example.com`

### Arkkitehtuuri

```
Uudelleenohjaus-käsittelijä
    │
    ├── try_send(RedirectEvent) ──► [tokio::mpsc-kanava]
    │   (ei-estävä,                    │
    │    fire-and-forget)                 ▼
    │                              Taustatehtävä
    │                              (erätöyntö koon/ajan mukaan)
    │                                     │
    │                                     ▼
    │                                [RabbitMQ-jono]
    │                                     │
    │                                     ▼
    │                              Tapahtumien kuluttaja
    │                              (erillinen binääri/säilö)
    │                                     │
    │                                     ▼
    │                              [PostgreSQL-analytiikka]
    │                              (kuukausittain osioitu)
```

### Pikaopas

```bash
# Ota käyttöön config.yaml:issa
events:
  enabled: true
  rabbitmq:
    url: amqp://guest:guest@localhost:5672/%2f

# Tai ympäristömuuttujan kautta
REDIRECTOR__EVENTS__ENABLED=true
RABBITMQ_URL=amqp://guest:guest@localhost:5672/%2f

# Suorita kuluttaja
RABBITMQ_URL=amqp://... DATABASE_URL=postgres://... cargo run --bin event_consumer
```

### Docker Compose tapahtumien kanssa

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
      - GEOIP_DB_PATH=/data/GeoLite2-City.mmdb  # valinnainen
    depends_on: [rabbitmq, analytics-db]

  rabbitmq:
    image: rabbitmq:4-management-alpine
    ports: ["5672:5672", "15672:15672"]

  analytics-db:
    image: postgres:16-alpine
    environment:
      POSTGRES_DB: analytics
```

### Tapahtumien mittarit

Kun tapahtumajulkaisu on käytössä, palvelu paljastaa lisämittareita osoitteessa `/metrics`:

```
events_published 50000
events_dropped 0
events_publish_errors 0
events_serialize_errors 0
rabbitmq_connected 1
```

## Lisenssi

MIT-lisenssi - katso [LICENSE](../LICENSE) yksityiskohdat.

## Osallistuminen

Osallistumiset ovat tervetulleita! Ole hyvä ja:

1. Forkkaa repositorio
2. Luo ominaisuushaara
3. Lähetä Pull Request

Suojattu master-haara vaatii PR-arvioinnin.
