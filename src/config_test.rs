#[cfg(test)]
mod tests {
    use crate::config::*;

    #[test]
    fn test_default_server_host() {
        assert_eq!(ServerConfig::default().host, "0.0.0.0");
    }

    #[test]
    fn test_default_server_port() {
        assert_eq!(ServerConfig::default().port, 8080);
    }

    #[test]
    fn test_default_hashids_min_length() {
        let config = HashidsConfig {
            salts: vec!["test".to_string()],
            min_length: 6,
        };
        assert_eq!(config.min_length, 6);
    }

    #[test]
    fn test_default_redis_cache_ttl() {
        assert_eq!(RedisConfig::default().cache_ttl_seconds, 86400);
    }

    #[test]
    fn test_default_pool_max_connections() {
        assert_eq!(PoolConfig::default().max_connections, 3);
    }

    #[test]
    fn test_default_pool_connect_timeout() {
        assert_eq!(PoolConfig::default().connect_timeout_seconds, 3);
    }

    #[test]
    fn test_default_db_rate_limit() {
        assert_eq!(DbRateLimitConfig::default().max_requests_per_second, 50);
    }

    #[test]
    fn test_default_circuit_breaker_failure_threshold() {
        assert_eq!(CircuitBreakerConfig::default().failure_threshold, 3);
    }

    #[test]
    fn test_default_circuit_breaker_reset_timeout() {
        assert_eq!(CircuitBreakerConfig::default().reset_timeout_seconds, 60);
    }

    #[test]
    fn test_default_interstitial_delay() {
        let config = InterstitialConfig { delay_seconds: 5 };
        assert_eq!(config.delay_seconds, 5);
    }

    #[test]
    fn test_default_rate_limit_rps() {
        assert_eq!(RateLimitConfig::default().requests_per_second, 1000);
    }

    #[test]
    fn test_default_rate_limit_burst() {
        assert_eq!(RateLimitConfig::default().burst, 100);
    }

    #[test]
    fn test_default_query_table() {
        assert_eq!(QueryConfig::default().table, "dictionary.urls");
    }

    #[test]
    fn test_default_query_id_column() {
        assert_eq!(QueryConfig::default().id_column, "id");
    }

    #[test]
    fn test_default_query_url_column() {
        assert_eq!(QueryConfig::default().url_column, "name");
    }
}
