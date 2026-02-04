# redirector

> **高性能URLショートナー＆リダイレクトサービス** Rust、Axum、Redis、PostgreSQLで構築。安全なインタースティシャルページ、リアルタイム管理ダッシュボード、エンタープライズグレードの可観測性を備えています。

[English](../README.md) | [Русский](README.ru.md) | [中文](README.zh.md) | [हिंदी](README.hi.md) | [Español](README.es.md) | [Português](README.pt.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | **日本語** | [한국어](README.ko.md) | [Polski](README.pl.md) | [Nederlands](README.nl.md) | [Italiano](README.it.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) | [עברית](README.he.md) | [Bahasa Indonesia](README.id.md) | [Tiếng Việt](README.vi.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![Docker Image Size](https://ghcr-badge.egpl.dev/brilliant-almazov/redirector/size)](https://github.com/brilliant-almazov/redirector/pkgs/container/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

**キーワード**: URLショートナー, リンク短縮, リダイレクトサービス, Rust Webサービス, Axumフレームワーク, Redisキャッシュ, PostgreSQL, Prometheusメトリクス, hashids, 短縮リンク, インタースティシャルページ, 安全なリダイレクト, 高性能, マイクロサービス

インタースティシャルページとhashidベースの短縮リンクを備えた安全なURLリダイレクトサービス。内部ツール、エンタープライズリンク管理、ブランド短縮URLサービスに最適です。

### パフォーマンス

| シナリオ | RPS | 平均レイテンシ | P99レイテンシ |
|----------|-----|----------------|---------------|
| 100% キャッシュヒット | **7,800+** | ~14ms | ~50ms |
| キャッシュミス (10K URLs) | **2,300+** | ~44ms | ~81ms |

**テスト条件**: wrk -t4 -c100 -d30s, PostgreSQL 15, Dragonfly (Redis), macOS M1 (Docker)

> ⚠️ 結果はmacOS上のDockerでのVM オーバーヘッドを含んでいます。ネイティブLinuxデプロイメントでは**3-5倍高速**になると予想されます。

## 問題

長いURLを共有するのは不便です。URL短縮サービスは存在しますが、多くの場合すぐにリダイレクトされ、セキュリティリスクになる可能性があります。ユーザーはリダイレクトされる前に行き先を確認できるべきです。

**redirector** は安全なリダイレクトを提供します：
- リダイレクト前にターゲットURLを表示するインタースティシャルページ
- ユーザー認識のためのカウントダウンタイマー
- 美しくブランド化されたページ

## 機能

- 🔗 **Hashid URL** - 短く、ユニークで、連続しないID（例: `/r/abc123`）
- ⏱️ **インタースティシャルページ** - リダイレクト前にターゲットURLを表示するカウントダウンタイマー
- ⚡ **Redisキャッシュ** - 設定可能なTTLによる高速ルックアップ
- 🛡️ **サーキットブレーカー** - カスケード障害に対するデータベース保護
- 🚦 **レート制限** - グローバルおよびデータベースレベルのレート制限
- 📊 **Prometheusメトリクス** - Basic Auth保護付きの完全な可観測性
- 🎨 **美しいページ** - 4つのテーマを備えたクリーンな404とインデックスページ
- 🔑 **複数のソルト** - マイグレーション用のhashidソルトローテーションサポート
- 📱 **管理ダッシュボード** - SSEによるリアルタイムメトリクス監視

## スクリーンショット

| ライト | ダーク | グレー | ウォーム |
|--------|--------|--------|----------|
| ![ダッシュボード ライト](screenshots/dashboard-light.png) | ![ダッシュボード ダーク](screenshots/dashboard-dark.png) | ![ダッシュボード グレー](screenshots/dashboard-gray.png) | ![ダッシュボード ウォーム](screenshots/dashboard-warm.png) |
| ![ログイン ライト](screenshots/login-light.png) | ![ログイン ダーク](screenshots/login-dark.png) | ![ログイン グレー](screenshots/login-gray.png) | ![ログイン ウォーム](screenshots/login-warm.png) |
| ![Index Light](screenshots/index-light.png) | ![Index Dark](screenshots/index-dark.png) | ![Index Gray](screenshots/index-gray.png) | ![Index Warm](screenshots/index-warm.png) |
| ![Interstitial Light](screenshots/interstitial-light.png) | ![Interstitial Dark](screenshots/interstitial-dark.png) | ![Interstitial Gray](screenshots/interstitial-gray.png) | ![Interstitial Warm](screenshots/interstitial-warm.png) |
| ![404 Light](screenshots/404-light.png) | ![404 Dark](screenshots/404-dark.png) | ![404 Gray](screenshots/404-gray.png) | ![404 Warm](screenshots/404-warm.png) |

### 負荷テストモーダル

| ライト | ダーク | グレー | ウォーム |
|--------|--------|--------|----------|
| ![モーダル ライト](screenshots/dashboard-modal-light.png) | ![モーダル ダーク](screenshots/dashboard-modal-dark.png) | ![モーダル グレー](screenshots/dashboard-modal-gray.png) | ![モーダル ウォーム](screenshots/dashboard-modal-warm.png) |

## 技術スタック

- **言語**: Rust（Tokioによる非同期）
- **Webフレームワーク**: Axum
- **キャッシュ**: Redis互換（Redis、Dragonfly、Valkey、KeyDBなど）
- **データベース**: PostgreSQL（プラグ可能なストレージレイヤー）
- **メトリクス**: Prometheus + metrics-rs
- **パスワードハッシュ**: Argon2

> **注意**: ストレージとキャッシュレイヤーは抽象化されており、互換性のある任意のデータソースに置き換えることができます。現在活発に開発中です。

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

#### Base64設定

設定ファイルのマウントができない環境（サーバーレス、PaaSなど）向け：

```bash
# Encode
cat config.yaml | base64

# Run with base64 config
CONFIG_BASE64="c2VydmVyOgogIGhvc3Q6IC..." docker run ghcr.io/brilliant-almazov/redirector:latest
```

## 仕組み

1. ユーザーが `/r/{hashid}` にアクセス（例: `/r/abc123`）
2. サービスがhashidを数値IDにデコード
3. RedisキャッシュでURLを確認
4. キャッシュミスの場合、PostgreSQLをクエリ
5. 結果をRedisにキャッシュ
6. カウントダウン付きのインタースティシャルページを表示
7. カウントダウン後、ターゲットURLにリダイレクト

## エンドポイント

| エンドポイント | 認証 | 説明 |
|----------------|------|------|
| `GET /` | なし | インデックスページ |
| `GET /r/{hashid}` | なし | インタースティシャル付きリダイレクト |
| `GET /d/{hashid}` | なし | デモリダイレクト（合成負荷テスト） |
| `GET /health` | なし | ヘルスチェック |
| `GET /metrics` | Basic | Prometheusメトリクス |
| `GET /admin` | Session | 管理ダッシュボードログイン |
| `GET /admin/dashboard` | Session | 管理ダッシュボード |

## 管理ダッシュボード

サービスにはリアルタイムメトリクス監視用のオプション管理ダッシュボードが含まれています。

### セットアップ

1. **パスワードハッシュを生成:**

```bash
cargo run --bin hash_password
# パスワードを入力、または:
cargo run --bin hash_password -- "your-password"
```

2. **config.yamlに追加:**

```yaml
admin:
  enabled: true
  session_ttl_hours: 24
  users:
    - username: admin
      password_hash: "$argon2id$v=19$m=19456,t=2,p=1$..."  # ステップ1から
```

3. **ダッシュボードにアクセス:**

`http://localhost:8080/admin` を開き、認証情報でログインします。

### 機能

- リアルタイムRPSとレイテンシチャート
- システムメトリクス（CPU、メモリ、アップタイム）
- キャッシュヒット率監視
- 最近のリダイレクトリスト
- テスト用の負荷シミュレーション
- 3つのテーマ: ライト、ダーク、ウォーム

## ライセンス

MITライセンス - 詳細は[LICENSE](../LICENSE)を参照。

## 貢献

貢献を歓迎します！以下の手順で：

1. リポジトリをフォーク
2. フィーチャーブランチを作成
3. プルリクエストを送信

保護されたmasterブランチはPRレビューが必要です。
