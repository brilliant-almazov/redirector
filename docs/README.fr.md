# redirector

> **Raccourcisseur d'URL et service de redirection haute performance** construit avec Rust, Axum, Redis et PostgreSQL. Inclut des pages interstitielles sécurisées, un tableau de bord d'administration en temps réel et une observabilité de niveau entreprise.

[English](../README.md) | [Русский](README.ru.md) | [中文](README.zh.md) | [हिंदी](README.hi.md) | [Español](README.es.md) | [Português](README.pt.md) | **Français** | [Deutsch](README.de.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Polski](README.pl.md) | [Nederlands](README.nl.md) | [Italiano](README.it.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) | [עברית](README.he.md) | [Bahasa Indonesia](README.id.md) | [Tiếng Việt](README.vi.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![Docker Image Size](https://ghcr-badge.egpl.dev/brilliant-almazov/redirector/size)](https://github.com/brilliant-almazov/redirector/pkgs/container/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

**Mots-clés**: raccourcisseur d'URL, raccourcisseur de liens, service de redirection, service web Rust, framework Axum, cache Redis, PostgreSQL, métriques Prometheus, hashids, liens courts, pages interstitielles, redirections sécurisées, haute performance, microservice

Service de redirection d'URL sécurisé avec pages interstitielles et liens courts basés sur hashid. Parfait pour les outils internes, la gestion de liens d'entreprise et les services d'URL courtes de marque.

### Performance

| Scénario | RPS | Latence Moy | Latence P99 |
|----------|-----|-------------|-------------|
| 100% Cache Hit | **7 800+** | ~14ms | ~50ms |
| Cache Miss (10K URLs) | **2 300+** | ~44ms | ~81ms |

**Conditions de test**: wrk -t4 -c100 -d30s, PostgreSQL 15, Dragonfly (Redis), macOS M1 (Docker)

> ⚠️ Les résultats proviennent de Docker sur macOS avec surcharge VM. Le déploiement Linux natif devrait être **3-5x plus rapide**.

## Problème

Partager de longues URLs est peu pratique. Les raccourcisseurs d'URL existent mais redirigent souvent immédiatement, ce qui peut présenter un risque de sécurité. Les utilisateurs devraient voir où ils vont avant d'être redirigés.

**redirector** fournit des redirections sécurisées avec :
- Page interstitielle montrant l'URL cible avant la redirection
- Minuterie de compte à rebours pour sensibiliser l'utilisateur
- Pages belles et personnalisées

## Fonctionnalités

- 🔗 **URLs Hashid** - IDs courts, uniques et non-séquentiels (ex. `/r/abc123`)
- ⏱️ **Page interstitielle** - Compte à rebours montrant l'URL cible avant redirection
- ⚡ **Cache Redis** - Recherches rapides avec TTL configurable
- 🛡️ **Circuit breaker** - Protection de base de données contre les pannes en cascade
- 🚦 **Limitation de débit** - Limites globales et au niveau base de données
- 📊 **Métriques Prometheus** - Observabilité complète avec protection Basic Auth
- 🎨 **Belles pages** - Pages 404 et index propres avec 4 thèmes
- 🔑 **Sels multiples** - Support de rotation de sel hashid pour migration
- 📱 **Tableau de bord admin** - Surveillance des métriques en temps réel avec SSE
- 📤 **Analytique d'événements** - Publication optionnelle d'événements vers RabbitMQ avec consommateur PostgreSQL

## Captures d'écran

| Clair | Sombre | Gris | Chaud |
|-------|--------|------|-------|
| ![Dashboard Clair](screenshots/dashboard-light.png) | ![Dashboard Sombre](screenshots/dashboard-dark.png) | ![Dashboard Gris](screenshots/dashboard-gray.png) | ![Dashboard Chaud](screenshots/dashboard-warm.png) |
| ![Login Clair](screenshots/login-light.png) | ![Login Sombre](screenshots/login-dark.png) | ![Login Gris](screenshots/login-gray.png) | ![Login Chaud](screenshots/login-warm.png) |
| ![Index Light](screenshots/index-light.png) | ![Index Dark](screenshots/index-dark.png) | ![Index Gray](screenshots/index-gray.png) | ![Index Warm](screenshots/index-warm.png) |
| ![Interstitial Light](screenshots/interstitial-light.png) | ![Interstitial Dark](screenshots/interstitial-dark.png) | ![Interstitial Gray](screenshots/interstitial-gray.png) | ![Interstitial Warm](screenshots/interstitial-warm.png) |
| ![404 Light](screenshots/404-light.png) | ![404 Dark](screenshots/404-dark.png) | ![404 Gray](screenshots/404-gray.png) | ![404 Warm](screenshots/404-warm.png) |

### Modal de test de charge

| Clair | Sombre | Gris | Chaud |
|-------|--------|------|-------|
| ![Modal Clair](screenshots/dashboard-modal-light.png) | ![Modal Sombre](screenshots/dashboard-modal-dark.png) | ![Modal Gris](screenshots/dashboard-modal-gray.png) | ![Modal Chaud](screenshots/dashboard-modal-warm.png) |

## Stack technologique

- **Langage**: Rust (async avec Tokio)
- **Framework web**: Axum
- **Cache**: Compatible Redis (Redis, Dragonfly, Valkey, KeyDB, etc.)
- **Base de données**: PostgreSQL (couche de stockage interchangeable)
- **Métriques**: Prometheus + metrics-rs
- **File de messages**: RabbitMQ (optionnel, pour l'analytique d'événements)
- **Hachage de mots de passe**: Argon2

> **Note**: Les couches de stockage et de cache sont abstraites et peuvent être remplacées par n'importe quelle source de données compatible. Actuellement en développement actif.

## Démarrage rapide

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

## Configuration

Créez `config.yaml` :

```yaml
server:
  host: "0.0.0.0"
  port: 8080

hashids:
  salts:
    - ${HASHID_SALT}          # Sel principal
    - ${HASHID_SALT_OLD}      # Optionnel : ancien sel pour migration
  min_length: 6

redis:
  url: ${REDIS_URL}
  cache_ttl_seconds: 86400    # 24 heures

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
    table: "dictionary.urls"    # Nom de votre table
    id_column: "id"             # Colonne ID
    url_column: "name"          # Colonne URL

interstitial:
  delay_seconds: 5            # Compte à rebours avant redirection

metrics:
  basic_auth:
    username: prometheus
    password: ${METRICS_PASSWORD}

rate_limit:
  requests_per_second: 1000
  burst: 100
```

### Options de configuration

#### Serveur

| Option | Par défaut | Description |
|--------|------------|-------------|
| `host` | `0.0.0.0` | Adresse de liaison |
| `port` | `8080` | Port HTTP |

#### Hashids

| Option | Par défaut | Description |
|--------|------------|-------------|
| `salts` | *requis* | Liste des sels hashid (premier = principal) |
| `min_length` | `6` | Longueur minimale du hashid |

#### Redis

| Option | Par défaut | Description |
|--------|------------|-------------|
| `url` | *requis* | URL de connexion Redis |
| `cache_ttl_seconds` | `86400` | TTL du cache en secondes |

#### Base de données

| Option | Par défaut | Description |
|--------|------------|-------------|
| `url` | *requis* | URL de connexion PostgreSQL |
| `pool.max_connections` | `3` | Taille du pool de connexions |
| `pool.connect_timeout_seconds` | `3` | Timeout de connexion |
| `rate_limit.max_requests_per_second` | `50` | Limite de débit BD |
| `circuit_breaker.failure_threshold` | `3` | Échecs avant ouverture |
| `circuit_breaker.reset_timeout_seconds` | `60` | Timeout de réinitialisation du circuit |

#### Limite de débit (Globale)

| Option | Par défaut | Description |
|--------|------------|-------------|
| `requests_per_second` | `1000` | Limite de débit globale |
| `burst` | `100` | Capacité de rafale |

### Variables d'environnement

Il existe **trois méthodes** pour configurer le service, classées par priorité (la plus haute en premier) :

| Priorité | Méthode | Cas d'utilisation |
|----------|---------|-------------------|
| 1 | Variables `REDIRECTOR__*` | Remplacer des valeurs individuelles |
| 2 | Variables PaaS standard (`DATABASE_URL`, etc.) | Plateformes PaaS (Railway, Heroku, Render) |
| 3 | Fichier de configuration (`config.yaml` ou `CONFIG_BASE64`) | Configuration de base |

#### Variables spéciales

| Variable | Par défaut | Description |
|----------|------------|-------------|
| `CONFIG_PATH` | `config.yaml` | Chemin vers le fichier de configuration YAML |
| `CONFIG_BASE64` | — | Configuration YAML encodée en Base64 (priorité sur `CONFIG_PATH`) |

#### Variables d'environnement PaaS standard

Elles sont automatiquement reconnues et appliquées. La plupart des plateformes PaaS les définissent pour vous :

| Variable | Chemin de configuration | Exemple |
|----------|------------------------|---------|
| `DATABASE_URL` | `database.url` | `postgres://user:pass@host:5432/db` |
| `REDIS_URL` | `redis.url` | `redis://host:6379` |
| `PORT` | `server.port` | `3000` |

> **Règle de priorité** : Si `DATABASE_URL` et `REDIRECTOR__DATABASE__URL` sont tous deux définis, la version préfixée `REDIRECTOR__` l'emporte.

#### Variables préfixées (`REDIRECTOR__*`)

Toute valeur de configuration peut être remplacée en utilisant le préfixe `REDIRECTOR__` avec `__` (double tiret bas) comme séparateur d'imbrication :

```
Chemin YAML de configuration →  Variable d'environnement
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

#### Exemples par plateforme de déploiement

**Railway / Render / Fly.io** (PaaS avec bases de données gérées) :

```bash
# Celles-ci sont généralement définies automatiquement par la plateforme :
DATABASE_URL=postgres://user:pass@host:5432/db
REDIS_URL=redis://host:6379
PORT=3000

# Définissez votre config via base64 :
CONFIG_BASE64=c2VydmVyOgogIGhvc3Q6IC...

# Ou remplacez des valeurs individuelles :
REDIRECTOR__HASHIDS__SALTS__0=my-secret-salt
REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD=strong-password
```

**Docker / Docker Compose** :

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
      # Ou remplacez des valeurs individuelles en plus du fichier de config :
      REDIRECTOR__RATE_LIMIT__REQUESTS_PER_SECOND: "2000"
      REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD: "${METRICS_PASSWORD}"
    volumes:
      - ./config.yaml:/app/config.yaml  # optionnel si CONFIG_BASE64 est utilisé
    depends_on:
      - postgres
      - redis
```

**Kubernetes** :

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

**Docker simple** :

```bash
docker run -p 8080:8080 \
  -e DATABASE_URL="postgres://user:pass@host:5432/db" \
  -e REDIS_URL="redis://host:6379" \
  -e CONFIG_BASE64="$(cat config.yaml | base64)" \
  ghcr.io/brilliant-almazov/redirector:latest
```

**Configuration minimale (variables d'env uniquement, sans fichier de config)** :

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

#### Configuration Base64

Pour les environnements où le montage de fichiers de configuration n'est pas pratique (PaaS, serverless, CI/CD), passez toute la configuration sous forme de chaîne encodée en Base64 :

```bash
# Encoder
cat config.yaml | base64

# Décoder (pour vérifier)
echo "$CONFIG_BASE64" | base64 -d
```

`CONFIG_BASE64` a priorité sur `CONFIG_PATH`. Les remplacements de variables d'environnement (`REDIRECTOR__*` et variables PaaS) sont appliqués **par-dessus** la configuration décodée.

## Comment ça fonctionne

1. L'utilisateur visite `/r/{hashid}` (ex. `/r/abc123`)
2. Le service décode le hashid en ID numérique
3. Vérifie le cache Redis pour l'URL
4. En cas de miss cache, interroge PostgreSQL
5. Met en cache le résultat dans Redis
6. Affiche la page interstitielle avec compte à rebours
7. Après le compte à rebours, redirige vers l'URL cible

## Endpoints

| Endpoint | Auth | Description |
|----------|------|-------------|
| `GET /` | Non | Page d'accueil |
| `GET /r/{hashid}` | Non | Redirection avec page interstitielle |
| `GET /d/{hashid}` | Non | Redirection démo (test de charge synthétique) |
| `GET /health` | Non | Vérification de santé |
| `GET /metrics` | Basic | Métriques Prometheus |
| `GET /admin` | Session | Connexion tableau de bord admin |
| `GET /admin/dashboard` | Session | Tableau de bord admin |

## Tableau de bord admin

Le service inclut un tableau de bord admin optionnel pour surveiller les métriques en temps réel.

### Configuration

1. **Générer le hash du mot de passe:**

```bash
cargo run --bin hash_password
# Entrez le mot de passe, ou:
cargo run --bin hash_password -- "your-password"
```

2. **Ajouter à config.yaml:**

```yaml
admin:
  enabled: true
  session_ttl_hours: 24
  users:
    - username: admin
      password_hash: "$argon2id$v=19$m=19456,t=2,p=1$..."  # de l'étape 1
```

3. **Accéder au tableau de bord:**

Ouvrez `http://localhost:8080/admin` et connectez-vous avec vos identifiants.

### Fonctionnalités

- Graphiques RPS et latence en temps réel
- Métriques système (CPU, mémoire, uptime)
- Surveillance du taux de cache hit
- Liste des redirections récentes
- Simulation de charge pour tests
- Trois thèmes: Clair, Sombre, Chaud

## Analyse d'événements

Pipeline optionnel de publication d'événements pour l'analyse des redirections. Lorsqu'il est activé, chaque événement de redirection est publié sur RabbitMQ et traité par un binaire séparé qui écrit dans PostgreSQL.

> **Documentation complète**: [EVENT_ANALYTICS.md](EVENT_ANALYTICS.md)

### Fonctionnalités

- **Publication fire-and-forget** — La latence de redirection n'est pas affectée par la disponibilité de la file
- **Batching** — Événements groupés par taille (100) ou temps (1 sec)
- **Analyse User-Agent** — Navigateur, version, OS, type d'appareil via woothee
- **Enrichissement GeoIP** — Pays et ville depuis l'IP (MaxMind mmdb avec rechargement à chaud)
- **Déduplication des références** — Déduplication basée sur MD5 pour les referers et user agents
- **Partitionnement mensuel** — Création automatique des partitions pour `redirect_events`

### Démarrage rapide

```bash
# Activer dans config.yaml
events:
  enabled: true
  rabbitmq:
    url: amqp://guest:guest@localhost:5672/%2f

# Ou via variables d'environnement
REDIRECTOR__EVENTS__ENABLED=true
RABBITMQ_URL=amqp://guest:guest@localhost:5672/%2f

# Lancer le consommateur
RABBITMQ_URL=amqp://... DATABASE_URL=postgres://... cargo run --bin event_consumer
```

### Métriques d'événements

Accédez à `/metrics` pour surveiller :

- `redirector_events_published_total` - Nombre total d'événements publiés
- `redirector_events_published_errors_total` - Nombre total d'erreurs de publication
- `redirector_events_batched_total` - Nombre total de batches traités
- `redirector_events_batch_size` - Taille des batches (histogramme)
- `redirector_events_batch_latency_ms` - Latence de batching en millisecondes

### Configuration

Ajoutez à `config.yaml` :

```yaml
events:
  enabled: true
  rabbitmq:
    url: ${RABBITMQ_URL}
    queue: "redirect_events"
    exchange: "redirector"
  batch:
    max_size: 100
    timeout_ms: 1000
  geoip:
    database_path: "/path/to/GeoLite2-City.mmdb"
    reload_interval_hours: 24
```

Variables d'environnement :

```
REDIRECTOR__EVENTS__ENABLED          → Activer l'analyse d'événements
REDIRECTOR__EVENTS__RABBITMQ__URL    → URL de connexion RabbitMQ
RABBITMQ_URL                         → Alternative pour compatibilité PaaS
```

## Licence

Licence MIT - voir [LICENSE](../LICENSE) pour les détails.

## Contribuer

Les contributions sont les bienvenues ! Veuillez :

1. Forker le dépôt
2. Créer une branche de fonctionnalité
3. Soumettre une Pull Request

La branche master protégée nécessite une revue de PR.
