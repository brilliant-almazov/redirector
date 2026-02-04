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

    #[test]
    fn test_load_from_base64_valid() {
        use base64::Engine;

        let yaml = r#"
server:
  host: "127.0.0.1"
  port: 9090
hashids:
  salts:
    - "test-salt"
  min_length: 8
redis:
  url: "redis://localhost"
database:
  url: "postgres://localhost/test"
  query:
    table: "urls"
    id_column: "id"
    url_column: "url"
interstitial:
  delay_seconds: 3
metrics:
  basic_auth:
    username: "admin"
    password: "secret"
"#;
        let encoded = base64::engine::general_purpose::STANDARD.encode(yaml);
        let config = Config::load_from_base64(&encoded).unwrap();
        assert_eq!(config.server.host, "127.0.0.1");
        assert_eq!(config.server.port, 9090);
        assert_eq!(config.hashids.salts[0], "test-salt");
        assert_eq!(config.hashids.min_length, 8);
        assert_eq!(config.interstitial.delay_seconds, 3);
    }

    #[test]
    fn test_load_from_base64_invalid_base64() {
        let result = Config::load_from_base64("not-valid-base64!!!");
        assert!(result.is_err());
    }

    #[test]
    fn test_load_from_base64_invalid_yaml() {
        use base64::Engine;

        let encoded = base64::engine::general_purpose::STANDARD.encode("not: valid: yaml: [[[");
        let result = Config::load_from_base64(&encoded);
        assert!(result.is_err());
    }

    #[test]
    fn test_load_from_base64_empty() {
        use base64::Engine;

        let encoded = base64::engine::general_purpose::STANDARD.encode("");
        let result = Config::load_from_base64(&encoded);
        assert!(result.is_err());
    }
}
