# redirector

> **Acortador de URL y servicio de redirección de alto rendimiento** construido con Rust, Axum, Redis y PostgreSQL. Incluye páginas intersticiales seguras, panel de administración en tiempo real y observabilidad de nivel empresarial.

[English](../README.md) | [Русский](README.ru.md) | [中文](README.zh.md) | [हिंदी](README.hi.md) | **Español** | [Português](README.pt.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Polski](README.pl.md) | [Nederlands](README.nl.md) | [Italiano](README.it.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) | [עברית](README.he.md) | [Bahasa Indonesia](README.id.md) | [Tiếng Việt](README.vi.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![Docker Image Size](https://ghcr-badge.egpl.dev/brilliant-almazov/redirector/size)](https://github.com/brilliant-almazov/redirector/pkgs/container/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

**Palabras clave**: acortador de URL, acortador de enlaces, servicio de redirección, servicio web Rust, framework Axum, caché Redis, PostgreSQL, métricas Prometheus, hashids, enlaces cortos, páginas intersticiales, redirecciones seguras, alto rendimiento, microservicio

Servicio de redirección de URL seguro con páginas intersticiales y enlaces cortos basados en hashid. Perfecto para herramientas internas, gestión de enlaces empresariales y servicios de URL cortas de marca.

### Rendimiento

| Escenario | RPS | Latencia Avg | Latencia P99 |
|-----------|-----|--------------|--------------|
| 100% Cache Hit | **7,800+** | ~14ms | ~50ms |
| Cache Miss (10K URLs) | **2,300+** | ~44ms | ~81ms |

**Condiciones de prueba**: wrk -t4 -c100 -d30s, PostgreSQL 15, Dragonfly (Redis), macOS M1 (Docker)

> ⚠️ Los resultados son de Docker en macOS con sobrecarga de VM. Se espera que el despliegue en Linux nativo sea **3-5x más rápido**.

## Problema

Compartir URLs largas es inconveniente. Los acortadores de URL existen, pero a menudo redirigen inmediatamente, lo que puede ser un riesgo de seguridad. Los usuarios deberían ver a dónde van antes de ser redirigidos.

**redirector** proporciona redirecciones seguras con:
- Página intersticial que muestra la URL de destino antes de redirigir
- Temporizador de cuenta regresiva para la conciencia del usuario
- Páginas hermosas y personalizadas

## Características

- 🔗 **URLs Hashid** - IDs cortos, únicos y no secuenciales (ej. `/r/abc123`)
- ⏱️ **Página intersticial** - Temporizador de cuenta regresiva muestra la URL de destino antes de redirigir
- ⚡ **Caché Redis** - Búsquedas rápidas con TTL configurable
- 🛡️ **Circuit breaker** - Protección de base de datos contra fallos en cascada
- 🚦 **Limitación de velocidad** - Límites de velocidad globales y a nivel de base de datos
- 📊 **Métricas Prometheus** - Observabilidad completa con protección Basic Auth
- 🎨 **Páginas hermosas** - Páginas 404 e índice limpias con 4 temas
- 🔑 **Múltiples sales** - Soporte de rotación de sal hashid para migración
- 📱 **Panel de administración** - Monitoreo de métricas en tiempo real con SSE

## Capturas de pantalla

| Claro | Oscuro | Gris | Cálido |
|-------|--------|------|--------|
| ![Dashboard Claro](screenshots/dashboard-light.png) | ![Dashboard Oscuro](screenshots/dashboard-dark.png) | ![Dashboard Gris](screenshots/dashboard-gray.png) | ![Dashboard Cálido](screenshots/dashboard-warm.png) |
| ![Login Claro](screenshots/login-light.png) | ![Login Oscuro](screenshots/login-dark.png) | ![Login Gris](screenshots/login-gray.png) | ![Login Cálido](screenshots/login-warm.png) |
| ![Index Light](screenshots/index-light.png) | ![Index Dark](screenshots/index-dark.png) | ![Index Gray](screenshots/index-gray.png) | ![Index Warm](screenshots/index-warm.png) |
| ![Interstitial Light](screenshots/interstitial-light.png) | ![Interstitial Dark](screenshots/interstitial-dark.png) | ![Interstitial Gray](screenshots/interstitial-gray.png) | ![Interstitial Warm](screenshots/interstitial-warm.png) |
| ![404 Light](screenshots/404-light.png) | ![404 Dark](screenshots/404-dark.png) | ![404 Gray](screenshots/404-gray.png) | ![404 Warm](screenshots/404-warm.png) |

### Modal de prueba de carga

| Claro | Oscuro | Gris | Cálido |
|-------|--------|------|--------|
| ![Modal Claro](screenshots/dashboard-modal-light.png) | ![Modal Oscuro](screenshots/dashboard-modal-dark.png) | ![Modal Gris](screenshots/dashboard-modal-gray.png) | ![Modal Cálido](screenshots/dashboard-modal-warm.png) |

## Stack tecnológico

- **Lenguaje**: Rust (async con Tokio)
- **Framework web**: Axum
- **Caché**: Compatible con Redis (Redis, Dragonfly, Valkey, KeyDB, etc.)
- **Base de datos**: PostgreSQL (capa de almacenamiento intercambiable)
- **Métricas**: Prometheus + metrics-rs
- **Hash de contraseñas**: Argon2

> **Nota**: Las capas de almacenamiento y caché son abstractas y pueden ser reemplazadas por cualquier fuente de datos compatible. Actualmente en desarrollo activo.

## Inicio rápido

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

#### Configuración Base64

Para entornos donde no es posible montar archivos de configuración (ej. serverless, PaaS):

```bash
# Encode
cat config.yaml | base64

# Run with base64 config
CONFIG_BASE64="c2VydmVyOgogIGhvc3Q6IC..." docker run ghcr.io/brilliant-almazov/redirector:latest
```

## Cómo funciona

1. El usuario visita `/r/{hashid}` (ej. `/r/abc123`)
2. El servicio decodifica el hashid a ID numérico
3. Verifica la caché Redis para la URL
4. En caso de fallo de caché, consulta PostgreSQL
5. Almacena el resultado en caché Redis
6. Muestra página intersticial con cuenta regresiva
7. Después de la cuenta regresiva, redirige a la URL de destino

## Endpoints

| Endpoint | Auth | Descripción |
|----------|------|-------------|
| `GET /` | No | Página principal |
| `GET /r/{hashid}` | No | Redirección con página intersticial |
| `GET /d/{hashid}` | No | Redirección demo (prueba de carga sintética) |
| `GET /health` | No | Verificación de salud |
| `GET /metrics` | Basic | Métricas Prometheus |
| `GET /admin` | Session | Inicio de sesión del panel de administración |
| `GET /admin/dashboard` | Session | Panel de administración |

## Panel de administración

El servicio incluye un panel de administración opcional para monitorear métricas en tiempo real.

### Configuración

1. **Generar hash de contraseña:**

```bash
cargo run --bin hash_password
# Ingrese contraseña, o:
cargo run --bin hash_password -- "your-password"
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

## Contribuir

¡Las contribuciones son bienvenidas! Por favor:

1. Haga fork del repositorio
2. Cree una rama de características
3. Envíe un Pull Request

La rama master protegida requiere revisión de PR.
