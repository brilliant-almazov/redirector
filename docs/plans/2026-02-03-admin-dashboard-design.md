# Admin Dashboard Design

## Overview

Real-time admin dashboard for monitoring redirector service. Built with Leptos (Rust), SSE for live updates, and Chart.js for graphs.

## Goals

- Live metrics visualization without page reload
- Simple static auth (users in config)
- Light, modern UI with blue accents
- Read-only dashboard (no admin actions in v1)
- Single binary (embedded in existing service)

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    redirector binary                     │
├─────────────────────────────────────────────────────────┤
│  Existing routes:                                        │
│    /r/{hashid}     → redirect handler                   │
│    /metrics        → prometheus                          │
│    /health         → health check                        │
├─────────────────────────────────────────────────────────┤
│  New admin routes:                                       │
│    /admin/         → login page (Leptos SSR)            │
│    /admin/dashboard→ main dashboard (Leptos SSR)        │
│    /admin/events   → SSE stream (live metrics)          │
└─────────────────────────────────────────────────────────┘
```

## Authentication

### Config

```yaml
admin:
  enabled: true
  session_secret: "random-32-byte-secret"
  session_ttl_hours: 24

  users:
    - username: admin
      password_hash: "$argon2id$v=19$m=65536,t=3,p=4$..."
```

### Flow

1. User visits `/admin/` → sees login form
2. Submits username/password
3. Server validates against Argon2 hash
4. On success: sets HttpOnly + Secure session cookie
5. Cookie contains random session token
6. Sessions stored in memory (HashMap<token, username>)
7. SSE endpoint also validates session cookie

### CLI Helper

```bash
./redirector hash-password "my-password"
# Output: $argon2id$v=19$m=65536,t=3,p=4$...
```

## SSE Data Stream

**Endpoint:** `GET /admin/events`

**Interval:** 500ms

**Payload:**

```json
{
  "timestamp": 1706900000,
  "system": {
    "uptime_secs": 300000,
    "cpu_percent": 23.5,
    "memory_mb": 128
  },
  "app": {
    "rps": 1250,
    "latency_p50_ms": 2.1,
    "latency_p95_ms": 8.3,
    "latency_p99_ms": 15.2,
    "cache_hit_rate": 0.94,
    "total_requests": 5420000
  },
  "recent": [
    {"hashid": "abc123", "url": "https://google.com", "at": 1706899999}
  ]
}
```

## UI Design

### Layout

```
┌────────────────────────────────────────────────────────┐
│  🔄 redirector                      [admin] [logout]   │
├────────────────────────────────────────────────────────┤
│                                                        │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐  │
│  │ Uptime   │ │ CPU      │ │ Memory   │ │ RPS      │  │
│  │ 3d 12h   │ │ 23%      │ │ 128 MB   │ │ 1,250    │  │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘  │
│                                                        │
│  ┌─────────────────────────────────────────────────┐  │
│  │          RPS + Latency (live chart)             │  │
│  │  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~   │  │
│  └─────────────────────────────────────────────────┘  │
│                                                        │
│  ┌─────────────────────┐ ┌─────────────────────────┐  │
│  │ Cache Hit Rate      │ │ Recent Redirects        │  │
│  │ [======= 94% =====] │ │ abc123 → google.com     │  │
│  │                     │ │ xyz789 → github.com     │  │
│  └─────────────────────┘ └─────────────────────────┘  │
└────────────────────────────────────────────────────────┘
```

### Style

- Background: `slate-50` / `white`
- Accents: `blue-500` / `blue-600`
- Text: `slate-700` / `slate-900`
- Cards: `shadow-sm`, rounded corners
- Tailwind CSS via CDN

## File Structure

```
src/
  admin/
    mod.rs              # module exports, route setup
    auth.rs             # login, logout, session middleware
    sse.rs              # SSE metrics stream handler
    state.rs            # shared state (sessions, recent redirects)
    pages/
      login.rs          # login page component
      dashboard.rs      # main dashboard component
    components/
      stats_card.rs     # single stat card
      chart.rs          # Chart.js wrapper
      recent_table.rs   # recent redirects table
      progress_bar.rs   # cache hit rate bar
```

## Dependencies

```toml
# Leptos
leptos = { version = "0.7", features = ["ssr"] }
leptos_axum = "0.7"

# Auth
argon2 = "0.5"

# System metrics
sysinfo = "0.32"

# Charts via CDN (no Rust dep)
```

## Metrics Collection

### System Metrics

Using `sysinfo` crate:
- CPU usage (%)
- Memory usage (MB)
- Uptime from existing `START_TIME`

### App Metrics

From existing Prometheus metrics:
- RPS: counter delta / time delta
- Latency percentiles: from histogram
- Cache hit rate: hits / (hits + misses)
- Total requests: counter value

### Recent Redirects

Ring buffer in memory (last 50):
- Captured in redirect handler
- Stored in `Arc<RwLock<VecDeque<RecentRedirect>>>`

## Implementation Plan

1. Add dependencies to Cargo.toml
2. Create admin module structure
3. Implement auth (config, login, session)
4. Implement SSE metrics stream
5. Create Leptos components
6. Style with Tailwind
7. Add Chart.js integration
8. Test locally
9. Update CI if needed

## Future Enhancements (not in v1)

- Admin actions (clear cache, etc.)
- User roles (admin/viewer)
- Persistent sessions (Redis)
- More detailed analytics
- Dark mode toggle
