# Event Analytics Pipeline

> Optional RabbitMQ-based event publishing with PostgreSQL analytics storage. Collect, enrich, and analyze redirect events in real-time.

## Overview

The event analytics pipeline captures every redirect event and stores enriched data in a separate PostgreSQL analytics database. The pipeline is designed for high performance with minimal impact on redirect latency.

## Architecture Diagrams

### High-Level Flow

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              redirector                                      │
│  ┌─────────────┐    ┌──────────────┐    ┌─────────────────┐                 │
│  │   Handler   │───▶│  Dispatcher  │───▶│    Publisher    │                 │
│  │ (redirect)  │    │ (fire-forget)│    │  (batch+flush)  │                 │
│  └─────────────┘    └──────────────┘    └────────┬────────┘                 │
└───────────────────────────────────────────────────│─────────────────────────┘
                                                    │
                                                    ▼
                                        ┌───────────────────┐
                                        │     RabbitMQ      │
                                        │ (events.analytics)│
                                        └─────────┬─────────┘
                                                  │
                                                  ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                          event_consumer                                      │
│  ┌──────────────┐    ┌──────────────┐    ┌─────────────────┐                │
│  │   Consumer   │───▶│   Enricher   │───▶│ PostgreSQL      │                │
│  │  (prefetch)  │    │ (UA, Geo, …) │    │ (partitioned)   │                │
│  └──────────────┘    └──────────────┘    └─────────────────┘                │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Publisher Detail

```
                          redirector process
┌────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│   HTTP Request                                                              │
│        │                                                                    │
│        ▼                                                                    │
│   ┌─────────────────┐                                                       │
│   │ redirect_handler│                                                       │
│   │   (axum)        │                                                       │
│   └────────┬────────┘                                                       │
│            │                                                                │
│            │ try_send(RedirectEvent)                                        │
│            │ non-blocking, drops if full                                    │
│            ▼                                                                │
│   ┌─────────────────┐         ┌─────────────────────────────────────┐      │
│   │  EventDispatcher│────────▶│  tokio::mpsc channel (10K buffer)   │      │
│   │    (facade)     │         └──────────────────┬──────────────────┘      │
│   └─────────────────┘                            │                          │
│                                                  │                          │
│                                                  ▼                          │
│                              ┌───────────────────────────────────┐          │
│                              │      Publisher Background Task     │          │
│                              │  ┌─────────────────────────────┐  │          │
│                              │  │  Buffer: Vec<RedirectEvent> │  │          │
│                              │  └─────────────────────────────┘  │          │
│                              │                                   │          │
│                              │  Flush when:                      │          │
│                              │  • buffer.len() >= 100 (batch)    │          │
│                              │  • 1 second elapsed (interval)    │          │
│                              │  • channel closed (shutdown)      │          │
│                              └───────────────┬───────────────────┘          │
│                                              │                              │
└──────────────────────────────────────────────│──────────────────────────────┘
                                               │
                                               │ JSON payload
                                               ▼
                                    ┌─────────────────────┐
                                    │      RabbitMQ       │
                                    │  queue: durable     │
                                    │  delivery: persist  │
                                    └─────────────────────┘
```

### Consumer & Enrichment Detail

```
                          event_consumer process
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                              │
│   ┌──────────────────┐                                                       │
│   │  RabbitMQ Queue  │                                                       │
│   │  (prefetch: 10)  │                                                       │
│   └────────┬─────────┘                                                       │
│            │                                                                 │
│            ▼                                                                 │
│   ┌──────────────────┐     ┌────────────────────────────────────────────┐   │
│   │  Deserialize     │────▶│  EventBatch { event_type: "redirect", ... }│   │
│   │  JSON → Struct   │     └────────────────────────────────────────────┘   │
│   └──────────────────┘                                                       │
│            │                                                                 │
│            ▼                                                                 │
│   ┌──────────────────────────────────────────────────────────────────────┐  │
│   │                        Enrichment Layer                               │  │
│   │  ┌────────────────┐  ┌────────────────┐  ┌────────────────────────┐  │  │
│   │  │ UserAgentResolver │  │ RefererResolver │  │ GeoLocationResolver   │  │  │
│   │  │                │  │                │  │                        │  │  │
│   │  │ woothee parse: │  │ MD5 hash for   │  │ MaxMind mmdb lookup:   │  │  │
│   │  │ • browser      │  │ deduplication  │  │ • country_code         │  │  │
│   │  │ • version      │  │                │  │ • city                 │  │  │
│   │  │ • os           │  │ Domain extract │  │                        │  │  │
│   │  │ • device_type  │  │ & normalize    │  │ Hot-reload every 1h    │  │  │
│   │  └────────────────┘  └────────────────┘  └────────────────────────┘  │  │
│   └──────────────────────────────────────────────────────────────────────┘  │
│            │                                                                 │
│            ▼                                                                 │
│   ┌──────────────────────────────────────────────────────────────────────┐  │
│   │                     PostgreSQL Analytics DB                           │  │
│   │  ┌─────────────────────────────────────────────────────────────────┐ │  │
│   │  │ redirect_events (PARTITION BY RANGE event_timestamp)            │ │  │
│   │  │   ├── redirect_events_2026_01                                   │ │  │
│   │  │   ├── redirect_events_2026_02                                   │ │  │
│   │  │   └── ... (auto-created)                                        │ │  │
│   │  └─────────────────────────────────────────────────────────────────┘ │  │
│   │  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌─────────────┐  │  │
│   │  │  referers    │ │ user_agents  │ │referer_domains│ │geo_locations│  │  │
│   │  │  (MD5 hash)  │ │ (MD5 + parse)│ │ (normalized) │ │ (country+   │  │  │
│   │  │              │ │              │ │              │ │  city)      │  │  │
│   │  └──────────────┘ └──────────────┘ └──────────────┘ └─────────────┘  │  │
│   │  ┌──────────────────────────────────────────────────────────────────┐│  │
│   │  │ urls (PARTITION BY HASH id, 10 buckets) — local URL copy         ││  │
│   │  └──────────────────────────────────────────────────────────────────┘│  │
│   └──────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
└──────────────────────────────────────────────────────────────────────────────┘
```

### Entity Resolution Pattern

```
resolve_or_create(resolver, raw_input)
         │
         ▼
   ┌───────────────┐
   │ 1. prepare()  │  Enrich raw input → Key
   │               │  (parse UA, normalize domain, hash, etc.)
   └───────┬───────┘
           │
           ▼
   ┌───────────────┐
   │ 2. find()     │  SELECT id FROM table WHERE key = $1
   └───────┬───────┘
           │
     ┌─────┴─────┐
     │ Found?    │
     └─────┬─────┘
       Yes │ No
           │
           ▼
   ┌───────────────┐
   │ 3. insert()   │  INSERT ... ON CONFLICT DO NOTHING RETURNING id
   └───────┬───────┘
           │
     ┌─────┴─────┐
     │ Returned? │ (race condition: another tx inserted)
     └─────┬─────┘
       Yes │ No
           │
           ▼
   ┌───────────────┐
   │ 4. find()     │  Fallback SELECT after race
   └───────────────┘
```

### Database Schema (ER Diagram)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          PostgreSQL Analytics Database                       │
└─────────────────────────────────────────────────────────────────────────────┘

┌──────────────────────┐
│   redirect_events    │ ◄─── PARTITIONED BY RANGE (event_timestamp)
├──────────────────────┤      Auto-creates: redirect_events_YYYY_MM
│ id             BIGSERIAL
│ url_id         BIGINT ───────────────────────────┐
│ event_timestamp TIMESTAMP ◄── partition key      │
│ latency_micros BIGINT                            │
│ source         ENUM('cache','database')          │
│ referer_id     BIGINT ─────┐                     │
│ user_agent_id  BIGINT ─────┼─────┐               │
│ referer_domain_id BIGINT ──┼─────┼─────┐         │
│ geo_location_id BIGINT ────┼─────┼─────┼─────┐   │
│ ip             INET        │     │     │     │   │
│ batch_id       BIGINT      │     │     │     │   │
│ created_at     TIMESTAMP   │     │     │     │   │
└──────────────────────┘     │     │     │     │   │
        PK: (id, event_timestamp)  │     │     │   │
                                   │     │     │   │
┌──────────────────────┐           │     │     │   │      ┌──────────────────┐
│     referers         │◄──────────┘     │     │   │      │      urls        │
├──────────────────────┤                 │     │   └─────▶├──────────────────┤
│ id      BIGSERIAL PK │                 │     │          │ id    BIGINT PK  │
│ hash    CHAR(32) UQ  │ ◄── MD5        │     │          │ url   TEXT       │
│ value   TEXT         │                 │     │          │ first_seen TS    │
└──────────────────────┘                 │     │          └──────────────────┘
                                         │     │           PARTITION BY HASH(id)
┌──────────────────────┐                 │     │           10 buckets
│    user_agents       │◄────────────────┘     │
├──────────────────────┤                       │
│ id             BIGSERIAL PK                  │
│ hash           CHAR(32) UQ  ◄── MD5         │
│ value          TEXT                          │
│ browser        TEXT         ◄── woothee     │
│ browser_version TEXT                         │
│ os             TEXT                          │
│ device_type    TEXT                          │
└──────────────────────┘                       │
                                               │
┌──────────────────────┐                       │
│  referer_domains     │◄──────────────────────┘
├──────────────────────┤
│ id      BIGSERIAL PK │
│ domain  TEXT UQ      │ ◄── normalized (lowercase, no www.)
└──────────────────────┘

┌──────────────────────┐
│   geo_locations      │◄──────────────────────────────────────────────────────
├──────────────────────┤
│ id           BIGSERIAL PK
│ country_code CHAR(2)     │ ◄── MaxMind
│ city         TEXT        │
└──────────────────────┘
  UQ: (country_code, city)

Note: ID = 1 is reserved as "unknown" sentinel in all reference tables.
```

## Features

- **Fire-and-forget publishing** — redirect latency unaffected by queue availability
- **Batching** — events grouped by size (100) or time (1 second)
- **Graceful degradation** — redirects work even if RabbitMQ is down
- **User-Agent parsing** — browser, version, OS, device type extraction
- **GeoIP enrichment** — country and city from IP (MaxMind mmdb)
- **Reference deduplication** — MD5-based dedup for referers and user agents
- **Monthly partitioning** — automatic partition creation for `redirect_events`
- **Snowflake IDs** — unique batch identifiers with custom epoch (2025-01-01)

## Quick Start

### 1. Enable Event Publishing

Add to `config.yaml`:

```yaml
events:
  enabled: true
  rabbitmq:
    url: amqp://guest:guest@localhost:5672/%2f
    queue: redirector.events.analytics
  publisher:
    channel_buffer_size: 10000
    batch_size: 100
    flush_interval_ms: 1000
```

Or via environment:

```bash
REDIRECTOR__EVENTS__ENABLED=true
RABBITMQ_URL=amqp://guest:guest@localhost:5672/%2f
```

### 2. Run Event Consumer

The consumer is a separate binary:

```bash
# Using cargo
RABBITMQ_URL=amqp://guest:guest@localhost:5672/%2f \
DATABASE_URL=postgres://localhost/redirector_analytics \
cargo run --bin event_consumer

# Using Docker
docker run -e RABBITMQ_URL=... -e DATABASE_URL=... \
  ghcr.io/brilliant-almazov/redirector:latest \
  /app/event_consumer
```

### 3. (Optional) Enable GeoIP

Download MaxMind GeoLite2-City database and provide path:

```bash
GEOIP_DB_PATH=/path/to/GeoLite2-City.mmdb
```

The consumer auto-reloads the database file every hour if it changes.

## Docker Compose

Complete setup with RabbitMQ and analytics database:

```yaml
services:
  redirector:
    image: ghcr.io/brilliant-almazov/redirector:latest
    environment:
      REDIRECTOR__EVENTS__ENABLED: "true"
      RABBITMQ_URL: "amqp://guest:guest@rabbitmq:5672/%2f"
    depends_on:
      - rabbitmq

  event_consumer:
    image: ghcr.io/brilliant-almazov/redirector:latest
    command: ["/app/event_consumer"]
    environment:
      RABBITMQ_URL: "amqp://guest:guest@rabbitmq:5672/%2f"
      DATABASE_URL: "postgres://postgres:postgres@analytics-db:5432/analytics"
      GEOIP_DB_PATH: "/data/GeoLite2-City.mmdb"
    volumes:
      - ./GeoLite2-City.mmdb:/data/GeoLite2-City.mmdb:ro
    depends_on:
      - rabbitmq
      - analytics-db

  rabbitmq:
    image: rabbitmq:3-management-alpine
    ports:
      - "5672:5672"
      - "15672:15672"

  analytics-db:
    image: postgres:16-alpine
    environment:
      POSTGRES_DB: analytics
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: postgres
    volumes:
      - analytics-data:/var/lib/postgresql/data

volumes:
  analytics-data:
```

## Configuration Reference

### Publisher (redirector)

| Option | Default | Description |
|--------|---------|-------------|
| `events.enabled` | `false` | Enable event publishing |
| `events.rabbitmq.url` | `amqp://localhost:5672/%2f` | RabbitMQ connection URL |
| `events.rabbitmq.queue` | `events.analytics` | Queue name |
| `events.publisher.channel_buffer_size` | `10000` | Internal event buffer size |
| `events.publisher.batch_size` | `100` | Events per batch before flush |
| `events.publisher.flush_interval_ms` | `1000` | Max delay before flush (ms) |

### Consumer (event_consumer)

| Environment Variable | Default | Description |
|---------------------|---------|-------------|
| `RABBITMQ_URL` | `amqp://localhost:5672/%2f` | RabbitMQ connection URL |
| `QUEUE` | `events.analytics` | Queue name to consume from |
| `DATABASE_URL` | `postgres://localhost/redirector_analytics` | Analytics database URL |
| `PREFETCH_COUNT` | `10` | RabbitMQ prefetch count |
| `GEOIP_DB_PATH` | — | Path to MaxMind .mmdb file |

## Database Schema

### Main Table: `redirect_events`

Partitioned by month on `event_timestamp`:

```sql
CREATE TABLE redirect_events (
    id BIGSERIAL,
    url_id BIGINT NOT NULL,
    event_timestamp TIMESTAMP NOT NULL,
    latency_micros BIGINT NOT NULL,
    source data_source NOT NULL,  -- ENUM: 'cache', 'database'
    referer_id BIGINT NOT NULL,
    user_agent_id BIGINT NOT NULL,
    referer_domain_id BIGINT NOT NULL,
    geo_location_id BIGINT NOT NULL,
    ip INET,
    batch_id BIGINT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    PRIMARY KEY (id, event_timestamp)
) PARTITION BY RANGE (event_timestamp);
```

Partitions created automatically: `redirect_events_2026_01`, `redirect_events_2026_02`, etc.

### Reference Tables

| Table | Key | Purpose |
|-------|-----|---------|
| `referers` | MD5 hash | Full referer URL deduplication |
| `user_agents` | MD5 hash | UA string + parsed fields (browser, os, device) |
| `referer_domains` | Normalized domain | Domain-level aggregation |
| `geo_locations` | country_code + city | GeoIP lookup results |
| `urls` | URL ID | Local copy of target URLs (hash-partitioned) |

All reference tables use ID `1` as sentinel for "unknown/missing" values.

## Event Format

Events are batched and serialized as JSON:

```json
{
  "event_type": "redirect",
  "events": [
    {
      "url_id": 12345,
      "target_url": "https://example.com/page",
      "timestamp": "2026-02-05T12:30:00",
      "latency_micros": 1500,
      "source": "cache",
      "referer": "https://google.com/search?q=...",
      "user_agent": "Mozilla/5.0 ...",
      "ip": "203.0.113.42"
    }
  ],
  "batch_id": 7318624813056,
  "produced_at": "2026-02-05T12:30:01"
}
```

## Metrics

### Publisher (redirector)

| Metric | Type | Description |
|--------|------|-------------|
| `events_published` | Counter | Events successfully published |
| `events_dropped` | Counter | Events dropped (buffer full, no connection) |
| `events_serialize_errors` | Counter | JSON serialization failures |
| `rabbitmq_connected` | Gauge | 1 if connected, 0 otherwise |

### Consumer (event_consumer)

Logs structured JSON with batch processing info. Prometheus metrics planned for future release.

## Example Queries

### Redirects per URL (last 24h)

```sql
SELECT url_id, COUNT(*) as redirects
FROM redirect_events
WHERE event_timestamp > NOW() - INTERVAL '24 hours'
GROUP BY url_id
ORDER BY redirects DESC
LIMIT 10;
```

### Cache hit rate

```sql
SELECT
  source,
  COUNT(*) as count,
  ROUND(100.0 * COUNT(*) / SUM(COUNT(*)) OVER (), 2) as percent
FROM redirect_events
WHERE event_timestamp > NOW() - INTERVAL '1 hour'
GROUP BY source;
```

### Top referrer domains

```sql
SELECT rd.domain, COUNT(*) as visits
FROM redirect_events re
JOIN referer_domains rd ON re.referer_domain_id = rd.id
WHERE re.event_timestamp > NOW() - INTERVAL '7 days'
  AND rd.domain != '(unknown)'
GROUP BY rd.domain
ORDER BY visits DESC
LIMIT 20;
```

### Browser distribution

```sql
SELECT ua.browser, COUNT(*) as visits
FROM redirect_events re
JOIN user_agents ua ON re.user_agent_id = ua.id
WHERE re.event_timestamp > NOW() - INTERVAL '30 days'
  AND ua.browser IS NOT NULL
GROUP BY ua.browser
ORDER BY visits DESC;
```

### Geographic distribution

```sql
SELECT gl.country_code, gl.city, COUNT(*) as visits
FROM redirect_events re
JOIN geo_locations gl ON re.geo_location_id = gl.id
WHERE re.event_timestamp > NOW() - INTERVAL '7 days'
  AND gl.country_code != '--'
GROUP BY gl.country_code, gl.city
ORDER BY visits DESC
LIMIT 50;
```

## Architecture Notes

### Why Separate Consumer?

1. **Isolation** — analytics failures don't affect redirects
2. **Scaling** — run multiple consumers for higher throughput
3. **Deployment** — different resource requirements
4. **Maintenance** — update schema without restarting redirector

### Partition Strategy

- **Events**: Monthly RANGE partitions on `event_timestamp`
- **URLs**: HASH partitions (10 buckets) for parallel scans
- Partitions created automatically on first event in month
- Old partitions can be archived/dropped independently

### Snowflake IDs

Batch IDs use Snowflake format with custom epoch (2025-01-01):
- 41 bits: timestamp (ms) — ~69 years from epoch
- 10 bits: machine ID (hardcoded 0 for single publisher)
- 12 bits: sequence (4096 per ms)

### GeoIP Hot-Reload

The consumer checks the mmdb file every hour and reloads if modified. This allows updating the database without restarting the consumer.

## Troubleshooting

### Events not appearing in database

1. Check RabbitMQ connection: `rabbitmq_connected` metric should be `1`
2. Check queue in RabbitMQ management UI (port 15672)
3. Check consumer logs for errors
4. Verify `events.enabled: true` in config

### High `events_dropped` count

1. RabbitMQ is down — events buffered then dropped
2. Buffer full — increase `channel_buffer_size`
3. Network issues — check connectivity

### Consumer stuck

1. Check for poison messages (malformed JSON)
2. Verify database connectivity
3. Check for partition creation failures (permissions)

## Future Plans

- Kafka transport option
- ClickHouse storage option
- Consumer Prometheus metrics
- Configurable machine ID for multi-publisher setups
- Dead letter queue for poison messages
