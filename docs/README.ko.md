# redirector

[English](../README.md) | [Русский](README.ru.md) | [中文](README.zh.md) | [हिंदी](README.hi.md) | [Español](README.es.md) | [Português](README.pt.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | **한국어** | [Polski](README.pl.md) | [Nederlands](README.nl.md) | [Italiano](README.it.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) | [Bahasa Indonesia](README.id.md) | [Tiếng Việt](README.vi.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

인터스티셜 페이지와 hashid 기반 짧은 링크를 갖춘 안전한 URL 리다이렉트 서비스.

## 문제

긴 URL을 공유하는 것은 불편합니다. URL 단축기가 존재하지만, 대부분 즉시 리다이렉트되어 보안 위험이 될 수 있습니다. 사용자는 리다이렉트되기 전에 어디로 가는지 볼 수 있어야 합니다.

**redirector**는 안전한 리다이렉트를 제공합니다:
- 리다이렉트 전 대상 URL을 보여주는 인터스티셜 페이지
- 사용자 인식을 위한 카운트다운 타이머
- 아름답고 브랜딩된 페이지

## 기능

- 🔗 **Hashid URL** - 짧고 고유하며 비순차적인 ID (예: `/r/abc123`)
- ⏱️ **인터스티셜 페이지** - 리다이렉트 전 대상 URL을 보여주는 카운트다운
- ⚡ **Redis 캐싱** - 구성 가능한 TTL로 빠른 조회
- 🛡️ **서킷 브레이커** - 연쇄 장애로부터 데이터베이스 보호
- 🚦 **속도 제한** - 전역 및 데이터베이스 수준 속도 제한
- 📊 **Prometheus 메트릭** - Basic Auth 보호를 통한 완전한 관찰성
- 🎨 **아름다운 페이지** - 깔끔한 404 및 인덱스 페이지
- 🔑 **다중 솔트** - 마이그레이션을 위한 hashid 솔트 로테이션 지원

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

## 작동 방식

1. 사용자가 `/r/{hashid}` 방문 (예: `/r/abc123`)
2. 서비스가 hashid를 숫자 ID로 디코딩
3. Redis 캐시에서 URL 확인
4. 캐시 미스 시 PostgreSQL 쿼리
5. 결과를 Redis에 캐시
6. 카운트다운이 있는 인터스티셜 페이지 표시
7. 카운트다운 후 대상 URL로 리다이렉트

## 엔드포인트

| 엔드포인트 | 인증 | 설명 |
|----------|------|-------------|
| `GET /` | 없음 | 인덱스 페이지 |
| `GET /r/{hashid}` | 없음 | 인터스티셜과 함께 리다이렉트 |
| `GET /health` | 없음 | 상태 확인 |
| `GET /metrics` | Basic | Prometheus 메트릭 |
| `GET /admin` | Session | 관리자 패널 로그인 |
| `GET /admin/dashboard` | Session | 관리자 패널 |

## 관리자 패널

서비스에는 실시간 메트릭 모니터링을 위한 선택적 관리자 패널이 포함되어 있습니다.

### 설정

1. **비밀번호 해시 생성:**

```bash
# Rust 사용
cargo run --bin hash_password

# 또는 Python 사용 (pip install argon2-cffi)
./scripts/hash_password.sh
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

3. **패널 접속:**

`http://localhost:8080/admin`을 열고 자격 증명으로 로그인합니다.

### 기능

- 실시간 RPS 및 지연 시간 차트
- 시스템 메트릭 (CPU, 메모리, 가동 시간)
- 캐시 적중률 모니터링
- 최근 리다이렉트 목록
- 테스트용 부하 시뮬레이션
- 세 가지 테마: 라이트, 다크, 웜

## 라이선스

MIT 라이선스 - 자세한 내용은 [LICENSE](../LICENSE) 참조.
