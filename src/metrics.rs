use once_cell::sync::Lazy;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::RwLock;
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
    if total > 0.0 {
        hits / total
    } else {
        0.0
    }
}

/// Get average latency in ms
pub fn get_avg_latency_ms() -> f64 {
    let sum = LATENCY_SUM_MICROS.load(Ordering::Relaxed) as f64;
    let count = LATENCY_COUNT.load(Ordering::Relaxed) as f64;
    if count > 0.0 {
        sum / count / 1000.0
    } else {
        0.0
    }
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
    let mut recent = RECENT_REDIRECTS.write().unwrap();
    recent.push_front(entry);
    while recent.len() > MAX_RECENT {
        recent.pop_back();
    }
}

/// Get recent redirects
pub fn get_recent_redirects() -> Vec<RecentRedirect> {
    RECENT_REDIRECTS.read().unwrap().iter().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uptime_secs() {
        let uptime = uptime_secs();
        assert!(uptime < 10); // Should be very small in tests
    }

    #[test]
    fn test_record_request() {
        let before = TOTAL_REQUESTS.load(Ordering::Relaxed);
        record_request(1000);
        let after = TOTAL_REQUESTS.load(Ordering::Relaxed);
        assert!(after > before);
    }

    #[test]
    fn test_record_cache_hit() {
        let before = CACHE_HITS.load(Ordering::Relaxed);
        record_cache_hit();
        let after = CACHE_HITS.load(Ordering::Relaxed);
        assert_eq!(after, before + 1);
    }

    #[test]
    fn test_record_cache_miss() {
        let before = CACHE_MISSES.load(Ordering::Relaxed);
        record_cache_miss();
        let after = CACHE_MISSES.load(Ordering::Relaxed);
        assert_eq!(after, before + 1);
    }

    #[test]
    fn test_get_total_requests() {
        // Just verify function returns without panic
        let _total = get_total_requests();
    }

    #[test]
    fn test_get_cache_hit_rate() {
        let rate = get_cache_hit_rate();
        assert!((0.0..=1.0).contains(&rate));
    }

    #[test]
    fn test_get_avg_latency_ms() {
        let latency = get_avg_latency_ms();
        assert!(latency >= 0.0);
    }

    #[test]
    fn test_record_recent_redirect() {
        let before_len = get_recent_redirects().len();
        record_recent_redirect("test123".to_string(), "https://test.com".to_string());
        let after = get_recent_redirects();
        assert!(after.len() >= before_len);
        assert!(after.iter().any(|r| r.hashid == "test123"));
    }

    #[test]
    fn test_get_recent_redirects() {
        let recent = get_recent_redirects();
        assert!(recent.len() <= MAX_RECENT);
    }

    #[test]
    fn test_recent_redirect_max_size() {
        // Add more than MAX_RECENT items
        for i in 0..MAX_RECENT + 10 {
            record_recent_redirect(format!("hash{}", i), format!("https://url{}.com", i));
        }
        let recent = get_recent_redirects();
        assert!(recent.len() <= MAX_RECENT);
    }
}
