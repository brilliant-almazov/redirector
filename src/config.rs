use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub hashids: HashidsConfig,
    pub redis: RedisConfig,
    pub database: DatabaseConfig,
    pub interstitial: InterstitialConfig,
    pub metrics: MetricsConfig,
    #[serde(default)]
    pub rate_limit: RateLimitConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    8080
}

#[derive(Debug, Deserialize, Clone)]
pub struct HashidsConfig {
    pub salts: Vec<String>,
    #[serde(default = "default_min_length")]
    pub min_length: usize,
}

fn default_min_length() -> usize {
    6
}

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub url: String,
    #[serde(default = "default_cache_ttl")]
    pub cache_ttl_seconds: u64,
}

fn default_cache_ttl() -> u64 {
    86400 // 24 hours
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    #[serde(default)]
    pub pool: PoolConfig,
    #[serde(default)]
    pub rate_limit: DbRateLimitConfig,
    #[serde(default)]
    pub circuit_breaker: CircuitBreakerConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PoolConfig {
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
    #[serde(default = "default_connect_timeout")]
    pub connect_timeout_seconds: u64,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            max_connections: default_max_connections(),
            connect_timeout_seconds: default_connect_timeout(),
        }
    }
}

fn default_max_connections() -> u32 {
    3
}

fn default_connect_timeout() -> u64 {
    3
}

#[derive(Debug, Deserialize, Clone)]
pub struct DbRateLimitConfig {
    #[serde(default = "default_db_rps")]
    pub max_requests_per_second: u32,
}

impl Default for DbRateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests_per_second: default_db_rps(),
        }
    }
}

fn default_db_rps() -> u32 {
    50
}

#[derive(Debug, Deserialize, Clone)]
pub struct CircuitBreakerConfig {
    #[serde(default = "default_failure_threshold")]
    pub failure_threshold: u32,
    #[serde(default = "default_reset_timeout")]
    pub reset_timeout_seconds: u64,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: default_failure_threshold(),
            reset_timeout_seconds: default_reset_timeout(),
        }
    }
}

fn default_failure_threshold() -> u32 {
    3
}

fn default_reset_timeout() -> u64 {
    60
}

#[derive(Debug, Deserialize, Clone)]
pub struct InterstitialConfig {
    #[serde(default = "default_delay")]
    pub delay_seconds: u32,
}

fn default_delay() -> u32 {
    5
}

#[derive(Debug, Deserialize, Clone)]
pub struct MetricsConfig {
    pub basic_auth: BasicAuthConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BasicAuthConfig {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RateLimitConfig {
    #[serde(default = "default_rps")]
    pub requests_per_second: u32,
    #[serde(default = "default_burst")]
    pub burst: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_second: default_rps(),
            burst: default_burst(),
        }
    }
}

fn default_rps() -> u32 {
    1000
}

fn default_burst() -> u32 {
    100
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let settings = config::Config::builder()
            .add_source(config::File::from(path.as_ref()))
            .add_source(
                config::Environment::default()
                    .separator("__")
                    .prefix("REDIRECTOR"),
            )
            .build()?;

        let config: Config = settings.try_deserialize()?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_values() {
        assert_eq!(default_host(), "0.0.0.0");
        assert_eq!(default_port(), 8080);
        assert_eq!(default_min_length(), 6);
        assert_eq!(default_cache_ttl(), 86400);
        assert_eq!(default_max_connections(), 3);
        assert_eq!(default_connect_timeout(), 3);
        assert_eq!(default_db_rps(), 50);
        assert_eq!(default_failure_threshold(), 3);
        assert_eq!(default_reset_timeout(), 60);
        assert_eq!(default_delay(), 5);
        assert_eq!(default_rps(), 1000);
        assert_eq!(default_burst(), 100);
    }
}
