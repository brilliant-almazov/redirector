# Click Analytics Design

## Overview

Record redirect clicks for analytics: who clicked, where from, when. Low traffic expected — simple synchronous inserts into PostgreSQL. When scaling is needed, switch to RabbitMQ (already available in infrastructure).

## Database Schema

### Click events table (partitioned by month)

```sql
CREATE SCHEMA IF NOT EXISTS analytics;

CREATE TABLE analytics.clicks (
    id              BIGSERIAL,
    url_id          BIGINT NOT NULL,
    ua_id           INT REFERENCES analytics.user_agents(id),
    ip              INET NOT NULL,
    referrer        TEXT,
    referrer_domain TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (id, created_at)
) PARTITION BY RANGE (created_at);

CREATE INDEX idx_clicks_url_id ON analytics.clicks (url_id);
CREATE INDEX idx_clicks_created_at ON analytics.clicks (created_at);
CREATE INDEX idx_clicks_ip ON analytics.clicks USING gist (ip inet_ops);
```

Monthly partitions:

```sql
CREATE TABLE analytics.clicks_2026_02 PARTITION OF analytics.clicks
    FOR VALUES FROM ('2026-02-01') TO ('2026-03-01');
```

Old partitions can be detached/dropped without vacuum overhead.

### User-Agent dictionary

```sql
CREATE TABLE analytics.user_agents (
    id      SERIAL PRIMARY KEY,
    hash    INT NOT NULL UNIQUE,
    raw     TEXT NOT NULL
);

CREATE INDEX idx_ua_hash ON analytics.user_agents (hash);
```

- Hash: CRC32 of raw User-Agent string for fast lookup
- Avoids storing duplicate 200+ byte UA strings per click
- In-app: `HashMap<u32, i32>` cache (hash → id), write-through to DB

## Data Flow

### Phase 1: Direct insert (current plan)

```
Client → Redirect Handler → INSERT into analytics.clicks → Response
```

Simple synchronous insert in the redirect handler. At low traffic (+2-3ms per redirect) this is acceptable.

### Phase 2: RabbitMQ (when needed)

```
Client → Redirect Handler → basic_publish to RabbitMQ → Response
                                       ↓
                              Consumer service
                                       ↓
                              Batch INSERT into PG
```

- Redirector publishes a message and forgets
- Separate consumer service writes to PG in batches
- RabbitMQ is already in infrastructure
- Zero data loss — messages persist in queue across crashes
- Consumer can be a separate microservice or a standalone binary in this repo

## Click Event Structure

| Field | Type | Source | Notes |
|-------|------|--------|-------|
| `url_id` | BIGINT | Decoded from hashid | Already available in handler |
| `ip` | INET | Request headers | `X-Forwarded-For` or `X-Real-IP` behind proxy |
| `user_agent` | TEXT | `User-Agent` header | Stored via dictionary (ua_id) |
| `referrer` | TEXT | `Referer` header | Raw value |
| `referrer_domain` | TEXT | Extracted from referrer | For grouping/dictionary |
| `created_at` | TIMESTAMPTZ | `now()` | DB default |

## PostgreSQL Type Choices

- **`INET`** for IP: 4 bytes (IPv4) / 16 bytes (IPv6), native subnet operators (`>>=`), GiST indexable
- **`TIMESTAMPTZ`** for time: proper timezone handling, partition-friendly
- **`BIGSERIAL`** for id: auto-increment, no application-side generation

## In-App Implementation

### Redirect handler change

```rust
// After resolving URL, before showing interstitial
analytics::record_click(url_id, &request).await;
```

### UA dictionary cache

```rust
struct UaCache {
    cache: HashMap<u32, i32>,  // crc32 hash → DB id
}

impl UaCache {
    fn get_or_insert(&mut self, ua: &str, db: &PgPool) -> i32 {
        let hash = crc32fast::hash(ua.as_bytes()) as i32;
        if let Some(&id) = self.cache.get(&hash) {
            return id;
        }
        // INSERT ... ON CONFLICT DO NOTHING RETURNING id
        let id = insert_ua(db, hash, ua).await;
        self.cache.insert(hash, id);
        id
    }
}
```

### IP extraction

```rust
fn client_ip(headers: &HeaderMap, addr: SocketAddr) -> IpAddr {
    headers.get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(addr.ip())
}
```

## Configuration

```yaml
analytics:
  enabled: true
  # Phase 1
  storage: postgres        # or "rabbitmq" for Phase 2
  # Phase 2 (future)
  rabbitmq:
    url: ${RABBITMQ_URL}
    queue: redirector.clicks
    exchange: redirector
```

## Cleanup

Monthly partition management:

```sql
-- Detach old data (instant, no locks)
ALTER TABLE analytics.clicks DETACH PARTITION analytics.clicks_2025_01;

-- Drop when no longer needed
DROP TABLE analytics.clicks_2025_01;
```

## Out of Scope (for now)

- Geographic data (GeoIP) — requires MaxMind DB or similar
- Analytics dashboard — read from PG later
- Export to CSV/JSON — standard `COPY` from partitions
- Aggregation tables — premature at low volumes
