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
    #[serde(default)]
    pub admin: AdminConfig,
}

pub type AppConfig = Config;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
        }
    }
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

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: "redis://localhost:6379".to_string(),
            cache_ttl_seconds: default_cache_ttl(),
        }
    }
}

fn default_cache_ttl() -> u64 {
    86400
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
    #[serde(default)]
    pub query: QueryConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct QueryConfig {
    #[serde(default = "default_table")]
    pub table: String,
    #[serde(default = "default_id_column")]
    pub id_column: String,
    #[serde(default = "default_url_column")]
    pub url_column: String,
}

impl Default for QueryConfig {
    fn default() -> Self {
        Self {
            table: default_table(),
            id_column: default_id_column(),
            url_column: default_url_column(),
        }
    }
}

fn default_table() -> String {
    "dictionary.urls".to_string()
}

fn default_id_column() -> String {
    "id".to_string()
}

fn default_url_column() -> String {
    "name".to_string()
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

#[derive(Debug, Deserialize, Clone)]
pub struct AdminConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_session_secret")]
    pub session_secret: String,
    #[serde(default = "default_session_ttl")]
    pub session_ttl_hours: u64,
    #[serde(default)]
    pub users: Vec<AdminUser>,
}

impl Default for AdminConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            session_secret: default_session_secret(),
            session_ttl_hours: default_session_ttl(),
            users: vec![],
        }
    }
}

fn default_session_secret() -> String {
    "change-me-in-production-32-bytes!".to_string()
}

fn default_session_ttl() -> u64 {
    24
}

#[derive(Debug, Deserialize, Clone)]
pub struct AdminUser {
    pub username: String,
    pub password_hash: String,
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
#[path = "config_test.rs"]
mod config_test;
