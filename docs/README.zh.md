# redirector

> **高性能 URL 缩短和重定向服务**，基于 Rust、Axum、Redis 和 PostgreSQL 构建。具有安全的过渡页面、实时管理面板和企业级可观测性。

[English](../README.md) | [Русский](README.ru.md) | **中文** | [हिंदी](README.hi.md) | [Español](README.es.md) | [Português](README.pt.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Polski](README.pl.md) | [Nederlands](README.nl.md) | [Italiano](README.it.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) | [עברית](README.he.md) | [Bahasa Indonesia](README.id.md) | [Tiếng Việt](README.vi.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![Docker Image Size](https://ghcr-badge.egpl.dev/brilliant-almazov/redirector/size)](https://github.com/brilliant-almazov/redirector/pkgs/container/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

**关键词**: URL缩短器, 短链接, 重定向服务, Rust Web服务, Axum框架, Redis缓存, PostgreSQL, Prometheus指标, hashids, 短链接, 过渡页面, 安全重定向, 高性能, 微服务

安全的 URL 重定向服务，具有过渡页面和基于 hashid 的短链接。非常适合内部工具、企业链接管理和品牌短 URL 服务。

### 性能

| 场景 | RPS | 平均延迟 | P99 延迟 |
|------|-----|----------|----------|
| 100% 缓存命中 | **7,800+** | ~14ms | ~50ms |
| 缓存未命中 (10K URLs) | **2,300+** | ~44ms | ~81ms |

**测试条件**: wrk -t4 -c100 -d30s, PostgreSQL 15, Dragonfly (Redis), macOS M1 (Docker)

> ⚠️ 结果来自 macOS Docker，有虚拟化开销。原生 Linux 部署预计 **快 3-5 倍**。

## 问题

分享长 URL 很不方便。URL 缩短器存在，但通常会立即重定向，这可能是安全风险。用户应该在被重定向之前看到他们要去的地方。

**redirector** 提供安全的重定向：
- 过渡页面在重定向前显示目标 URL
- 倒计时器提醒用户
- 美观的品牌页面

## 功能特性

- 🔗 **Hashid URLs** - 短的、唯一的、非顺序的 ID（如 `/r/abc123`）
- ⏱️ **过渡页面** - 倒计时器在重定向前显示目标 URL
- ⚡ **Redis 缓存** - 可配置 TTL 的快速查询
- 🛡️ **熔断器** - 防止级联故障的数据库保护
- 🚦 **速率限制** - 全局和数据库级别的速率限制
- 📊 **Prometheus 指标** - 带 Basic Auth 保护的完整可观测性
- 🎨 **美观页面** - 简洁的 404 和索引页面，支持 3 种主题
- 🔑 **多盐值** - 支持 hashid 盐值轮换以便迁移
- 📱 **管理面板** - 通过 SSE 实时监控指标

## 截图

| 浅色 | 深色 | 暖色 |
|------|------|------|
| ![仪表盘浅色](screenshots/dashboard-light.png) | ![仪表盘深色](screenshots/dashboard-dark.png) | ![仪表盘暖色](screenshots/dashboard-warm.png) |
| ![登录浅色](screenshots/login-light.png) | ![登录深色](screenshots/login-dark.png) | ![登录暖色](screenshots/login-warm.png) |
| ![404浅色](screenshots/not-found-light.png) | ![404深色](screenshots/not-found-dark.png) | ![404暖色](screenshots/not-found-warm.png) |

| 首页 | 过渡页 |
|------|--------|
| ![首页](screenshots/index.png) | ![过渡页](screenshots/interstitial.png) |

## 技术栈

- **语言**: Rust（异步，使用 Tokio）
- **Web 框架**: Axum
- **缓存**: Redis 兼容（Redis、Dragonfly、Valkey、KeyDB 等）
- **数据库**: PostgreSQL（可插拔存储层）
- **指标**: Prometheus + metrics-rs
- **密码哈希**: Argon2

> **注意**: 存储和缓存层是抽象的，可以用任何兼容的数据源替换。目前正在积极开发中。

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

1. 用户访问 `/r/{hashid}`（例如 `/r/abc123`）
2. 服务将 hashid 解码为数字 ID
3. 检查 Redis 缓存中的 URL
4. 缓存未命中时，查询 PostgreSQL
5. 将结果缓存到 Redis
6. 显示带倒计时的过渡页面
7. 倒计时结束后重定向到目标 URL

## 端点

| 端点 | 认证 | 描述 |
|------|------|------|
| `GET /` | 否 | 首页 |
| `GET /r/{hashid}` | 否 | 带过渡页面的重定向 |
| `GET /d/{hashid}` | 否 | 演示重定向（合成负载测试） |
| `GET /health` | 否 | 健康检查 |
| `GET /metrics` | Basic | Prometheus 指标 |
| `GET /admin` | Session | 管理面板登录 |
| `GET /admin/dashboard` | Session | 管理面板 |

## 管理面板

该服务包含一个可选的管理面板，用于实时监控指标。

### 设置

1. **生成密码哈希：**

```bash
cargo run --bin hash_password
# 输入密码，或：
cargo run --bin hash_password -- "your-password"
```

2. **添加到 config.yaml：**

```yaml
admin:
  enabled: true
  session_ttl_hours: 24
  users:
    - username: admin
      password_hash: "$argon2id$v=19$m=19456,t=2,p=1$..."  # 来自步骤 1
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

## 许可证

MIT 许可证 - 详见 [LICENSE](../LICENSE)。

## 贡献

欢迎贡献！请：

1. Fork 仓库
2. 创建功能分支
3. 提交 Pull Request

受保护的 master 分支需要 PR 审查。
