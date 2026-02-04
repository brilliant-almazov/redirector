# redirector

> **Encurtador de URL e serviço de redirecionamento de alto desempenho** construído com Rust, Axum, Redis e PostgreSQL. Inclui páginas intersticiais seguras, painel de administração em tempo real e observabilidade de nível empresarial.

[English](../README.md) | [Русский](README.ru.md) | [中文](README.zh.md) | [हिंदी](README.hi.md) | [Español](README.es.md) | **Português** | [Français](README.fr.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Polski](README.pl.md) | [Nederlands](README.nl.md) | [Italiano](README.it.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) | [עברית](README.he.md) | [Bahasa Indonesia](README.id.md) | [Tiếng Việt](README.vi.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![Docker Image Size](https://ghcr-badge.egpl.dev/brilliant-almazov/redirector/size)](https://github.com/brilliant-almazov/redirector/pkgs/container/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

**Palavras-chave**: encurtador de URL, encurtador de links, serviço de redirecionamento, serviço web Rust, framework Axum, cache Redis, PostgreSQL, métricas Prometheus, hashids, links curtos, páginas intersticiais, redirecionamentos seguros, alto desempenho, microsserviço

Serviço de redirecionamento de URL seguro com páginas intersticiais e links curtos baseados em hashid. Perfeito para ferramentas internas, gerenciamento de links empresariais e serviços de URL curtas de marca.

### Desempenho

| Cenário | RPS | Latência Méd | Latência P99 |
|---------|-----|--------------|--------------|
| 100% Cache Hit | **7.800+** | ~14ms | ~50ms |
| Cache Miss (10K URLs) | **2.300+** | ~44ms | ~81ms |

**Condições de teste**: wrk -t4 -c100 -d30s, PostgreSQL 15, Dragonfly (Redis), macOS M1 (Docker)

> ⚠️ Os resultados são do Docker no macOS com sobrecarga de VM. Implantação nativa no Linux esperada ser **3-5x mais rápida**.

## Problema

Compartilhar URLs longas é inconveniente. Encurtadores de URL existem, mas frequentemente redirecionam imediatamente, o que pode ser um risco de segurança. Os usuários devem ver para onde estão indo antes de serem redirecionados.

**redirector** fornece redirecionamentos seguros com:
- Página intersticial mostrando URL de destino antes do redirecionamento
- Temporizador de contagem regressiva para conscientização do usuário
- Páginas bonitas e personalizadas

## Recursos

- 🔗 **URLs Hashid** - IDs curtos, únicos e não sequenciais (ex. `/r/abc123`)
- ⏱️ **Página intersticial** - Temporizador de contagem regressiva mostra URL de destino antes de redirecionar
- ⚡ **Cache Redis** - Buscas rápidas com TTL configurável
- 🛡️ **Circuit breaker** - Proteção de banco de dados contra falhas em cascata
- 🚦 **Limitação de taxa** - Limites de taxa globais e no nível do banco de dados
- 📊 **Métricas Prometheus** - Observabilidade completa com proteção Basic Auth
- 🎨 **Páginas bonitas** - Páginas 404 e índice limpas com 4 temas
- 🔑 **Múltiplos sais** - Suporte a rotação de sal hashid para migração
- 📱 **Painel de administração** - Monitoramento de métricas em tempo real com SSE

## Capturas de tela

| Claro | Escuro | Cinza | Quente |
|-------|--------|-------|--------|
| ![Dashboard Claro](screenshots/dashboard-light.png) | ![Dashboard Escuro](screenshots/dashboard-dark.png) | ![Dashboard Cinza](screenshots/dashboard-gray.png) | ![Dashboard Quente](screenshots/dashboard-warm.png) |
| ![Login Claro](screenshots/login-light.png) | ![Login Escuro](screenshots/login-dark.png) | ![Login Cinza](screenshots/login-gray.png) | ![Login Quente](screenshots/login-warm.png) |
| ![Index Light](screenshots/index-light.png) | ![Index Dark](screenshots/index-dark.png) | ![Index Gray](screenshots/index-gray.png) | ![Index Warm](screenshots/index-warm.png) |
| ![Interstitial Light](screenshots/interstitial-light.png) | ![Interstitial Dark](screenshots/interstitial-dark.png) | ![Interstitial Gray](screenshots/interstitial-gray.png) | ![Interstitial Warm](screenshots/interstitial-warm.png) |
| ![404 Light](screenshots/404-light.png) | ![404 Dark](screenshots/404-dark.png) | ![404 Gray](screenshots/404-gray.png) | ![404 Warm](screenshots/404-warm.png) |

### Modal de teste de carga

| Claro | Escuro | Cinza | Quente |
|-------|--------|-------|--------|
| ![Modal Claro](screenshots/dashboard-modal-light.png) | ![Modal Escuro](screenshots/dashboard-modal-dark.png) | ![Modal Cinza](screenshots/dashboard-modal-gray.png) | ![Modal Quente](screenshots/dashboard-modal-warm.png) |

## Stack tecnológica

- **Linguagem**: Rust (async com Tokio)
- **Framework web**: Axum
- **Cache**: Compatível com Redis (Redis, Dragonfly, Valkey, KeyDB, etc.)
- **Banco de dados**: PostgreSQL (camada de armazenamento intercambiável)
- **Métricas**: Prometheus + metrics-rs
- **Hash de senhas**: Argon2

> **Nota**: As camadas de armazenamento e cache são abstratas e podem ser substituídas por qualquer fonte de dados compatível. Atualmente em desenvolvimento ativo.

## Início rápido

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

### Variáveis de Ambiente

Existem **três maneiras** de configurar o serviço, listadas por prioridade (maior primeiro):

| Prioridade | Método | Caso de Uso |
|------------|--------|-------------|
| 1 | Variáveis de ambiente `REDIRECTOR__*` | Substituir valores individuais |
| 2 | Variáveis PaaS padrão (`DATABASE_URL`, etc.) | Plataformas PaaS (Railway, Heroku, Render) |
| 3 | Arquivo de configuração (`config.yaml` ou `CONFIG_BASE64`) | Configuração base |

#### Variáveis Especiais

| Variável | Padrão | Descrição |
|----------|--------|-----------|
| `CONFIG_PATH` | `config.yaml` | Caminho para o arquivo de configuração YAML |
| `CONFIG_BASE64` | — | Configuração YAML codificada em Base64 (tem prioridade sobre `CONFIG_PATH`) |

#### Variáveis de Ambiente PaaS Padrão

Estas são automaticamente reconhecidas e aplicadas. A maioria das plataformas PaaS as define automaticamente:

| Variável | Caminho de Configuração | Exemplo |
|----------|------------------------|---------|
| `DATABASE_URL` | `database.url` | `postgres://user:pass@host:5432/db` |
| `REDIS_URL` | `redis.url` | `redis://host:6379` |
| `PORT` | `server.port` | `3000` |
| `HASHIDS_SALTS` | `hashids.salts` | `new-salt,old-salt` (separado por vírgulas) |

> **Regra de prioridade**: Se ambos `DATABASE_URL` e `REDIRECTOR__DATABASE__URL` estiverem definidos, a versão com prefixo `REDIRECTOR__` vence. Da mesma forma, `REDIRECTOR__HASHIDS__SALTS__0` tem prioridade sobre `HASHIDS_SALTS`.

#### Variáveis de Ambiente com Prefixo (`REDIRECTOR__*`)

Qualquer valor de configuração pode ser substituído usando o prefixo `REDIRECTOR__` com `__` (sublinhado duplo) como separador de aninhamento. Abaixo está a **referência completa** de todas as variáveis substituíveis:

##### Server

| Variável de Ambiente | Caminho de Configuração | Padrão | Descrição |
|---------------------|------------------------|--------|-----------|
| `REDIRECTOR__SERVER__HOST` | `server.host` | `0.0.0.0` | Endereço de ligação |
| `REDIRECTOR__SERVER__PORT` | `server.port` | `8080` | Porta HTTP |

##### Hashids

| Variável de Ambiente | Caminho de Configuração | Padrão | Descrição |
|---------------------|------------------------|--------|-----------|
| `REDIRECTOR__HASHIDS__SALTS__0` | `hashids.salts[0]` | *obrigatório* | Salt primário do hashid |
| `REDIRECTOR__HASHIDS__SALTS__1` | `hashids.salts[1]` | — | Salt antigo (para migração) |
| `REDIRECTOR__HASHIDS__MIN_LENGTH` | `hashids.min_length` | `6` | Comprimento mínimo do hashid |

> **Arrays**: Os itens da lista são indexados com `__0`, `__1`, `__2`, etc. Para rotação de salt de hashid, defina `__0` para o novo salt e `__1` para o antigo.

##### Redis / Cache

| Variável de Ambiente | Caminho de Configuração | Padrão | Descrição |
|---------------------|------------------------|--------|-----------|
| `REDIRECTOR__REDIS__URL` | `redis.url` | *obrigatório* | URL de conexão Redis |
| `REDIRECTOR__REDIS__CACHE_TTL_SECONDS` | `redis.cache_ttl_seconds` | `86400` | TTL do cache (segundos). `86400` = 24h |

##### Banco de Dados

| Variável de Ambiente | Caminho de Configuração | Padrão | Descrição |
|---------------------|------------------------|--------|-----------|
| `REDIRECTOR__DATABASE__URL` | `database.url` | *obrigatório* | URL de conexão PostgreSQL |
| `REDIRECTOR__DATABASE__POOL__MAX_CONNECTIONS` | `database.pool.max_connections` | `3` | Tamanho do pool de conexões |
| `REDIRECTOR__DATABASE__POOL__CONNECT_TIMEOUT_SECONDS` | `database.pool.connect_timeout_seconds` | `3` | Timeout de conexão (segundos) |
| `REDIRECTOR__DATABASE__RATE_LIMIT__MAX_REQUESTS_PER_SECOND` | `database.rate_limit.max_requests_per_second` | `50` | Máx. consultas ao banco por segundo |
| `REDIRECTOR__DATABASE__CIRCUIT_BREAKER__FAILURE_THRESHOLD` | `database.circuit_breaker.failure_threshold` | `3` | Falhas consecutivas antes do circuito abrir |
| `REDIRECTOR__DATABASE__CIRCUIT_BREAKER__RESET_TIMEOUT_SECONDS` | `database.circuit_breaker.reset_timeout_seconds` | `60` | Segundos antes da tentativa half-open |
| `REDIRECTOR__DATABASE__QUERY__TABLE` | `database.query.table` | `dictionary.urls` | Nome da tabela para buscas de URL |
| `REDIRECTOR__DATABASE__QUERY__ID_COLUMN` | `database.query.id_column` | `id` | Nome da coluna para ID numérico |
| `REDIRECTOR__DATABASE__QUERY__URL_COLUMN` | `database.query.url_column` | `name` | Nome da coluna para URL de destino |

##### Página Intersticial

| Variável de Ambiente | Caminho de Configuração | Padrão | Descrição |
|---------------------|------------------------|--------|-----------|
| `REDIRECTOR__INTERSTITIAL__DELAY_SECONDS` | `interstitial.delay_seconds` | `5` | Contagem regressiva antes do redirecionamento |

##### Métricas

| Variável de Ambiente | Caminho de Configuração | Padrão | Descrição |
|---------------------|------------------------|--------|-----------|
| `REDIRECTOR__METRICS__BASIC_AUTH__USERNAME` | `metrics.basic_auth.username` | *obrigatório* | Nome de usuário para o endpoint `/metrics` |
| `REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD` | `metrics.basic_auth.password` | *obrigatório* | Senha para o endpoint `/metrics` |

##### Limitação de Taxa (Global)

| Variável de Ambiente | Caminho de Configuração | Padrão | Descrição |
|---------------------|------------------------|--------|-----------|
| `REDIRECTOR__RATE_LIMIT__REQUESTS_PER_SECOND` | `rate_limit.requests_per_second` | `1000` | Máx. requisições por segundo |
| `REDIRECTOR__RATE_LIMIT__BURST` | `rate_limit.burst` | `100` | Capacidade de burst acima do limite de RPS |

##### Painel de Administração

| Variável de Ambiente | Caminho de Configuração | Padrão | Descrição |
|---------------------|------------------------|--------|-----------|
| `REDIRECTOR__ADMIN__ENABLED` | `admin.enabled` | `false` | Habilitar painel de administração |
| `REDIRECTOR__ADMIN__SESSION_SECRET` | `admin.session_secret` | `change-me-...` | Segredo de assinatura de sessão (mín. 32 caracteres) |
| `REDIRECTOR__ADMIN__SESSION_TTL_HOURS` | `admin.session_ttl_hours` | `24` | Tempo de vida da sessão em horas |

> **Nota**: Usuários administradores (`admin.users`) com `username` e `password_hash` não podem ser definidos via variáveis de ambiente devido à sua estrutura complexa. Defina-os no arquivo de configuração ou `CONFIG_BASE64`.

#### Exemplos por Plataforma de Implantação

**Railway / Render / Fly.io** (PaaS com bancos de dados gerenciados):

```bash
# Estas são geralmente definidas automaticamente pela plataforma:
DATABASE_URL=postgres://user:pass@host:5432/db
REDIS_URL=redis://host:6379
PORT=3000

# Defina sua configuração via base64:
CONFIG_BASE64=c2VydmVyOgogIGhvc3Q6IC...

# Ou substitua valores individuais:
REDIRECTOR__HASHIDS__SALTS__0=my-secret-salt
REDIRECTOR__METRICS__BASIC_AUTH__USERNAME=prometheus
REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD=strong-password
REDIRECTOR__ADMIN__ENABLED=true
REDIRECTOR__ADMIN__SESSION_SECRET=random-32-byte-secret-for-sessions
```

**Docker Compose (exemplo completo com todas as substituições)**:

```yaml
services:
  redirector:
    image: ghcr.io/brilliant-almazov/redirector:latest
    ports:
      - "8080:8080"
    environment:
      # --- URLs de conexão (estilo PaaS) ---
      DATABASE_URL: "postgres://redirector:${DB_PASSWORD}@postgres:5432/redirector"
      REDIS_URL: "redis://redis:6379"

      # --- Arquivo de configuração ---
      CONFIG_BASE64: "${CONFIG_BASE64}"

      # --- Server ---
      REDIRECTOR__SERVER__HOST: "0.0.0.0"
      REDIRECTOR__SERVER__PORT: "8080"

      # --- Salts do hashid ---
      REDIRECTOR__HASHIDS__SALTS__0: "${HASHID_SALT}"        # salt primário
      REDIRECTOR__HASHIDS__SALTS__1: "${HASHID_SALT_OLD}"    # salt antigo para migração
      REDIRECTOR__HASHIDS__MIN_LENGTH: "6"

      # --- Cache Redis ---
      REDIRECTOR__REDIS__CACHE_TTL_SECONDS: "43200"          # 12 horas

      # --- Pool do banco de dados e resiliência ---
      REDIRECTOR__DATABASE__POOL__MAX_CONNECTIONS: "5"
      REDIRECTOR__DATABASE__POOL__CONNECT_TIMEOUT_SECONDS: "5"
      REDIRECTOR__DATABASE__RATE_LIMIT__MAX_REQUESTS_PER_SECOND: "100"
      REDIRECTOR__DATABASE__CIRCUIT_BREAKER__FAILURE_THRESHOLD: "5"
      REDIRECTOR__DATABASE__CIRCUIT_BREAKER__RESET_TIMEOUT_SECONDS: "30"

      # --- Mapeamento personalizado de tabela ---
      REDIRECTOR__DATABASE__QUERY__TABLE: "public.short_urls"
      REDIRECTOR__DATABASE__QUERY__ID_COLUMN: "id"
      REDIRECTOR__DATABASE__QUERY__URL_COLUMN: "target_url"

      # --- Página intersticial ---
      REDIRECTOR__INTERSTITIAL__DELAY_SECONDS: "3"

      # --- Autenticação de métricas ---
      REDIRECTOR__METRICS__BASIC_AUTH__USERNAME: "prometheus"
      REDIRECTOR__METRICS__BASIC_AUTH__PASSWORD: "${METRICS_PASSWORD}"

      # --- Limitação de taxa global ---
      REDIRECTOR__RATE_LIMIT__REQUESTS_PER_SECOND: "2000"
      REDIRECTOR__RATE_LIMIT__BURST: "200"

      # --- Painel de administração ---
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

**Kubernetes**:

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

**Docker simples (comando único)**:

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

**Configuração mínima (apenas variáveis de ambiente, sem arquivo de configuração)**:

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

#### Rotação de Salt via Variáveis de Ambiente

Ao rotacionar salts de hashid, o serviço tenta os salts em ordem -- a primeira correspondência vence. Defina o novo salt primeiro para que novos links o usem, e mantenha o salt antigo para compatibilidade retroativa:

**Opção 1: Variável única com separador de vírgula** (recomendado):

```bash
# Antes da rotação
HASHIDS_SALTS=original-salt

# Após a rotação — novo salt primeiro, salt antigo para links existentes
HASHIDS_SALTS=new-salt,original-salt
```

**Opção 2: Variáveis indexadas**:

```bash
# Antes da rotação
REDIRECTOR__HASHIDS__SALTS__0=original-salt

# Após a rotação
REDIRECTOR__HASHIDS__SALTS__0=new-salt
REDIRECTOR__HASHIDS__SALTS__1=original-salt
```

> **Nota**: Se `REDIRECTOR__HASHIDS__SALTS__0` estiver definido, `HASHIDS_SALTS` é ignorado.

#### Configuração Base64

Para ambientes onde a montagem de arquivos de configuração não é prática (PaaS, serverless, CI/CD), passe toda a configuração como uma string codificada em base64:

```bash
# Encode
cat config.yaml | base64

# Decodificar (para verificar)
echo "$CONFIG_BASE64" | base64 -d
```

`CONFIG_BASE64` tem prioridade sobre `CONFIG_PATH`. Substituições de variáveis de ambiente (`REDIRECTOR__*` e variáveis PaaS) são aplicadas **por cima** da configuração decodificada.

## Como funciona

1. O usuário visita `/r/{hashid}` (ex. `/r/abc123`)
2. O serviço decodifica o hashid para ID numérico
3. Verifica o cache Redis para a URL
4. Em caso de miss no cache, consulta PostgreSQL
5. Armazena o resultado no cache Redis
6. Exibe página intersticial com contagem regressiva
7. Após a contagem regressiva, redireciona para a URL de destino

## Endpoints

| Endpoint | Auth | Descrição |
|----------|------|-----------|
| `GET /` | Não | Página inicial |
| `GET /r/{hashid}` | Não | Redirecionamento com página intersticial |
| `GET /d/{hashid}` | Não | Redirecionamento demo (teste de carga sintético) |
| `GET /health` | Não | Verificação de saúde |
| `GET /metrics` | Basic | Métricas Prometheus |
| `GET /admin` | Session | Login do painel de administração |
| `GET /admin/dashboard` | Session | Painel de administração |

## Painel de administração

O serviço inclui um painel de administração opcional para monitorar métricas em tempo real.

### Configuração

1. **Gerar hash de senha:**

```bash
cargo run --bin hash_password
# Digite a senha, ou:
cargo run --bin hash_password -- "your-password"
```

2. **Adicionar ao config.yaml:**

```yaml
admin:
  enabled: true
  session_ttl_hours: 24
  users:
    - username: admin
      password_hash: "$argon2id$v=19$m=19456,t=2,p=1$..."  # do passo 1
```

3. **Acessar o painel:**

Abra `http://localhost:8080/admin` e faça login com suas credenciais.

### Recursos

- Gráficos de RPS e latência em tempo real
- Métricas do sistema (CPU, memória, uptime)
- Monitoramento de taxa de acerto de cache
- Lista de redirecionamentos recentes
- Simulação de carga para testes
- Três temas: Claro, Escuro, Quente

## Licença

Licença MIT - veja [LICENSE](../LICENSE) para detalhes.

## Contribuir

Contribuições são bem-vindas! Por favor:

1. Faça fork do repositório
2. Crie um branch de feature
3. Envie um Pull Request

Branch master protegido requer revisão de PR.
