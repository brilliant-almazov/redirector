use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

static START_TIME: Lazy<Instant> = Lazy::new(Instant::now);

// Recent redirects buffer
const MAX_RECENT: usize = 50;

#[derive(Clone)]
pub struct RecentRedirect {
    pub hashid: String,
    pub url: String,
    pub timestamp: u64,
}

static RECENT_REDIRECTS: Lazy<RwLock<VecDeque<RecentRedirect>>> =
    Lazy::new(|| RwLock::new(VecDeque::new()));

// Global counters for SSE dashboard
pub static TOTAL_REQUESTS: AtomicU64 = AtomicU64::new(0);
pub static CACHE_HITS: AtomicU64 = AtomicU64::new(0);
pub static CACHE_MISSES: AtomicU64 = AtomicU64::new(0);

// Latency tracking (using simple moving average approximation)
pub static LATENCY_SUM_MICROS: AtomicU64 = AtomicU64::new(0);
pub static LATENCY_COUNT: AtomicU64 = AtomicU64::new(0);

/// Initialize service metrics
pub fn init() {
    // Record start time to calculate uptime
    let _ = *START_TIME;

    // Build info as gauge
    metrics::gauge!("redirector_build_info",
        "version" => env!("CARGO_PKG_VERSION")
    )
    .set(1.0);

    // Service up indicator
    metrics::gauge!("redirector_up").set(1.0);

    // Initial uptime
    metrics::gauge!("redirector_uptime_seconds").set(0.0);
}

/// Update runtime metrics - should be called from metrics endpoint
pub fn update() {
    let uptime = START_TIME.elapsed().as_secs_f64();
    metrics::gauge!("redirector_uptime_seconds").set(uptime);
}

/// Get uptime in seconds
pub fn uptime_secs() -> u64 {
    START_TIME.elapsed().as_secs()
}

/// Record a request
pub fn record_request(latency_micros: u64) {
    TOTAL_REQUESTS.fetch_add(1, Ordering::Relaxed);
    LATENCY_SUM_MICROS.fetch_add(latency_micros, Ordering::Relaxed);
    LATENCY_COUNT.fetch_add(1, Ordering::Relaxed);
}

/// Record cache hit
pub fn record_cache_hit() {
    CACHE_HITS.fetch_add(1, Ordering::Relaxed);
}

/// Record cache miss
pub fn record_cache_miss() {
    CACHE_MISSES.fetch_add(1, Ordering::Relaxed);
}

/// Get total requests
pub fn get_total_requests() -> u64 {
    TOTAL_REQUESTS.load(Ordering::Relaxed)
}

/// Get cache hit rate (0.0 - 1.0)
pub fn get_cache_hit_rate() -> f64 {
    let hits = CACHE_HITS.load(Ordering::Relaxed) as f64;
    let misses = CACHE_MISSES.load(Ordering::Relaxed) as f64;
    let total = hits + misses;
    if total > 0.0 { hits / total } else { 0.0 }
}

/// Get average latency in ms
pub fn get_avg_latency_ms() -> f64 {
    let sum = LATENCY_SUM_MICROS.load(Ordering::Relaxed) as f64;
    let count = LATENCY_COUNT.load(Ordering::Relaxed) as f64;
    if count > 0.0 { sum / count / 1000.0 } else { 0.0 }
}

/// Record a recent redirect
pub fn record_recent_redirect(hashid: String, url: String) {
    let entry = RecentRedirect {
        hashid,
        url,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
    };
    let mut recent = RECENT_REDIRECTS.write();
    recent.push_front(entry);
    while recent.len() > MAX_RECENT {
        recent.pop_back();
    }
}

/// Get recent redirects
pub fn get_recent_redirects() -> Vec<RecentRedirect> {
    RECENT_REDIRECTS.read().iter().cloned().collect()
}
