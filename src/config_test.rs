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

    // --- PaaS env override tests ---

    fn make_test_config() -> Config {
        Config {
            server: ServerConfig::default(),
            hashids: HashidsConfig {
                salts: vec!["test".to_string()],
                min_length: 6,
            },
            redis: RedisConfig::default(),
            database: DatabaseConfig {
                url: "postgres://original/db".to_string(),
                pool: PoolConfig::default(),
                rate_limit: DbRateLimitConfig::default(),
                circuit_breaker: CircuitBreakerConfig::default(),
                query: QueryConfig::default(),
            },
            interstitial: InterstitialConfig { delay_seconds: 5 },
            metrics: MetricsConfig {
                basic_auth: BasicAuthConfig {
                    username: "admin".to_string(),
                    password: "secret".to_string(),
                },
            },
            rate_limit: RateLimitConfig::default(),
            admin: AdminConfig::default(),
        }
    }

    /// Guard that saves/restores env vars on drop.
    struct EnvGuard(Vec<(String, Option<String>)>);

    impl EnvGuard {
        fn new() -> Self {
            Self(Vec::new())
        }

        fn set(&mut self, key: &str, val: &str) {
            let old = std::env::var(key).ok();
            self.0.push((key.to_string(), old));
            unsafe { std::env::set_var(key, val) };
        }

        fn remove(&mut self, key: &str) {
            let old = std::env::var(key).ok();
            self.0.push((key.to_string(), old));
            unsafe { std::env::remove_var(key) };
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            for (key, old) in self.0.iter().rev() {
                match old {
                    Some(val) => unsafe { std::env::set_var(key, val) },
                    None => unsafe { std::env::remove_var(key) },
                }
            }
        }
    }

    #[test]
    fn test_paas_database_url_override() {
        let mut guard = EnvGuard::new();
        guard.set("DATABASE_URL", "postgres://paas/mydb");
        guard.remove("REDIRECTOR__DATABASE__URL");

        let mut config = make_test_config();
        config.apply_paas_overrides();

        assert_eq!(config.database.url, "postgres://paas/mydb");
    }

    #[test]
    fn test_paas_redis_url_override() {
        let mut guard = EnvGuard::new();
        guard.set("REDIS_URL", "redis://paas-redis:6379");
        guard.remove("REDIRECTOR__REDIS__URL");

        let mut config = make_test_config();
        config.apply_paas_overrides();

        assert_eq!(config.redis.url, "redis://paas-redis:6379");
    }

    #[test]
    fn test_paas_port_override() {
        let mut guard = EnvGuard::new();
        guard.set("PORT", "3000");
        guard.remove("REDIRECTOR__SERVER__PORT");

        let mut config = make_test_config();
        config.apply_paas_overrides();

        assert_eq!(config.server.port, 3000);
    }

    #[test]
    fn test_paas_port_invalid_ignored() {
        let mut guard = EnvGuard::new();
        guard.set("PORT", "not-a-number");
        guard.remove("REDIRECTOR__SERVER__PORT");

        let mut config = make_test_config();
        let original_port = config.server.port;
        config.apply_paas_overrides();

        assert_eq!(config.server.port, original_port);
    }

    #[test]
    fn test_prefixed_var_takes_priority_over_paas() {
        let mut guard = EnvGuard::new();
        guard.set("DATABASE_URL", "postgres://paas/mydb");
        guard.set("REDIRECTOR__DATABASE__URL", "postgres://explicit/mydb");

        let mut config = make_test_config();
        config.apply_paas_overrides();

        // DATABASE_URL should NOT override because REDIRECTOR__DATABASE__URL is set
        assert_ne!(config.database.url, "postgres://paas/mydb");
    }

    #[test]
    fn test_no_paas_vars_keeps_original() {
        let mut guard = EnvGuard::new();
        guard.remove("DATABASE_URL");
        guard.remove("REDIS_URL");
        guard.remove("PORT");
        guard.remove("REDIRECTOR__DATABASE__URL");
        guard.remove("REDIRECTOR__REDIS__URL");
        guard.remove("REDIRECTOR__SERVER__PORT");

        let mut config = make_test_config();
        let original_db = config.database.url.clone();
        let original_redis = config.redis.url.clone();
        let original_port = config.server.port;
        config.apply_paas_overrides();

        assert_eq!(config.database.url, original_db);
        assert_eq!(config.redis.url, original_redis);
        assert_eq!(config.server.port, original_port);
    }

    #[test]
    fn test_paas_mappings_constant() {
        assert_eq!(PAAS_ENV_MAPPINGS.len(), 3);

        let paas_vars: Vec<&str> = PAAS_ENV_MAPPINGS.iter().map(|(k, _)| *k).collect();
        assert!(paas_vars.contains(&"DATABASE_URL"));
        assert!(paas_vars.contains(&"REDIS_URL"));
        assert!(paas_vars.contains(&"PORT"));
    }

    #[test]
    fn test_hashids_salts_env_override() {
        // All HASHIDS_SALTS tests in one function to avoid env var race conditions.
        let mut guard = EnvGuard::new();
        guard.remove("REDIRECTOR__HASHIDS__SALTS__0");

        // Comma-separated: two salts
        guard.set("HASHIDS_SALTS", "new-salt,old-salt");
        let mut config = make_test_config();
        config.apply_paas_overrides();
        assert_eq!(config.hashids.salts, vec!["new-salt", "old-salt"]);

        // Comma-separated: with spaces (trimmed)
        guard.set("HASHIDS_SALTS", " salt-a , salt-b , salt-c ");
        let mut config = make_test_config();
        config.apply_paas_overrides();
        assert_eq!(config.hashids.salts, vec!["salt-a", "salt-b", "salt-c"]);

        // Single salt (no comma)
        guard.set("HASHIDS_SALTS", "only-salt");
        let mut config = make_test_config();
        config.apply_paas_overrides();
        assert_eq!(config.hashids.salts, vec!["only-salt"]);

        // Indexed var takes priority over HASHIDS_SALTS
        guard.set("HASHIDS_SALTS", "comma-salt");
        guard.set("REDIRECTOR__HASHIDS__SALTS__0", "indexed-salt");
        let mut config = make_test_config();
        config.apply_paas_overrides();
        assert_ne!(config.hashids.salts, vec!["comma-salt".to_string()]);
    }
}
