# redirector

[English](../README.md) | [Русский](README.ru.md) | [中文](README.zh.md) | [हिंदी](README.hi.md) | [Español](README.es.md) | [Português](README.pt.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | **日本語** | [한국어](README.ko.md) | [Polski](README.pl.md) | [Nederlands](README.nl.md) | [Italiano](README.it.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) | [Bahasa Indonesia](README.id.md) | [Tiếng Việt](README.vi.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

インタースティシャルページとhashidベースの短縮リンクを備えた安全なURLリダイレクトサービス。

## 問題

長いURLを共有するのは不便です。URL短縮サービスは存在しますが、多くの場合すぐにリダイレクトされ、セキュリティリスクになる可能性があります。ユーザーはリダイレクトされる前に行き先を確認できるべきです。

**redirector** は安全なリダイレクトを提供します：
- リダイレクト前にターゲットURLを表示するインタースティシャルページ
- ユーザー認識のためのカウントダウンタイマー
- 美しくブランド化されたページ

## 機能

- 🔗 **Hashid URL** - 短く、ユニークで、非連続のID（例：`/r/abc123`）
- ⏱️ **インタースティシャルページ** - リダイレクト前にターゲットURLを表示するカウントダウン
- ⚡ **Redisキャッシング** - 設定可能なTTLによる高速ルックアップ
- 🛡️ **サーキットブレーカー** - カスケード障害に対するデータベース保護
- 🚦 **レート制限** - グローバルおよびデータベースレベルのレート制限
- 📊 **Prometheusメトリクス** - Basic Auth保護による完全な可観測性
- 🎨 **美しいページ** - クリーンな404とインデックスページ
- 🔑 **複数のソルト** - 移行用のhashidソルトローテーションサポート

## クイックスタート

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

## 仕組み

1. ユーザーが `/r/{hashid}` にアクセス（例：`/r/abc123`）
2. サービスがhashidを数値IDにデコード
3. RedisキャッシュでURLを確認
4. キャッシュミスの場合、PostgreSQLにクエリ
5. 結果をRedisにキャッシュ
6. カウントダウン付きのインタースティシャルページを表示
7. カウントダウン後、ターゲットURLにリダイレクト

## ライセンス

MITライセンス - 詳細は [LICENSE](../LICENSE) を参照。
