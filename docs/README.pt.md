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
