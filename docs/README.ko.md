# redirector

> **고성능 URL 단축 및 리다이렉트 서비스** Rust, Axum, Redis, PostgreSQL로 구축. 안전한 인터스티셜 페이지, 실시간 관리 대시보드, 엔터프라이즈급 관측성을 제공합니다.

[English](../README.md) | [Русский](README.ru.md) | [中文](README.zh.md) | [हिंदी](README.hi.md) | [Español](README.es.md) | [Português](README.pt.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | **한국어** | [Polski](README.pl.md) | [Nederlands](README.nl.md) | [Italiano](README.it.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) | [עברית](README.he.md) | [Bahasa Indonesia](README.id.md) | [Tiếng Việt](README.vi.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![Docker Image Size](https://ghcr-badge.egpl.dev/brilliant-almazov/redirector/size)](https://github.com/brilliant-almazov/redirector/pkgs/container/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

**키워드**: URL 단축기, 링크 단축기, 리다이렉트 서비스, Rust 웹 서비스, Axum 프레임워크, Redis 캐시, PostgreSQL, Prometheus 메트릭, hashids, 짧은 링크, 인터스티셜 페이지, 안전한 리다이렉트, 고성능, 마이크로서비스

인터스티셜 페이지와 hashid 기반 짧은 링크를 갖춘 안전한 URL 리다이렉트 서비스. 내부 도구, 기업 링크 관리, 브랜드 단축 URL 서비스에 적합합니다.

### 성능

| 시나리오 | RPS | 평균 지연 | P99 지연 |
|----------|-----|-----------|----------|
| 100% 캐시 히트 | **7,800+** | ~14ms | ~50ms |
| 캐시 미스 (10K URLs) | **2,300+** | ~44ms | ~81ms |

**테스트 조건**: wrk -t4 -c100 -d30s, PostgreSQL 15, Dragonfly (Redis), macOS M1 (Docker)

> ⚠️ 결과는 VM 오버헤드가 있는 macOS Docker에서 측정되었습니다. 네이티브 Linux 배포는 **3-5배 빠를** 것으로 예상됩니다.

## 문제

긴 URL을 공유하는 것은 불편합니다. URL 단축기가 존재하지만, 대부분 즉시 리다이렉트되어 보안 위험이 될 수 있습니다. 사용자는 리다이렉트되기 전에 어디로 가는지 볼 수 있어야 합니다.

**redirector**는 안전한 리다이렉트를 제공합니다:
- 리다이렉트 전 대상 URL을 보여주는 인터스티셜 페이지
- 사용자 인식을 위한 카운트다운 타이머
- 아름답고 브랜드화된 페이지

## 기능

- 🔗 **Hashid URL** - 짧고, 고유하고, 비순차적인 ID (예: `/r/abc123`)
- ⏱️ **인터스티셜 페이지** - 리다이렉트 전 대상 URL을 보여주는 카운트다운 타이머
- ⚡ **Redis 캐싱** - 설정 가능한 TTL로 빠른 조회
- 🛡️ **서킷 브레이커** - 연쇄 장애에 대한 데이터베이스 보호
- 🚦 **속도 제한** - 전역 및 데이터베이스 수준 속도 제한
- 📊 **Prometheus 메트릭** - Basic Auth 보호가 있는 완전한 관측성
- 🎨 **아름다운 페이지** - 4가지 테마가 있는 깔끔한 404 및 인덱스 페이지
- 🔑 **다중 솔트** - 마이그레이션을 위한 hashid 솔트 로테이션 지원
- 📱 **관리 대시보드** - SSE를 통한 실시간 메트릭 모니터링
- 📤 **이벤트 분석** - RabbitMQ 이벤트 퍼블리싱과 PostgreSQL 컨슈머 (선택사항)

## 스크린샷

| 라이트 | 다크 | 그레이 | 웜 |
|--------|------|--------|-----|
| ![대시보드 라이트](screenshots/dashboard-light.png) | ![대시보드 다크](screenshots/dashboard-dark.png) | ![대시보드 그레이](screenshots/dashboard-gray.png) | ![대시보드 웜](screenshots/dashboard-warm.png) |
| ![로그인 라이트](screenshots/login-light.png) | ![로그인 다크](screenshots/login-dark.png) | ![로그인 그레이](screenshots/login-gray.png) | ![로그인 웜](screenshots/login-warm.png) |
| ![Index Light](screenshots/index-light.png) | ![Index Dark](screenshots/index-dark.png) | ![Index Gray](screenshots/index-gray.png) | ![Index Warm](screenshots/index-warm.png) |
| ![Interstitial Light](screenshots/interstitial-light.png) | ![Interstitial Dark](screenshots/interstitial-dark.png) | ![Interstitial Gray](screenshots/interstitial-gray.png) | ![Interstitial Warm](screenshots/interstitial-warm.png) |
| ![404 Light](screenshots/404-light.png) | ![404 Dark](screenshots/404-dark.png) | ![404 Gray](screenshots/404-gray.png) | ![404 Warm](screenshots/404-warm.png) |

### 부하 테스트 모달

| 라이트 | 다크 | 그레이 | 웜 |
|--------|------|--------|-----|
| ![모달 라이트](screenshots/dashboard-modal-light.png) | ![모달 다크](screenshots/dashboard-modal-dark.png) | ![모달 그레이](screenshots/dashboard-modal-gray.png) | ![모달 웜](screenshots/dashboard-modal-warm.png) |

## 기술 스택

- **언어**: Rust (Tokio를 사용한 비동기)
- **웹 프레임워크**: Axum
- **캐시**: Redis 호환 (Redis, Dragonfly, Valkey, KeyDB 등)
- **데이터베이스**: PostgreSQL (플러그 가능한 스토리지 레이어)
- **메트릭**: Prometheus + metrics-rs
- **메시지 큐**: RabbitMQ (선택사항, 이벤트 분석용)
- **비밀번호 해싱**: Argon2

> **참고**: 스토리지 및 캐시 레이어는 추상화되어 있으며 호환되는 모든 데이터 소스로 교체할 수 있습니다. 현재 활발히 개발 중입니다.

## 빠른 시작

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

## 설정

`config.yaml`을 생성하세요:

```yaml
server:
  host: "0.0.0.0"
  port: 8080

hashids:
  salts:
    - ${HASHID_SALT}          # 기본 솔트
    - ${HASHID_SALT_OLD}      # 선택: 마이그레이션용 이전 솔트
  min_length: 6

redis:
  url: ${REDIS_URL}
  cache_ttl_seconds: 86400    # 24시간

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
    table: "dictionary.urls"    # 테이블 이름
    id_column: "id"             # ID 컬럼
    url_column: "name"          # URL 컬럼

interstitial:
  delay_seconds: 5            # 리다이렉트 전 카운트다운

metrics:
  basic_auth:
    username: prometheus
    password: ${METRICS_PASSWORD}

rate_limit:
  requests_per_second: 1000
  burst: 100
```

### 설정 옵션

#### 서버

| 옵션 | 기본값 | 설명 |
|------|--------|------|
| `host` | `0.0.0.0` | 바인드 주소 |
| `port` | `8080` | HTTP 포트 |

#### Hashids

| 옵션 | 기본값 | 설명 |
|------|--------|------|
| `salts` | *필수* | hashid 솔트 목록 (첫 번째 = 기본) |
| `min_length` | `6` | 최소 hashid 길이 |

#### Redis

| 옵션 | 기본값 | 설명 |
|------|--------|------|
| `url` | *필수* | Redis 연결 URL |
| `cache_ttl_seconds` | `86400` | 캐시 TTL (초) |

#### 데이터베이스

| 옵션 | 기본값 | 설명 |
|------|--------|------|
| `url` | *필수* | PostgreSQL 연결 URL |
| `pool.max_connections` | `3` | 커넥션 풀 크기 |
| `pool.connect_timeout_seconds` | `3` | 연결 타임아웃 |
| `rate_limit.max_requests_per_second` | `50` | DB 속도 제한 |
| `circuit_breaker.failure_threshold` | `3` | 오픈까지 실패 횟수 |
| `circuit_breaker.reset_timeout_seconds` | `60` | 서킷 리셋 타임아웃 |

#### 속도 제한 (글로벌)

| 옵션 | 기본값 | 설명 |
|------|--------|------|
| `requests_per_second` | `1000` | 글로벌 속도 제한 |
| `burst` | `100` | 버스트 허용량 |

### 환경 변수

서비스를 설정하는 **세 가지 방법**이 있으며, 우선순위 순으로 나열됩니다 (높은 순):

| 우선순위 | 방법 | 사용 사례 |
|----------|------|-----------|
| 1 | `REDIRECTOR__*` 환경 변수 | 개별 값 오버라이드 |
| 2 | 표준 PaaS 환경 변수 (`DATABASE_URL` 등) | PaaS 플랫폼 (Railway, Heroku, Render) |
| 3 | 설정 파일 (`config.yaml` 또는 `CONFIG_BASE64`) | 기본 설정 |

#### 특수 변수

| 변수 | 기본값 | 설명 |
|------|--------|------|
| `CONFIG_PATH` | `config.yaml` | YAML 설정 파일 경로 |
| `CONFIG_BASE64` | — | Base64로 인코딩된 YAML 설정 (`CONFIG_PATH`보다 우선) |

#### 표준 PaaS 환경 변수

이들은 자동으로 인식되고 적용됩니다. 대부분의 PaaS 플랫폼이 자동으로 설정합니다:

| 변수 | 설정 경로 | 예시 |
|------|-----------|------|
| `DATABASE_URL` | `database.url` | `postgres://user:pass@host:5432/db` |
| `REDIS_URL` | `redis.url` | `redis://host:6379` |
| `PORT` | `server.port` | `3000` |

> **우선순위 규칙**: `DATABASE_URL`과 `REDIRECTOR__DATABASE__URL`이 모두 설정된 경우, `REDIRECTOR__` 프리픽스 버전이 우선합니다.

#### 프리픽스 환경 변수 (`REDIRECTOR__*`)

모든 설정 값은 `REDIRECTOR__` 프리픽스와 `__` (더블 언더스코어)를 중첩 구분자로 사용하여 오버라이드할 수 있습니다:

```
YAML 설정 경로               →  환경 변수
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

#### 배포 플랫폼별 예시

**Railway / Render / Fly.io** (관리형 데이터베이스를 가진 PaaS):

```bash
# 이들은 보통 플랫폼에 의해 자동으로 설정됩니다:
DATABASE_URL=postgres://user:pass@host:5432/db
REDIS_URL=redis://host:6379
PORT=3000

# Base64로 설정을 지정:
CONFIG_BASE64=c2VydmVyOgogIGhvc3Q6IC...

# 또는 개별 값을 오버라이드:
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
      # 또는 설정 파일 위에 개별 값을 오버라이드:
      REDIRECTOR__RATE_LIMIT__REQUESTS_PER_SECOND: "2000"
      REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD: "${METRICS_PASSWORD}"
    volumes:
      - ./config.yaml:/app/config.yaml  # CONFIG_BASE64 사용 시 선택사항
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

**일반 Docker**:

```bash
docker run -p 8080:8080 \
  -e DATABASE_URL="postgres://user:pass@host:5432/db" \
  -e REDIS_URL="redis://host:6379" \
  -e CONFIG_BASE64="$(cat config.yaml | base64)" \
  ghcr.io/brilliant-almazov/redirector:latest
```

**최소 설정 (환경 변수만, 설정 파일 없음)**:

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

#### Base64 설정

설정 파일 마운트가 실용적이지 않은 환경(PaaS, 서버리스, CI/CD)에서는 전체 설정을 Base64 인코딩 문자열로 전달하세요:

```bash
# 인코딩
cat config.yaml | base64

# 디코딩 (확인용)
echo "$CONFIG_BASE64" | base64 -d
```

`CONFIG_BASE64`는 `CONFIG_PATH`보다 우선합니다. 환경 변수 오버라이드 (`REDIRECTOR__*` 및 PaaS 변수)는 디코딩된 설정 **위에** 적용됩니다.

## 작동 방식

1. 사용자가 `/r/{hashid}` 방문 (예: `/r/abc123`)
2. 서비스가 hashid를 숫자 ID로 디코딩
3. Redis 캐시에서 URL 확인
4. 캐시 미스 시 PostgreSQL 쿼리
5. 결과를 Redis에 캐싱
6. 카운트다운이 있는 인터스티셜 페이지 표시
7. 카운트다운 후 대상 URL로 리다이렉트

## 엔드포인트

| 엔드포인트 | 인증 | 설명 |
|------------|------|------|
| `GET /` | 없음 | 인덱스 페이지 |
| `GET /r/{hashid}` | 없음 | 인터스티셜 페이지와 함께 리다이렉트 |
| `GET /d/{hashid}` | 없음 | 데모 리다이렉트 (합성 부하 테스트) |
| `GET /health` | 없음 | 헬스 체크 |
| `GET /metrics` | Basic | Prometheus 메트릭 |
| `GET /admin` | Session | 관리 대시보드 로그인 |
| `GET /admin/dashboard` | Session | 관리 대시보드 |

## 관리 대시보드

서비스에는 실시간 메트릭 모니터링을 위한 선택적 관리 대시보드가 포함되어 있습니다.

### 설정

1. **비밀번호 해시 생성:**

```bash
cargo run --bin hash_password
# 비밀번호 입력, 또는:
cargo run --bin hash_password -- "your-password"
```

2. **config.yaml에 추가:**

```yaml
admin:
  enabled: true
  session_ttl_hours: 24
  users:
    - username: admin
      password_hash: "$argon2id$v=19$m=19456,t=2,p=1$..."  # 1단계에서
```

3. **대시보드 접근:**

`http://localhost:8080/admin`을 열고 자격 증명으로 로그인합니다.

### 기능

- 실시간 RPS 및 지연 차트
- 시스템 메트릭 (CPU, 메모리, 가동 시간)
- 캐시 히트율 모니터링
- 최근 리다이렉트 목록
- 테스트용 부하 시뮬레이션
- 세 가지 테마: 라이트, 다크, 웜

## 이벤트 분석

리다이렉트 분석을 위한 선택적 이벤트 퍼블리싱 파이프라인. 활성화하면 각 리다이렉트 이벤트가 RabbitMQ에 게시되고 별도의 바이너리에서 PostgreSQL에 기록합니다.

> **전체 문서**: [EVENT_ANALYTICS.md](EVENT_ANALYTICS.md)

### 기능

- **Fire-and-forget 게시** — 리다이렉트 지연 시간이 큐 가용성에 영향받지 않음
- **배치 처리** — 크기(100개) 또는 시간(1초)별로 이벤트 그룹화
- **User-Agent 분석** — woothee를 통한 브라우저, 버전, OS, 장치 유형
- **GeoIP 보강** — IP에서 국가 및 도시 (MaxMind mmdb, 핫 리로드 지원)
- **참조 중복 제거** — referer 및 user agent의 MD5 기반 중복 제거
- **월별 파티셔닝** — `redirect_events` 테이블 자동 파티션 생성

### 아키텍처

```
리다이렉트 핸들러
    │
    ├── try_send(RedirectEvent) ──► [tokio::mpsc 채널]
    │   (비블로킹,                      │
    │    fire-and-forget)              ▼
    │                             백그라운드 태스크
    │                             (크기/시간별 배치)
    │                                     │
    │                                     ▼
    │                             [RabbitMQ 큐]
    │                                     │
    │                                     ▼
    │                             이벤트 컨슈머
    │                             (별도 바이너리/컨테이너)
    │                                     │
    │                                     ▼
    │                             [PostgreSQL 분석]
    │                             (월별 파티션)
```

### 빠른 시작

```bash
# config.yaml에서 활성화
events:
  enabled: true
  rabbitmq:
    url: amqp://guest:guest@localhost:5672/%2f

# 또는 환경 변수로
REDIRECTOR__EVENTS__ENABLED=true
RABBITMQ_URL=amqp://guest:guest@localhost:5672/%2f

# 컨슈머 실행
RABBITMQ_URL=amqp://... DATABASE_URL=postgres://... cargo run --bin event_consumer
```

### 이벤트 메트릭

선택적 이벤트 분석은 다음 메트릭을 제공합니다:

| 메트릭 | 설명 |
|--------|------|
| `events_published_total` | 게시된 총 리다이렉트 이벤트 |
| `events_consumed_total` | 컨슈된 총 리다이렉트 이벤트 |
| `events_consume_lag_seconds` | 컨슈 레그 (대기 중인 메시지) |
| `events_duplicate_referers` | 기록된 중복 참조 개수 |
| `geoip_lookups_total` | GeoIP 조회 총 개수 |

## 라이선스

MIT 라이선스 - 자세한 내용은 [LICENSE](../LICENSE)를 참조하세요.

## 기여

기여를 환영합니다! 다음 단계를 따라주세요:

1. 저장소를 포크
2. 기능 브랜치 생성
3. Pull Request 제출

보호된 master 브랜치는 PR 리뷰가 필요합니다.
