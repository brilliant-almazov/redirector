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
}
