# redirector

[English](../README.md) | [Русский](README.ru.md) | **中文** | [हिंदी](README.hi.md) | [Español](README.es.md) | [Português](README.pt.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Polski](README.pl.md) | [Nederlands](README.nl.md) | [Italiano](README.it.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) | [Bahasa Indonesia](README.id.md) | [Tiếng Việt](README.vi.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

安全的URL重定向服务，带有过渡页面和基于hashid的短链接。

## 问题

分享长URL很不方便。URL缩短器存在，但通常会立即重定向，这可能是安全风险。用户应该在被重定向之前看到他们要去的地方。

**redirector** 提供安全的重定向：
- 过渡页面在重定向前显示目标URL
- 倒计时器提醒用户
- 美观的品牌页面

## 功能特性

- 🔗 **Hashid URLs** - 短的、唯一的、非顺序的ID（如 `/r/abc123`）
- ⏱️ **过渡页面** - 倒计时器在重定向前显示目标URL
- ⚡ **Redis缓存** - 可配置TTL的快速查询
- 🛡️ **熔断器** - 防止级联故障的数据库保护
- 🚦 **速率限制** - 全局和数据库级别的速率限制
- 📊 **Prometheus指标** - 带Basic Auth保护的完整可观测性
- 🎨 **美观页面** - 简洁的404和索引页面
- 🔑 **多盐值** - 支持hashid盐值轮换以便迁移

## 截图

| 浅色 | 深色 | 暖色 |
|------|------|------|
| ![仪表盘浅色](screenshots/dashboard-light.png) | ![仪表盘深色](screenshots/dashboard-dark.png) | ![仪表盘暖色](screenshots/dashboard-warm.png) |
| ![登录浅色](screenshots/login-light.png) | ![登录深色](screenshots/login-dark.png) | ![登录暖色](screenshots/login-warm.png) |
| ![404浅色](screenshots/not-found-light.png) | ![404深色](screenshots/not-found-dark.png) | ![404暖色](screenshots/not-found-warm.png) |

| 首页 | 中间页 |
|------|--------|
| ![首页](screenshots/index.png) | ![中间页](screenshots/interstitial.png) |

## 快速开始

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

## 工作原理

1. 用户访问 `/r/{hashid}`（如 `/r/abc123`）
2. 服务将hashid解码为数字ID
3. 检查Redis缓存中的URL
4. 缓存未命中时，查询PostgreSQL
5. 将结果缓存到Redis
6. 显示带倒计时的过渡页面
7. 倒计时结束后，重定向到目标URL

## 端点

| 端点 | 认证 | 描述 |
|------|------|------|
| `GET /` | 无 | 首页 |
| `GET /r/{hashid}` | 无 | 带过渡页面的重定向 |
| `GET /health` | 无 | 健康检查 |
| `GET /metrics` | Basic | Prometheus指标 |
| `GET /admin` | Session | 管理面板登录 |
| `GET /admin/dashboard` | Session | 管理面板 |

## 管理面板

服务包含可选的管理面板，用于实时监控指标。

### 设置

1. **生成密码哈希：**

```bash
# 使用 Rust
cargo run --bin hash_password

# 或使用 Python (pip install argon2-cffi)
./scripts/hash_password.sh
```

2. **添加到 config.yaml：**

```yaml
admin:
  enabled: true
  session_ttl_hours: 24
  users:
    - username: admin
      password_hash: "$argon2id$v=19$m=19456,t=2,p=1$..."  # 来自步骤1
```

3. **访问面板：**

打开 `http://localhost:8080/admin` 并使用您的凭据登录。

### 功能

- 实时 RPS 和延迟图表
- 系统指标（CPU、内存、运行时间）
- 缓存命中率监控
- 最近重定向列表
- 负载模拟测试
- 三种主题：浅色、深色、暖色

## 指标

服务在 `/metrics` 端点暴露 Prometheus 指标（需要 Basic Auth）。

## 技术栈

- **语言**: Rust（基于 Tokio 的异步）
- **Web 框架**: Axum
- **缓存**: Redis-compatible (Redis, Dragonfly, Valkey, KeyDB)
- **数据库**: PostgreSQL（可插拔存储层）
- **指标**: Prometheus + metrics-rs
- **密码哈希**: Argon2

> **注意**: PostgreSQL 作为默认存储后端。存储层是抽象的，可以替换为任何数据源。项目正在积极开发中。

## 许可证

MIT许可证 - 详见 [LICENSE](../LICENSE)。
