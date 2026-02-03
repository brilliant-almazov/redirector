# redirector

[English](../README.md) | [Русский](README.ru.md) | [中文](README.zh.md) | [हिंदी](README.hi.md) | [Español](README.es.md) | **Português** | [Français](README.fr.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Polski](README.pl.md) | [Nederlands](README.nl.md) | [Italiano](README.it.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) | [Bahasa Indonesia](README.id.md) | [Tiếng Việt](README.vi.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

Serviço de redirecionamento de URL seguro com páginas intersticiais e links curtos baseados em hashid.

## Problema

Compartilhar URLs longas é inconveniente. Encurtadores de URL existem, mas frequentemente redirecionam imediatamente, o que pode ser um risco de segurança. Os usuários devem ver para onde estão indo antes de serem redirecionados.

**redirector** fornece redirecionamentos seguros com:
- Página intersticial mostrando URL de destino antes do redirecionamento
- Timer de contagem regressiva para conscientização do usuário
- Páginas bonitas e personalizadas

## Recursos

- 🔗 **URLs Hashid** - IDs curtos, únicos e não-sequenciais (ex. `/r/abc123`)
- ⏱️ **Página intersticial** - Timer mostra URL de destino antes do redirecionamento
- ⚡ **Cache Redis** - Buscas rápidas com TTL configurável
- 🛡️ **Circuit breaker** - Proteção do banco de dados contra falhas em cascata
- 🚦 **Limitação de taxa** - Limites globais e a nível de banco de dados
- 📊 **Métricas Prometheus** - Observabilidade completa com proteção Basic Auth
- 🎨 **Páginas bonitas** - Páginas 404 e índice limpas
- 🔑 **Múltiplos sais** - Suporte a rotação de sal hashid para migração

## Início Rápido

### Docker

```bash
docker run -p 8080:8080 \
  -v $(pwd)/config.yaml:/config.yaml \
  ghcr.io/brilliant-almazov/redirector:latest
```

## Como Funciona

1. Usuário visita `/r/{hashid}` (ex. `/r/abc123`)
2. Serviço decodifica hashid para ID numérico
3. Verifica cache Redis para URL
4. Em caso de miss no cache, consulta PostgreSQL
5. Armazena resultado em cache no Redis
6. Mostra página intersticial com contagem regressiva
7. Após contagem, redireciona para URL de destino

## Licença

Licença MIT - veja [LICENSE](../LICENSE) para detalhes.
