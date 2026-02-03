#[cfg(test)]
mod tests {
    use crate::middleware::RateLimitLayer;

    #[test]
    fn test_rate_limiter_allows_requests() {
        let limiter = RateLimitLayer::new(10, 10);

        for _ in 0..10 {
            assert!(limiter.check());
        }
    }

    #[test]
    fn test_rate_limiter_blocks_excess() {
        let limiter = RateLimitLayer::new(1, 1);

        assert!(limiter.check());
        assert!(!limiter.check());
    }

    #[test]
    fn test_rate_limiter_with_burst() {
        let limiter = RateLimitLayer::new(1, 5);

        // Should allow burst
        for _ in 0..5 {
            assert!(limiter.check());
        }

        // Should block after burst
        assert!(!limiter.check());
    }

    #[test]
    fn test_rate_limiter_high_rate() {
        let limiter = RateLimitLayer::new(1000, 100);

        // Should allow many requests with high rate
        for _ in 0..100 {
            assert!(limiter.check());
        }
    }

    #[test]
    fn test_rate_limiter_clone() {
        let limiter1 = RateLimitLayer::new(10, 10);
        let limiter2 = limiter1.clone();

        // Both limiters share the same state
        for _ in 0..10 {
            limiter1.check();
        }
        // limiter2 should also be affected since they share Arc
        assert!(!limiter2.check());
    }

    #[test]
    fn test_rate_limiter_minimum_values() {
        let limiter = RateLimitLayer::new(1, 1);
        assert!(limiter.check());
        assert!(!limiter.check());
    }
}
