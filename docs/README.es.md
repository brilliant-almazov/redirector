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
- 📤 **Análisis de eventos** - Publicación opcional de eventos en RabbitMQ con consumidor PostgreSQL

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
- **Cola de mensajes**: RabbitMQ (opcional, para análisis de eventos)
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

## Configuración

Cree `config.yaml`:

```yaml
server:
  host: "0.0.0.0"
  port: 8080

hashids:
  salts:
    - ${HASHID_SALT}          # Salt principal
    - ${HASHID_SALT_OLD}      # Opcional: salt antiguo para migración
  min_length: 6

redis:
  url: ${REDIS_URL}
  cache_ttl_seconds: 86400    # 24 horas

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
    table: "dictionary.urls"    # Nombre de su tabla
    id_column: "id"             # Columna ID
    url_column: "name"          # Columna URL

interstitial:
  delay_seconds: 5            # Cuenta regresiva antes de redirigir

metrics:
  basic_auth:
    username: prometheus
    password: ${METRICS_PASSWORD}

rate_limit:
  requests_per_second: 1000
  burst: 100
```

### Opciones de configuración

#### Servidor

| Opción | Predeterminado | Descripción |
|--------|----------------|-------------|
| `host` | `0.0.0.0` | Dirección de enlace |
| `port` | `8080` | Puerto HTTP |

#### Hashids

| Opción | Predeterminado | Descripción |
|--------|----------------|-------------|
| `salts` | *requerido* | Lista de sales hashid (primera = principal) |
| `min_length` | `6` | Longitud mínima de hashid |

#### Redis

| Opción | Predeterminado | Descripción |
|--------|----------------|-------------|
| `url` | *requerido* | URL de conexión Redis |
| `cache_ttl_seconds` | `86400` | TTL de caché en segundos |

#### Base de datos

| Opción | Predeterminado | Descripción |
|--------|----------------|-------------|
| `url` | *requerido* | URL de conexión PostgreSQL |
| `pool.max_connections` | `3` | Tamaño del pool de conexiones |
| `pool.connect_timeout_seconds` | `3` | Timeout de conexión |
| `rate_limit.max_requests_per_second` | `50` | Límite de velocidad de BD |
| `circuit_breaker.failure_threshold` | `3` | Fallos antes de apertura |
| `circuit_breaker.reset_timeout_seconds` | `60` | Timeout de reinicio del circuit |

#### Límite de velocidad (Global)

| Opción | Predeterminado | Descripción |
|--------|----------------|-------------|
| `requests_per_second` | `1000` | Límite de velocidad global |
| `burst` | `100` | Capacidad de ráfaga |

### Variables de entorno

Hay **tres formas** de configurar el servicio, listadas por prioridad (mayor primero):

| Prioridad | Método | Caso de uso |
|-----------|--------|-------------|
| 1 | Variables `REDIRECTOR__*` | Sobrescribir valores individuales |
| 2 | Variables PaaS estándar (`DATABASE_URL`, etc.) | Plataformas PaaS (Railway, Heroku, Render) |
| 3 | Archivo de configuración (`config.yaml` o `CONFIG_BASE64`) | Configuración base |

#### Variables especiales

| Variable | Predeterminado | Descripción |
|----------|----------------|-------------|
| `CONFIG_PATH` | `config.yaml` | Ruta al archivo de configuración YAML |
| `CONFIG_BASE64` | — | Configuración YAML en Base64 (tiene prioridad sobre `CONFIG_PATH`) |

#### Variables de entorno PaaS estándar

Se reconocen y aplican automáticamente. La mayoría de las plataformas PaaS las configuran por usted:

| Variable | Ruta en configuración | Ejemplo |
|----------|----------------------|---------|
| `DATABASE_URL` | `database.url` | `postgres://user:pass@host:5432/db` |
| `REDIS_URL` | `redis.url` | `redis://host:6379` |
| `PORT` | `server.port` | `3000` |

> **Regla de prioridad**: Si tanto `DATABASE_URL` como `REDIRECTOR__DATABASE__URL` están configurados, la versión con prefijo `REDIRECTOR__` gana.

#### Variables con prefijo (`REDIRECTOR__*`)

Cualquier valor de configuración puede sobrescribirse usando el prefijo `REDIRECTOR__` con `__` (doble guion bajo) como separador de anidamiento:

```
Ruta YAML de configuración  →  Variable de entorno
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

#### Ejemplos por plataforma de despliegue

**Railway / Render / Fly.io** (PaaS con bases de datos gestionadas):

```bash
# Estas se configuran normalmente automáticamente por la plataforma:
DATABASE_URL=postgres://user:pass@host:5432/db
REDIS_URL=redis://host:6379
PORT=3000

# Configure su config vía base64:
CONFIG_BASE64=c2VydmVyOgogIGhvc3Q6IC...

# O sobrescriba valores individuales:
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
      # O sobrescriba valores individuales sobre el archivo de configuración:
      REDIRECTOR__RATE_LIMIT__REQUESTS_PER_SECOND: "2000"
      REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD: "${METRICS_PASSWORD}"
    volumes:
      - ./config.yaml:/app/config.yaml  # opcional si usa CONFIG_BASE64
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

**Docker simple**:

```bash
docker run -p 8080:8080 \
  -e DATABASE_URL="postgres://user:pass@host:5432/db" \
  -e REDIS_URL="redis://host:6379" \
  -e CONFIG_BASE64="$(cat config.yaml | base64)" \
  ghcr.io/brilliant-almazov/redirector:latest
```

**Configuración mínima (solo variables de entorno, sin archivo de configuración)**:

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

#### Configuración Base64

Para entornos donde montar archivos de configuración no es práctico (PaaS, serverless, CI/CD), pase toda la configuración como una cadena codificada en Base64:

```bash
# Codificar
cat config.yaml | base64

# Decodificar (para verificar)
echo "$CONFIG_BASE64" | base64 -d
```

`CONFIG_BASE64` tiene prioridad sobre `CONFIG_PATH`. Las sobrescrituras de variables de entorno (`REDIRECTOR__*` y variables PaaS) se aplican **sobre** la configuración decodificada.

## Cómo funciona

1. El usuario visita `/r/{hashid}` (ej. `/r/abc123`)
2. El servicio decodifica el hashid a ID numérico
3. Verifica la caché Redis para la URL
4. En caso de fallo de caché, consulta PostgreSQL
5. Almacena el resultado en caché Redis
6. Muestra página intersticial con cuenta regresiva
7. Después de la cuenta regresiva, redirige a la URL de destino

```
┌──────┐     ┌───────────┐     ┌───────┐     ┌──────────┐
│Client│────▶│Redirector │────▶│ Redis │────▶│PostgreSQL│
└──────┘     └───────────┘     └───────┘     └──────────┘
                 │  │
                 │  └──────────────────┐ (opcional)
                 ▼                     ▼
          ┌─────────────┐     ┌──────────────┐     ┌──────────────┐
          │Página       │     │  RabbitMQ    │────▶│Consumidor    │
          │intersticial │     └──────────────┘     │de eventos    │
          └─────────────┘                          └──────┬───────┘
                                                    │
                                            ┌──────▼───────────┐
                                            │PostgreSQL        │
                                            │Analítica         │
                                            └──────────────────┘
```

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

## Análisis de eventos

Pipeline opcional de publicación de eventos para análisis de redirecciones. Cuando está habilitado, cada evento de redirección se publica en RabbitMQ y es procesado por un binario separado que escribe en PostgreSQL con enriquecimiento detallado.

> **Documentación completa**: [docs/EVENT_ANALYTICS.md](../docs/EVENT_ANALYTICS.md)

### Características

- **Publicación fire-and-forget** — La latencia de redirección no se ve afectada por la disponibilidad de la cola
- **Batching** — Eventos agrupados por tamaño (100) o tiempo (1 segundo)
- **Análisis de User-Agent** — Navegador, versión, SO, tipo de dispositivo vía woothee
- **Enriquecimiento GeoIP** — País y ciudad desde IP (MaxMind mmdb con recarga en caliente)
- **Deduplicación de referencias** — Deduplicación basada en MD5 para referers y user agents
- **Particionamiento mensual** — Creación automática de particiones para `redirect_events`
- **Normalización de dominios** — `WWW.Example.COM` → `example.com`

### Arquitectura

```
Manejador de redirección
    │
    ├── try_send(RedirectEvent) ──► [canal tokio::mpsc]
    │   (no bloqueante,                 │
    │    fire-and-forget)               ▼
    │                             Tarea en segundo plano
    │                             (agrupar por tamaño/tiempo)
    │                                     │
    │                                     ▼
    │                              [Cola RabbitMQ]
    │                                     │
    │                                     ▼
    │                              Consumidor de eventos
    │                              (binario/contenedor separado)
    │                                     │
    │                                     ▼
    │                            [PostgreSQL Analítica]
    │                            (particionado mensualmente)
```

### Inicio rápido

```bash
# Habilitar en config.yaml
events:
  enabled: true
  rabbitmq:
    url: amqp://guest:guest@localhost:5672/%2f

# O mediante variables de entorno
REDIRECTOR__EVENTS__ENABLED=true
RABBITMQ_URL=amqp://guest:guest@localhost:5672/%2f

# Ejecutar consumidor
RABBITMQ_URL=amqp://... DATABASE_URL=postgres://... cargo run --bin event_consumer
```

### Docker Compose con eventos

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
      - GEOIP_DB_PATH=/data/GeoLite2-City.mmdb  # opcional
    depends_on: [rabbitmq, analytics-db]

  rabbitmq:
    image: rabbitmq:4-management-alpine
    ports: ["5672:5672", "15672:15672"]

  analytics-db:
    image: postgres:16-alpine
    environment:
      POSTGRES_DB: analytics
```

### Decisiones de diseño clave

- **Nunca bloquea redirecciones**: `try_send()` en canal acotado, descarta eventos si está lleno
- **Lotes de eventos con tipos seguros**: `EventBatch` es un enum Rust marcado por `event_type`
- **IDs de lote Snowflake**: Época personalizada 2025-01-01, ~69 años de IDs únicos
- **Degradación elegante**: Si RabbitMQ está caído, las redirecciones continúan; los eventos se descartan con métricas

## Métricas

El servicio expone métricas completas de Prometheus en `/metrics` (requiere autenticación básica):

### Métricas de servicio
```
redirector_up 1
redirector_build_info{version="0.1.0"} 1
redirector_uptime_seconds 3600.5
```

### Métricas de solicitud
```
redirect_requests_total 150000
not_found_requests_total 50
request_duration_seconds{quantile="0.5"} 0.040
request_duration_seconds{quantile="0.99"} 0.081
```

### Métricas de caché
```
cache_hits_total 140000
cache_misses_total 10000
cache_get_duration_seconds{quantile="0.5"} 0.002
cache_set_duration_seconds{quantile="0.5"} 0.002
```

### Métricas de base de datos
```
db_queries_total 10000
db_hits_total 9950
db_misses_total 50
db_errors_total 0
db_query_duration_seconds{quantile="0.5"} 0.035
db_rate_limit_exceeded_total 0
circuit_breaker_rejections_total 0
```

### Limitación de velocidad
```
rate_limit_exceeded_total 0
```

### Eventos (cuando está habilitado)
```
events_published 50000
events_dropped 0
events_publish_errors 0
events_serialize_errors 0
rabbitmq_connected 1
```

## Licencia

Licencia MIT - ver [LICENSE](../LICENSE) para detalles.

## Contribuir

¡Las contribuciones son bienvenidas! Por favor:

1. Haga fork del repositorio
2. Cree una rama de características
3. Envíe un Pull Request

La rama master protegida requiere revisión de PR.
