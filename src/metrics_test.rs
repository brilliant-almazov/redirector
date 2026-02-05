use super::*;
use std::sync::atomic::Ordering;

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
    for i in 0..MAX_RECENT + 10 {
        record_recent_redirect(format!("hash{}", i), format!("https://url{}.com", i));
    }
    let recent = get_recent_redirects();
    assert!(recent.len() <= MAX_RECENT);
}
