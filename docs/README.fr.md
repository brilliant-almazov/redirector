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
- 🎨 **Belles pages** - Pages 404 et index propres avec 3 thèmes
- 🔑 **Sels multiples** - Support de rotation de sel hashid pour migration
- 📱 **Tableau de bord admin** - Surveillance des métriques en temps réel avec SSE

## Captures d'écran

| Clair | Sombre | Chaud |
|-------|--------|-------|
| ![Dashboard Clair](screenshots/dashboard-light.png) | ![Dashboard Sombre](screenshots/dashboard-dark.png) | ![Dashboard Chaud](screenshots/dashboard-warm.png) |
| ![Login Clair](screenshots/login-light.png) | ![Login Sombre](screenshots/login-dark.png) | ![Login Chaud](screenshots/login-warm.png) |
| ![404 Clair](screenshots/not-found-light.png) | ![404 Sombre](screenshots/not-found-dark.png) | ![404 Chaud](screenshots/not-found-warm.png) |

| Page d'accueil | Interstitielle |
|----------------|----------------|
| ![Page d'accueil](screenshots/index.png) | ![Interstitielle](screenshots/interstitial.png) |

## Stack technologique

- **Langage**: Rust (async avec Tokio)
- **Framework web**: Axum
- **Cache**: Compatible Redis (Redis, Dragonfly, Valkey, KeyDB, etc.)
- **Base de données**: PostgreSQL (couche de stockage interchangeable)
- **Métriques**: Prometheus + metrics-rs
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

## Licence

Licence MIT - voir [LICENSE](../LICENSE) pour les détails.

## Contribuer

Les contributions sont les bienvenues ! Veuillez :

1. Forker le dépôt
2. Créer une branche de fonctionnalité
3. Soumettre une Pull Request

La branche master protégée nécessite une revue de PR.
