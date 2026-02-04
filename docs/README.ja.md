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

## 設定

`config.yaml` を作成してください：

```yaml
server:
  host: "0.0.0.0"
  port: 8080

hashids:
  salts:
    - ${HASHID_SALT}          # プライマリソルト
    - ${HASHID_SALT_OLD}      # オプション：移行用の旧ソルト
  min_length: 6

redis:
  url: ${REDIS_URL}
  cache_ttl_seconds: 86400    # 24時間

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
    table: "dictionary.urls"    # テーブル名
    id_column: "id"             # IDカラム
    url_column: "name"          # URLカラム

interstitial:
  delay_seconds: 5            # リダイレクト前のカウントダウン

metrics:
  basic_auth:
    username: prometheus
    password: ${METRICS_PASSWORD}

rate_limit:
  requests_per_second: 1000
  burst: 100
```

### 設定オプション

#### サーバー

| オプション | デフォルト | 説明 |
|------------|------------|------|
| `host` | `0.0.0.0` | バインドアドレス |
| `port` | `8080` | HTTPポート |

#### Hashids

| オプション | デフォルト | 説明 |
|------------|------------|------|
| `salts` | *必須* | hashidソルトのリスト（最初 = プライマリ） |
| `min_length` | `6` | hashidの最小長 |

#### Redis

| オプション | デフォルト | 説明 |
|------------|------------|------|
| `url` | *必須* | Redis接続URL |
| `cache_ttl_seconds` | `86400` | キャッシュTTL（秒） |

#### データベース

| オプション | デフォルト | 説明 |
|------------|------------|------|
| `url` | *必須* | PostgreSQL接続URL |
| `pool.max_connections` | `3` | コネクションプールサイズ |
| `pool.connect_timeout_seconds` | `3` | 接続タイムアウト |
| `rate_limit.max_requests_per_second` | `50` | DBレート制限 |
| `circuit_breaker.failure_threshold` | `3` | オープンまでの失敗回数 |
| `circuit_breaker.reset_timeout_seconds` | `60` | サーキットリセットタイムアウト |

#### レート制限（グローバル）

| オプション | デフォルト | 説明 |
|------------|------------|------|
| `requests_per_second` | `1000` | グローバルレート制限 |
| `burst` | `100` | バースト容量 |

### 環境変数

サービスの設定には**3つの方法**があります。優先度順（高い順）に記載：

| 優先度 | 方法 | 使用ケース |
|--------|------|------------|
| 1 | `REDIRECTOR__*` 環境変数 | 個別の値を上書き |
| 2 | 標準PaaS環境変数（`DATABASE_URL` 等） | PaaSプラットフォーム（Railway、Heroku、Render） |
| 3 | 設定ファイル（`config.yaml` または `CONFIG_BASE64`） | 基本設定 |

#### 特殊変数

| 変数 | デフォルト | 説明 |
|------|------------|------|
| `CONFIG_PATH` | `config.yaml` | YAML設定ファイルへのパス |
| `CONFIG_BASE64` | — | Base64エンコードされたYAML設定（`CONFIG_PATH`より優先） |

#### 標準PaaS環境変数

これらは自動的に認識され適用されます。ほとんどのPaaSプラットフォームが自動で設定します：

| 変数 | 設定パス | 例 |
|------|----------|-----|
| `DATABASE_URL` | `database.url` | `postgres://user:pass@host:5432/db` |
| `REDIS_URL` | `redis.url` | `redis://host:6379` |
| `PORT` | `server.port` | `3000` |

> **優先度ルール**：`DATABASE_URL` と `REDIRECTOR__DATABASE__URL` の両方が設定されている場合、`REDIRECTOR__` プレフィックス版が優先されます。

#### プレフィックス付き環境変数（`REDIRECTOR__*`）

任意の設定値は `REDIRECTOR__` プレフィックスと `__`（ダブルアンダースコア）をネストの区切りとして使用して上書きできます：

```
YAML設定パス                →  環境変数
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

#### デプロイプラットフォーム別の例

**Railway / Render / Fly.io**（マネージドDBを持つPaaS）：

```bash
# これらは通常プラットフォームによって自動設定されます：
DATABASE_URL=postgres://user:pass@host:5432/db
REDIS_URL=redis://host:6379
PORT=3000

# Base64で設定を指定：
CONFIG_BASE64=c2VydmVyOgogIGhvc3Q6IC...

# または個別の値を上書き：
REDIRECTOR__HASHIDS__SALTS__0=my-secret-salt
REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD=strong-password
```

**Docker / Docker Compose**：

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
      # または設定ファイルの上に個別の値を上書き：
      REDIRECTOR__RATE_LIMIT__REQUESTS_PER_SECOND: "2000"
      REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD: "${METRICS_PASSWORD}"
    volumes:
      - ./config.yaml:/app/config.yaml  # CONFIG_BASE64使用時はオプション
    depends_on:
      - postgres
      - redis
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

**プレーンDocker**：

```bash
docker run -p 8080:8080 \
  -e DATABASE_URL="postgres://user:pass@host:5432/db" \
  -e REDIS_URL="redis://host:6379" \
  -e CONFIG_BASE64="$(cat config.yaml | base64)" \
  ghcr.io/brilliant-almazov/redirector:latest
```

**最小構成（環境変数のみ、設定ファイルなし）**：

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

#### Base64設定

設定ファイルのマウントが実用的でない環境（PaaS、サーバーレス、CI/CD）では、設定全体をBase64エンコード文字列として渡します：

```bash
# エンコード
cat config.yaml | base64

# デコード（確認用）
echo "$CONFIG_BASE64" | base64 -d
```

`CONFIG_BASE64` は `CONFIG_PATH` より優先されます。環境変数のオーバーライド（`REDIRECTOR__*` とPaaS変数）はデコードされた設定の**上に**適用されます。

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
