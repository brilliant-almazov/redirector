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

## 许可证

MIT许可证 - 详见 [LICENSE](../LICENSE)。
