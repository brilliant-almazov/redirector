# redirector

[English](../README.md) | [Русский](README.ru.md) | [中文](README.zh.md) | [हिंदी](README.hi.md) | [Español](README.es.md) | [Português](README.pt.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Polski](README.pl.md) | [Nederlands](README.nl.md) | **Italiano** | [Türkçe](README.tr.md) | [Українська](README.uk.md) | [Bahasa Indonesia](README.id.md) | [Tiếng Việt](README.vi.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md)

[![CI](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml/badge.svg)](https://github.com/brilliant-almazov/redirector/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/coverage.json)](https://github.com/brilliant-almazov/redirector)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![RPS](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/rps.json)](https://github.com/brilliant-almazov/redirector)
[![Latency](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/latency.json)](https://github.com/brilliant-almazov/redirector)
[![Cache Hit](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/brilliant-almazov/5f930cca5d181b300d81d45850ddaf67/raw/cache_hit_rate.json)](https://github.com/brilliant-almazov/redirector)

Servizio di reindirizzamento URL sicuro con pagine interstiziali e link brevi basati su hashid.

## Problema

Condividere URL lunghi è scomodo. Gli accorciatori di URL esistono ma spesso reindirizzano immediatamente, il che può essere un rischio per la sicurezza. Gli utenti dovrebbero vedere dove stanno andando prima di essere reindirizzati.

**redirector** fornisce reindirizzamenti sicuri con:
- Pagina interstiziale che mostra l'URL di destinazione prima del reindirizzamento
- Timer con conto alla rovescia per la consapevolezza dell'utente
- Pagine belle e brandizzate

## Funzionalità

- 🔗 **URL Hashid** - ID brevi, unici e non sequenziali (es. `/r/abc123`)
- ⏱️ **Pagina interstiziale** - Conto alla rovescia mostra URL di destinazione prima del reindirizzamento
- ⚡ **Cache Redis** - Ricerche veloci con TTL configurabile
- 🛡️ **Circuit breaker** - Protezione database contro errori a cascata
- 🚦 **Limitazione velocità** - Limiti globali e a livello database
- 📊 **Metriche Prometheus** - Osservabilità completa con protezione Basic Auth
- 🎨 **Pagine belle** - Pagine 404 e indice pulite
- 🔑 **Salt multipli** - Supporto rotazione salt hashid per migrazione

## Avvio Rapido

### Docker

```bash
docker run -p 8080:8080 \
  -v $(pwd)/config.yaml:/config.yaml \
  ghcr.io/brilliant-almazov/redirector:latest
```

## Come Funziona

1. L'utente visita `/r/{hashid}` (es. `/r/abc123`)
2. Il servizio decodifica hashid in ID numerico
3. Controlla cache Redis per URL
4. In caso di cache miss, interroga PostgreSQL
5. Memorizza risultato in Redis
6. Mostra pagina interstiziale con conto alla rovescia
7. Dopo il conto alla rovescia, reindirizza all'URL di destinazione

## Licenza

Licenza MIT - vedi [LICENSE](../LICENSE) per i dettagli.
