# redirector

[English](../README.md) | [Русский](README.ru.md) | [中文](README.zh.md) | [हिंदी](README.hi.md) | **Español** | [Português](README.pt.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Polski](README.pl.md) | [Nederlands](README.nl.md) | [Italiano](README.it.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) | [Bahasa Indonesia](README.id.md) | [Tiếng Việt](README.vi.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

Servicio de redirección de URL seguro con páginas intersticiales y enlaces cortos basados en hashid.

## Problema

Compartir URLs largas es inconveniente. Los acortadores de URL existen, pero a menudo redirigen inmediatamente, lo que puede ser un riesgo de seguridad. Los usuarios deberían ver a dónde van antes de ser redirigidos.

**redirector** proporciona redirecciones seguras con:
- Página intersticial que muestra la URL de destino antes de redirigir
- Temporizador de cuenta regresiva para la conciencia del usuario
- Páginas hermosas y con marca

## Características

- 🔗 **URLs Hashid** - IDs cortos, únicos y no secuenciales (ej. `/r/abc123`)
- ⏱️ **Página intersticial** - Temporizador muestra URL de destino antes de redirigir
- ⚡ **Caché Redis** - Búsquedas rápidas con TTL configurable
- 🛡️ **Circuit breaker** - Protección de base de datos contra fallos en cascada
- 🚦 **Limitación de tasa** - Límites globales y a nivel de base de datos
- 📊 **Métricas Prometheus** - Observabilidad completa con protección Basic Auth
- 🎨 **Páginas hermosas** - Páginas 404 e índice limpias
- 🔑 **Múltiples sales** - Soporte de rotación de sal hashid para migración

## Inicio Rápido

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

## Cómo Funciona

1. El usuario visita `/r/{hashid}` (ej. `/r/abc123`)
2. El servicio decodifica hashid a ID numérico
3. Verifica caché Redis para URL
4. En caso de fallo de caché, consulta PostgreSQL
5. Almacena resultado en Redis
6. Muestra página intersticial con cuenta regresiva
7. Después de la cuenta regresiva, redirige a URL de destino

## Endpoints

| Endpoint | Auth | Descripción |
|----------|------|-------------|
| `GET /` | No | Página de inicio |
| `GET /r/{hashid}` | No | Redirección con intersticial |
| `GET /health` | No | Verificación de salud |
| `GET /metrics` | Basic | Métricas Prometheus |
| `GET /admin` | Session | Inicio de sesión del panel de administración |
| `GET /admin/dashboard` | Session | Panel de administración |

## Panel de Administración

El servicio incluye un panel de administración opcional para monitorear métricas en tiempo real.

### Configuración

1. **Generar hash de contraseña:**

```bash
# Usando Rust
cargo run --bin hash_password

# O usando Python (pip install argon2-cffi)
./scripts/hash_password.sh
```

2. **Agregar a config.yaml:**

```yaml
admin:
  enabled: true
  session_ttl_hours: 24
  users:
    - username: admin
      password_hash: "$argon2id$v=19$m=19456,t=2,p=1$..."  # del paso 1
```

3. **Acceder al panel:**

Abra `http://localhost:8080/admin` e inicie sesión con sus credenciales.

### Características

- Gráficos de RPS y latencia en tiempo real
- Métricas del sistema (CPU, memoria, tiempo de actividad)
- Monitoreo de tasa de aciertos de caché
- Lista de redirecciones recientes
- Simulación de carga para pruebas
- Tres temas: Claro, Oscuro, Cálido

## Licencia

Licencia MIT - ver [LICENSE](../LICENSE) para detalles.
