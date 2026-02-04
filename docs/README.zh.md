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
- 🎨 **美观页面** - 简洁的 404 和索引页面，支持 4 种主题
- 🔑 **多盐值** - 支持 hashid 盐值轮换以便迁移
- 📱 **管理面板** - 通过 SSE 实时监控指标

## 截图

| 浅色 | 深色 | 灰色 | 暖色 |
|------|------|------|------|
| ![仪表盘浅色](screenshots/dashboard-light.png) | ![仪表盘深色](screenshots/dashboard-dark.png) | ![仪表盘灰色](screenshots/dashboard-gray.png) | ![仪表盘暖色](screenshots/dashboard-warm.png) |
| ![登录浅色](screenshots/login-light.png) | ![登录深色](screenshots/login-dark.png) | ![登录灰色](screenshots/login-gray.png) | ![登录暖色](screenshots/login-warm.png) |
| ![Index Light](screenshots/index-light.png) | ![Index Dark](screenshots/index-dark.png) | ![Index Gray](screenshots/index-gray.png) | ![Index Warm](screenshots/index-warm.png) |
| ![Interstitial Light](screenshots/interstitial-light.png) | ![Interstitial Dark](screenshots/interstitial-dark.png) | ![Interstitial Gray](screenshots/interstitial-gray.png) | ![Interstitial Warm](screenshots/interstitial-warm.png) |
| ![404 Light](screenshots/404-light.png) | ![404 Dark](screenshots/404-dark.png) | ![404 Gray](screenshots/404-gray.png) | ![404 Warm](screenshots/404-warm.png) |

### 负载测试模态框

| 浅色 | 深色 | 灰色 | 暖色 |
|------|------|------|------|
| ![模态框浅色](screenshots/dashboard-modal-light.png) | ![模态框深色](screenshots/dashboard-modal-dark.png) | ![模态框灰色](screenshots/dashboard-modal-gray.png) | ![模态框暖色](screenshots/dashboard-modal-warm.png) |

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

### 环境变量

配置服务有**三种方式**，按优先级排列（从高到低）：

| 优先级 | 方式 | 使用场景 |
|--------|------|----------|
| 1 | `REDIRECTOR__*` 环境变量 | 覆盖单个值 |
| 2 | 标准 PaaS 环境变量（`DATABASE_URL` 等） | PaaS 平台（Railway、Heroku、Render） |
| 3 | 配置文件（`config.yaml` 或 `CONFIG_BASE64`） | 基础配置 |

#### 特殊变量

| 变量 | 默认值 | 描述 |
|------|--------|------|
| `CONFIG_PATH` | `config.yaml` | YAML 配置文件路径 |
| `CONFIG_BASE64` | — | Base64 编码的 YAML 配置（优先于 `CONFIG_PATH`） |

#### 标准 PaaS 环境变量

这些变量会被自动识别和应用。大多数 PaaS 平台会为您自动设置：

| 变量 | 配置路径 | 示例 |
|------|----------|------|
| `DATABASE_URL` | `database.url` | `postgres://user:pass@host:5432/db` |
| `REDIS_URL` | `redis.url` | `redis://host:6379` |
| `PORT` | `server.port` | `3000` |
| `HASHIDS_SALTS` | `hashids.salts` | `new-salt,old-salt`（逗号分隔） |

> **优先级规则**：如果同时设置了 `DATABASE_URL` 和 `REDIRECTOR__DATABASE__URL`，带 `REDIRECTOR__` 前缀的版本优先。同样，`REDIRECTOR__HASHIDS__SALTS__0` 优先于 `HASHIDS_SALTS`。

#### 前缀环境变量（`REDIRECTOR__*`）

任何配置值都可以使用 `REDIRECTOR__` 前缀覆盖，使用 `__`（双下划线）作为嵌套分隔符。以下是所有可覆盖变量的**完整参考**：

##### Server

| 环境变量 | 配置路径 | 默认值 | 描述 |
|----------|----------|--------|------|
| `REDIRECTOR__SERVER__HOST` | `server.host` | `0.0.0.0` | 绑定地址 |
| `REDIRECTOR__SERVER__PORT` | `server.port` | `8080` | HTTP 端口 |

##### Hashids

| 环境变量 | 配置路径 | 默认值 | 描述 |
|----------|----------|--------|------|
| `REDIRECTOR__HASHIDS__SALTS__0` | `hashids.salts[0]` | *必填* | 主 hashid 盐值 |
| `REDIRECTOR__HASHIDS__SALTS__1` | `hashids.salts[1]` | — | 旧盐值（用于迁移） |
| `REDIRECTOR__HASHIDS__MIN_LENGTH` | `hashids.min_length` | `6` | 最小 hashid 长度 |

> **数组**：列表元素通过 `__0`、`__1`、`__2` 等索引。对于 hashid 盐值轮换，将 `__0` 设为新盐值，`__1` 设为旧盐值。

##### Redis / 缓存

| 环境变量 | 配置路径 | 默认值 | 描述 |
|----------|----------|--------|------|
| `REDIRECTOR__REDIS__URL` | `redis.url` | *必填* | Redis 连接 URL |
| `REDIRECTOR__REDIS__CACHE_TTL_SECONDS` | `redis.cache_ttl_seconds` | `86400` | 缓存 TTL（秒）。`86400` = 24 小时 |

##### 数据库

| 环境变量 | 配置路径 | 默认值 | 描述 |
|----------|----------|--------|------|
| `REDIRECTOR__DATABASE__URL` | `database.url` | *必填* | PostgreSQL 连接 URL |
| `REDIRECTOR__DATABASE__POOL__MAX_CONNECTIONS` | `database.pool.max_connections` | `3` | 连接池大小 |
| `REDIRECTOR__DATABASE__POOL__CONNECT_TIMEOUT_SECONDS` | `database.pool.connect_timeout_seconds` | `3` | 连接超时（秒） |
| `REDIRECTOR__DATABASE__RATE_LIMIT__MAX_REQUESTS_PER_SECOND` | `database.rate_limit.max_requests_per_second` | `50` | 每秒最大数据库查询数 |
| `REDIRECTOR__DATABASE__CIRCUIT_BREAKER__FAILURE_THRESHOLD` | `database.circuit_breaker.failure_threshold` | `3` | 熔断器打开前的连续失败次数 |
| `REDIRECTOR__DATABASE__CIRCUIT_BREAKER__RESET_TIMEOUT_SECONDS` | `database.circuit_breaker.reset_timeout_seconds` | `60` | 半开重试前的等待秒数 |
| `REDIRECTOR__DATABASE__QUERY__TABLE` | `database.query.table` | `dictionary.urls` | URL 查询表名 |
| `REDIRECTOR__DATABASE__QUERY__ID_COLUMN` | `database.query.id_column` | `id` | 数字 ID 列名 |
| `REDIRECTOR__DATABASE__QUERY__URL_COLUMN` | `database.query.url_column` | `name` | 目标 URL 列名 |

##### 过渡页面

| 环境变量 | 配置路径 | 默认值 | 描述 |
|----------|----------|--------|------|
| `REDIRECTOR__INTERSTITIAL__DELAY_SECONDS` | `interstitial.delay_seconds` | `5` | 重定向前的倒计时 |

##### 指标

| 环境变量 | 配置路径 | 默认值 | 描述 |
|----------|----------|--------|------|
| `REDIRECTOR__METRICS__BASIC_AUTH__USERNAME` | `metrics.basic_auth.username` | *必填* | `/metrics` 端点的用户名 |
| `REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD` | `metrics.basic_auth.password` | *必填* | `/metrics` 端点的密码 |

##### 速率限制（全局）

| 环境变量 | 配置路径 | 默认值 | 描述 |
|----------|----------|--------|------|
| `REDIRECTOR__RATE_LIMIT__REQUESTS_PER_SECOND` | `rate_limit.requests_per_second` | `1000` | 每秒最大请求数 |
| `REDIRECTOR__RATE_LIMIT__BURST` | `rate_limit.burst` | `100` | 超过 RPS 限制的突发允许量 |

##### 管理面板

| 环境变量 | 配置路径 | 默认值 | 描述 |
|----------|----------|--------|------|
| `REDIRECTOR__ADMIN__ENABLED` | `admin.enabled` | `false` | 启用管理面板 |
| `REDIRECTOR__ADMIN__SESSION_SECRET` | `admin.session_secret` | `change-me-...` | 会话签名密钥（最少 32 个字符） |
| `REDIRECTOR__ADMIN__SESSION_TTL_HOURS` | `admin.session_ttl_hours` | `24` | 会话有效期（小时） |

> **注意**：管理用户（`admin.users`）的 `username` 和 `password_hash` 由于结构复杂，无法通过环境变量设置。请在配置文件或 `CONFIG_BASE64` 中定义。

#### 按部署平台的示例

**Railway / Render / Fly.io**（带托管数据库的 PaaS）：

```bash
# 这些通常由平台自动设置：
DATABASE_URL=postgres://user:pass@host:5432/db
REDIS_URL=redis://host:6379
PORT=3000

# 通过 base64 设置配置：
CONFIG_BASE64=c2VydmVyOgogIGhvc3Q6IC...

# 或覆盖单个值：
REDIRECTOR__HASHIDS__SALTS__0=my-secret-salt
REDIRECTOR__METRICS__BASIC_AUTH__USERNAME=prometheus
REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD=strong-password
REDIRECTOR__ADMIN__ENABLED=true
REDIRECTOR__ADMIN__SESSION_SECRET=random-32-byte-secret-for-sessions
```

**Docker Compose（包含所有覆盖的完整示例）**：

```yaml
services:
  redirector:
    image: ghcr.io/brilliant-almazov/redirector:latest
    ports:
      - "8080:8080"
    environment:
      # --- 连接 URL（PaaS 风格）---
      DATABASE_URL: "postgres://redirector:${DB_PASSWORD}@postgres:5432/redirector"
      REDIS_URL: "redis://redis:6379"

      # --- 配置文件 ---
      CONFIG_BASE64: "${CONFIG_BASE64}"

      # --- 服务器 ---
      REDIRECTOR__SERVER__HOST: "0.0.0.0"
      REDIRECTOR__SERVER__PORT: "8080"

      # --- Hashid 盐值 ---
      REDIRECTOR__HASHIDS__SALTS__0: "${HASHID_SALT}"        # 主盐值
      REDIRECTOR__HASHIDS__SALTS__1: "${HASHID_SALT_OLD}"    # 迁移用旧盐值
      REDIRECTOR__HASHIDS__MIN_LENGTH: "6"

      # --- Redis 缓存 ---
      REDIRECTOR__REDIS__CACHE_TTL_SECONDS: "43200"          # 12 小时

      # --- 数据库连接池和容错 ---
      REDIRECTOR__DATABASE__POOL__MAX_CONNECTIONS: "5"
      REDIRECTOR__DATABASE__POOL__CONNECT_TIMEOUT_SECONDS: "5"
      REDIRECTOR__DATABASE__RATE_LIMIT__MAX_REQUESTS_PER_SECOND: "100"
      REDIRECTOR__DATABASE__CIRCUIT_BREAKER__FAILURE_THRESHOLD: "5"
      REDIRECTOR__DATABASE__CIRCUIT_BREAKER__RESET_TIMEOUT_SECONDS: "30"

      # --- 自定义表映射 ---
      REDIRECTOR__DATABASE__QUERY__TABLE: "public.short_urls"
      REDIRECTOR__DATABASE__QUERY__ID_COLUMN: "id"
      REDIRECTOR__DATABASE__QUERY__URL_COLUMN: "target_url"

      # --- 过渡页面 ---
      REDIRECTOR__INTERSTITIAL__DELAY_SECONDS: "3"

      # --- 指标认证 ---
      REDIRECTOR__METRICS__BASIC_AUTH__USERNAME: "prometheus"
      REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD: "${METRICS_PASSWORD}"

      # --- 全局速率限制 ---
      REDIRECTOR__RATE_LIMIT__REQUESTS_PER_SECOND: "2000"
      REDIRECTOR__RATE_LIMIT__BURST: "200"

      # --- 管理面板 ---
      REDIRECTOR__ADMIN__ENABLED: "true"
      REDIRECTOR__ADMIN__SESSION_SECRET: "${SESSION_SECRET}"
      REDIRECTOR__ADMIN__SESSION_TTL_HOURS: "8"
    depends_on:
      - postgres
      - redis

  postgres:
    image: postgres:16-alpine
    environment:
      POSTGRES_USER: redirector
      POSTGRES_PASSWORD: ${DB_PASSWORD}
      POSTGRES_DB: redirector

  redis:
    image: redis:7-alpine
```

**Kubernetes**：

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
            - name: REDIRECTOR__HASHIDS__SALTS__0
              valueFrom:
                secretKeyRef:
                  name: redirector-secrets
                  key: hashid-salt
            - name: REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD
              valueFrom:
                secretKeyRef:
                  name: redirector-secrets
                  key: metrics-password
            - name: REDIRECTOR__ADMIN__SESSION_SECRET
              valueFrom:
                secretKeyRef:
                  name: redirector-secrets
                  key: session-secret
            - name: CONFIG_BASE64
              valueFrom:
                configMapKeyRef:
                  name: redirector-config
                  key: config-base64
```

**纯 Docker（单条命令）**：

```bash
docker run -p 8080:8080 \
  -e DATABASE_URL="postgres://user:pass@host:5432/db" \
  -e REDIS_URL="redis://host:6379" \
  -e REDIRECTOR__HASHIDS__SALTS__0="my-secret-salt" \
  -e REDIRECTOR__METRICS__BASIC_AUTH__USERNAME="prometheus" \
  -e REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD="strong-password" \
  -e REDIRECTOR__INTERSTITIAL__DELAY_SECONDS="3" \
  -e CONFIG_BASE64="$(cat config.yaml | base64)" \
  ghcr.io/brilliant-almazov/redirector:latest
```

**最小化配置（仅环境变量，无配置文件）**：

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

#### 通过环境变量进行盐值轮换

轮换 hashid 盐值时，服务按顺序尝试盐值 -- 第一个匹配的获胜。将新盐值排在前面以便新链接使用它，同时保留旧盐值以实现向后兼容：

**方式 1：使用逗号分隔的单个变量**（推荐）：

```bash
# 轮换前
HASHIDS_SALTS=original-salt

# 轮换后 -- 新盐值在前，旧盐值用于现有链接
HASHIDS_SALTS=new-salt,original-salt
```

**方式 2：索引变量**：

```bash
# 轮换前
REDIRECTOR__HASHIDS__SALTS__0=original-salt

# 轮换后
REDIRECTOR__HASHIDS__SALTS__0=new-salt
REDIRECTOR__HASHIDS__SALTS__1=original-salt
```

> **注意**：如果设置了 `REDIRECTOR__HASHIDS__SALTS__0`，`HASHIDS_SALTS` 将被忽略。

#### Base64 配置

对于无法挂载配置文件的环境（PaaS、serverless、CI/CD），可以将整个配置作为 base64 编码字符串传递：

```bash
# Encode
cat config.yaml | base64

# Decode（用于验证）
echo "$CONFIG_BASE64" | base64 -d
```

`CONFIG_BASE64` 优先于 `CONFIG_PATH`。环境变量覆盖（`REDIRECTOR__*` 和 PaaS 变量）在解码后的配置**之上**应用。

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
