# redirector

[English](../README.md) | [Русский](README.ru.md) | [中文](README.zh.md) | [हिंदी](README.hi.md) | [Español](README.es.md) | [Português](README.pt.md) | **Français** | [Deutsch](README.de.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Polski](README.pl.md) | [Nederlands](README.nl.md) | [Italiano](README.it.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) | [Bahasa Indonesia](README.id.md) | [Tiếng Việt](README.vi.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

Service de redirection d'URL sécurisé avec pages interstitielles et liens courts basés sur hashid.

## Problème

Partager de longues URLs est peu pratique. Les raccourcisseurs d'URL existent mais redirigent souvent immédiatement, ce qui peut présenter un risque de sécurité. Les utilisateurs devraient voir où ils vont avant d'être redirigés.

**redirector** fournit des redirections sécurisées avec :
- Page interstitielle montrant l'URL cible avant la redirection
- Minuterie de compte à rebours pour la sensibilisation de l'utilisateur
- Pages belles et personnalisées

## Fonctionnalités

- 🔗 **URLs Hashid** - IDs courts, uniques et non-séquentiels (ex. `/r/abc123`)
- ⏱️ **Page interstitielle** - Minuterie affiche l'URL cible avant la redirection
- ⚡ **Cache Redis** - Recherches rapides avec TTL configurable
- 🛡️ **Circuit breaker** - Protection de base de données contre les défaillances en cascade
- 🚦 **Limitation de débit** - Limites globales et au niveau de la base de données
- 📊 **Métriques Prometheus** - Observabilité complète avec protection Basic Auth
- 🎨 **Belles pages** - Pages 404 et index propres
- 🔑 **Sels multiples** - Support de rotation de sel hashid pour la migration

## Démarrage Rapide

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

## Comment ça Fonctionne

1. L'utilisateur visite `/r/{hashid}` (ex. `/r/abc123`)
2. Le service décode le hashid en ID numérique
3. Vérifie le cache Redis pour l'URL
4. En cas d'absence dans le cache, interroge PostgreSQL
5. Met en cache le résultat dans Redis
6. Affiche la page interstitielle avec compte à rebours
7. Après le compte à rebours, redirige vers l'URL cible

## Endpoints

| Endpoint | Auth | Description |
|----------|------|-------------|
| `GET /` | Non | Page d'accueil |
| `GET /r/{hashid}` | Non | Redirection avec page interstitielle |
| `GET /health` | Non | Vérification de santé |
| `GET /metrics` | Basic | Métriques Prometheus |
| `GET /admin` | Session | Connexion au panneau d'administration |
| `GET /admin/dashboard` | Session | Panneau d'administration |

## Panneau d'Administration

Le service comprend un panneau d'administration optionnel pour surveiller les métriques en temps réel.

### Configuration

1. **Générer le hash du mot de passe :**

```bash
# Avec Rust
cargo run --bin hash_password

# Ou avec Python (pip install argon2-cffi)
./scripts/hash_password.sh
```

2. **Ajouter à config.yaml :**

```yaml
admin:
  enabled: true
  session_ttl_hours: 24
  users:
    - username: admin
      password_hash: "$argon2id$v=19$m=19456,t=2,p=1$..."  # de l'étape 1
```

3. **Accéder au panneau :**

Ouvrez `http://localhost:8080/admin` et connectez-vous avec vos identifiants.

### Fonctionnalités

- Graphiques RPS et latence en temps réel
- Métriques système (CPU, mémoire, uptime)
- Surveillance du taux de succès du cache
- Liste des redirections récentes
- Simulation de charge pour les tests
- Trois thèmes : Clair, Sombre, Chaud

## Licence

Licence MIT - voir [LICENSE](../LICENSE) pour les détails.
