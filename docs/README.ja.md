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

## エンドポイント

| エンドポイント | 認証 | 説明 |
|----------|------|-------------|
| `GET /` | なし | インデックスページ |
| `GET /r/{hashid}` | なし | インタースティシャル付きリダイレクト |
| `GET /health` | なし | ヘルスチェック |
| `GET /metrics` | Basic | Prometheusメトリクス |
| `GET /admin` | Session | 管理パネルログイン |
| `GET /admin/dashboard` | Session | 管理パネル |

## 管理パネル

サービスにはリアルタイムメトリクス監視用のオプションの管理パネルが含まれています。

### セットアップ

1. **パスワードハッシュを生成：**

```bash
# Rustを使用
cargo run --bin hash_password

# またはPythonを使用 (pip install argon2-cffi)
./scripts/hash_password.sh
```

2. **config.yamlに追加：**

```yaml
admin:
  enabled: true
  session_ttl_hours: 24
  users:
    - username: admin
      password_hash: "$argon2id$v=19$m=19456,t=2,p=1$..."  # ステップ1から
```

3. **パネルにアクセス：**

`http://localhost:8080/admin`を開き、認証情報でログインします。

### 機能

- リアルタイムRPSとレイテンシグラフ
- システムメトリクス（CPU、メモリ、稼働時間）
- キャッシュヒット率監視
- 最近のリダイレクト一覧
- テスト用負荷シミュレーション
- 3つのテーマ：ライト、ダーク、ウォーム

## ライセンス

MITライセンス - 詳細は [LICENSE](../LICENSE) を参照。
