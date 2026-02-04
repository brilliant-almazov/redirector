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

#### Base64 설정

설정 파일 마운트가 불가능한 환경(예: Railway, 서버리스):

```bash
# Encode
cat config.yaml | base64

# Run with base64 config
CONFIG_BASE64="c2VydmVyOgogIGhvc3Q6IC..." docker run ghcr.io/brilliant-almazov/redirector:latest
```

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

## 라이선스

MIT 라이선스 - 자세한 내용은 [LICENSE](../LICENSE)를 참조하세요.

## 기여

기여를 환영합니다! 다음 단계를 따라주세요:

1. 저장소를 포크
2. 기능 브랜치 생성
3. Pull Request 제출

보호된 master 브랜치는 PR 리뷰가 필요합니다.
