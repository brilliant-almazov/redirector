use base64::Engine;
use serde::Deserialize;
use std::fmt;
use std::path::Path;

// ---------------------------------------------------------------
// Configuration Error Type
// ---------------------------------------------------------------

#[derive(Debug)]
pub enum ConfigError {
    MissingEnvVars(Vec<String>),
    InvalidJson { var: String, message: String },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::MissingEnvVars(vars) => {
                write!(
                    f,
                    "Missing required environment variables: {}",
                    vars.join(", ")
                )
            }
            ConfigError::InvalidJson { var, message } => {
                write!(
                    f,
                    "Invalid JSON in environment variable {}: {}",
                    var, message
                )
            }
        }
    }
}

impl std::error::Error for ConfigError {}

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
    #[serde(default)]
    pub events: EventsConfig,
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

// ---------------------------------------------------------------
// Events / RabbitMQ configuration
// ---------------------------------------------------------------

#[derive(Debug, Deserialize, Clone, Default)]
pub struct EventsConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub rabbitmq: RabbitMqConnectionConfig,
    #[serde(default)]
    pub publisher: PublisherConfig,
    #[serde(default)]
    pub consumer: ConsumerConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RabbitMqConnectionConfig {
    #[serde(default = "default_rabbitmq_url")]
    pub url: String,
    #[serde(default = "default_queue")]
    pub queue: String,
}

impl Default for RabbitMqConnectionConfig {
    fn default() -> Self {
        Self {
            url: default_rabbitmq_url(),
            queue: default_queue(),
        }
    }
}

fn default_rabbitmq_url() -> String {
    "amqp://guest:guest@localhost:5672/%2f".to_string()
}

fn default_queue() -> String {
    "events.analytics".to_string()
}

#[derive(Debug, Deserialize, Clone)]
pub struct PublisherConfig {
    #[serde(default = "default_channel_buffer")]
    pub channel_buffer_size: usize,
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,
    #[serde(default = "default_flush_interval_ms")]
    pub flush_interval_ms: u64,
}

impl Default for PublisherConfig {
    fn default() -> Self {
        Self {
            channel_buffer_size: default_channel_buffer(),
            batch_size: default_batch_size(),
            flush_interval_ms: default_flush_interval_ms(),
        }
    }
}

fn default_channel_buffer() -> usize {
    10_000
}

fn default_batch_size() -> usize {
    100
}

fn default_flush_interval_ms() -> u64 {
    1_000
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConsumerConfig {
    #[serde(default = "default_prefetch_count")]
    pub prefetch_count: u16,
    #[serde(default = "default_consumer_db_url")]
    pub database_url: String,
}

impl Default for ConsumerConfig {
    fn default() -> Self {
        Self {
            prefetch_count: default_prefetch_count(),
            database_url: default_consumer_db_url(),
        }
    }
}

fn default_prefetch_count() -> u16 {
    10
}

fn default_consumer_db_url() -> String {
    "postgres://localhost/redirector_analytics".to_string()
}

/// Standard PaaS environment variable mappings.
/// Each entry: (PaaS env var, corresponding REDIRECTOR__ prefixed var).
const PAAS_ENV_MAPPINGS: &[(&str, &str)] = &[
    ("DATABASE_URL", "REDIRECTOR__DATABASE__URL"),
    ("REDIS_URL", "REDIRECTOR__REDIS__URL"),
    ("PORT", "REDIRECTOR__SERVER__PORT"),
    ("RABBITMQ_URL", "REDIRECTOR__EVENTS__RABBITMQ__URL"),
];

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

        let mut config: Config = settings.try_deserialize()?;
        config.apply_paas_overrides();
        Ok(config)
    }

    pub fn load_from_base64(encoded: &str) -> anyhow::Result<Self> {
        let bytes = base64::engine::general_purpose::STANDARD.decode(encoded)?;
        let yaml = String::from_utf8(bytes)?;

        let settings = config::Config::builder()
            .add_source(config::File::from_str(&yaml, config::FileFormat::Yaml))
            .add_source(
                config::Environment::default()
                    .separator("__")
                    .prefix("REDIRECTOR"),
            )
            .build()?;

        let mut config: Config = settings.try_deserialize()?;
        config.apply_paas_overrides();
        Ok(config)
    }

    /// Apply standard PaaS environment variables (DATABASE_URL, REDIS_URL, PORT, HASHIDS_SALTS).
    /// These only take effect if the explicit REDIRECTOR__* version is not set,
    /// giving the prefixed form higher priority.
    fn apply_paas_overrides(&mut self) {
        for &(paas_var, prefixed_var) in PAAS_ENV_MAPPINGS {
            if std::env::var(prefixed_var).is_ok() {
                continue;
            }
            if let Ok(val) = std::env::var(paas_var) {
                match paas_var {
                    "DATABASE_URL" => self.database.url = val,
                    "REDIS_URL" => self.redis.url = val,
                    "RABBITMQ_URL" => self.events.rabbitmq.url = val,
                    "PORT" => {
                        if let Ok(port) = val.parse::<u16>() {
                            self.server.port = port;
                        }
                    }
                    _ => {}
                }
            }
        }

        // Support comma-separated hashid salts via HASHIDS_SALTS env var.
        // Only applies if no REDIRECTOR__HASHIDS__SALTS__* indexed vars are set.
        if std::env::var("REDIRECTOR__HASHIDS__SALTS__0").is_err() {
            if let Ok(salts) = std::env::var("HASHIDS_SALTS") {
                let parsed: Vec<String> = salts
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                if !parsed.is_empty() {
                    self.hashids.salts = parsed;
                }
            }
        }
    }

    /// Load configuration purely from environment variables.
    ///
    /// Required variables:
    /// - `DATABASE_URL` - PostgreSQL connection string
    /// - `REDIS_URL` - Redis connection string
    /// - `HASHIDS_SALTS` - Comma-separated salts (e.g., `salt1,salt2`)
    /// - `METRICS_USERNAME` - Basic auth username for /metrics
    /// - `METRICS_PASSWORD` - Basic auth password for /metrics
    ///
    /// Optional variables (with defaults):
    /// - `HOST` (default: 0.0.0.0)
    /// - `PORT` (default: 8080)
    /// - `ADMIN_ENABLED` (default: false)
    /// - `ADMIN_USERS` - JSON array: `[{"username":"x","password_hash":"y"}]`
    /// - `EVENTS_ENABLED` (default: false)
    /// - `RABBITMQ_URL` (default: amqp://guest:guest@localhost:5672/%2f)
    ///
    /// See docs/CONFIGURATION.md for full list of environment variables.
    pub fn load_from_env() -> Result<Self, ConfigError> {
        // Collect all missing required vars for better error messages
        let mut missing_vars = Vec::new();

        let database_url = std::env::var("DATABASE_URL");
        let redis_url = std::env::var("REDIS_URL");
        let metrics_username = std::env::var("METRICS_USERNAME");
        let metrics_password = std::env::var("METRICS_PASSWORD");
        let hashids_salts = Self::parse_hashids_salts_from_env();

        if database_url.is_err() {
            missing_vars.push("DATABASE_URL".to_string());
        }
        if redis_url.is_err() {
            missing_vars.push("REDIS_URL".to_string());
        }
        if metrics_username.is_err() {
            missing_vars.push("METRICS_USERNAME".to_string());
        }
        if metrics_password.is_err() {
            missing_vars.push("METRICS_PASSWORD".to_string());
        }
        if hashids_salts.is_empty() {
            missing_vars.push("HASHIDS_SALTS".to_string());
        }

        if !missing_vars.is_empty() {
            return Err(ConfigError::MissingEnvVars(missing_vars));
        }

        // Parse admin users from JSON
        let admin_users = Self::parse_admin_users_from_env()?;

        let config = Config {
            server: ServerConfig {
                host: env_or_default("HOST", &default_host()),
                port: env_parse_or_default("PORT", default_port()),
            },
            hashids: HashidsConfig {
                salts: hashids_salts,
                min_length: env_parse_or_default("HASHIDS_MIN_LENGTH", default_min_length()),
            },
            redis: RedisConfig {
                url: redis_url.unwrap(),
                cache_ttl_seconds: env_parse_or_default("REDIS_CACHE_TTL", default_cache_ttl()),
            },
            database: DatabaseConfig {
                url: database_url.unwrap(),
                pool: PoolConfig {
                    max_connections: env_parse_or_default(
                        "DB_MAX_CONNECTIONS",
                        default_max_connections(),
                    ),
                    connect_timeout_seconds: env_parse_or_default(
                        "DB_CONNECT_TIMEOUT",
                        default_connect_timeout(),
                    ),
                },
                rate_limit: DbRateLimitConfig {
                    max_requests_per_second: env_parse_or_default("DB_RPS", default_db_rps()),
                },
                circuit_breaker: CircuitBreakerConfig {
                    failure_threshold: env_parse_or_default(
                        "CB_FAILURE_THRESHOLD",
                        default_failure_threshold(),
                    ),
                    reset_timeout_seconds: env_parse_or_default(
                        "CB_RESET_TIMEOUT",
                        default_reset_timeout(),
                    ),
                },
                query: QueryConfig {
                    table: env_or_default("DB_TABLE", &default_table()),
                    id_column: env_or_default("DB_ID_COLUMN", &default_id_column()),
                    url_column: env_or_default("DB_URL_COLUMN", &default_url_column()),
                },
            },
            interstitial: InterstitialConfig {
                delay_seconds: env_parse_or_default("INTERSTITIAL_DELAY", default_delay()),
            },
            metrics: MetricsConfig {
                basic_auth: BasicAuthConfig {
                    username: metrics_username.unwrap(),
                    password: metrics_password.unwrap(),
                },
            },
            rate_limit: RateLimitConfig {
                requests_per_second: env_parse_or_default("RATE_LIMIT_RPS", default_rps()),
                burst: env_parse_or_default("RATE_LIMIT_BURST", default_burst()),
            },
            admin: AdminConfig {
                enabled: env_parse_bool("ADMIN_ENABLED", false),
                session_secret: env_or_default("ADMIN_SESSION_SECRET", &default_session_secret()),
                session_ttl_hours: env_parse_or_default(
                    "ADMIN_SESSION_TTL_HOURS",
                    default_session_ttl(),
                ),
                users: admin_users,
            },
            events: EventsConfig {
                enabled: env_parse_bool("EVENTS_ENABLED", false),
                rabbitmq: RabbitMqConnectionConfig {
                    url: env_or_default("RABBITMQ_URL", &default_rabbitmq_url()),
                    queue: env_or_default("RABBITMQ_QUEUE", &default_queue()),
                },
                publisher: PublisherConfig {
                    channel_buffer_size: env_parse_or_default(
                        "PUBLISHER_BUFFER_SIZE",
                        default_channel_buffer(),
                    ),
                    batch_size: env_parse_or_default("PUBLISHER_BATCH_SIZE", default_batch_size()),
                    flush_interval_ms: env_parse_or_default(
                        "PUBLISHER_FLUSH_INTERVAL_MS",
                        default_flush_interval_ms(),
                    ),
                },
                consumer: ConsumerConfig {
                    prefetch_count: env_parse_or_default(
                        "CONSUMER_PREFETCH_COUNT",
                        default_prefetch_count(),
                    ),
                    database_url: env_or_default(
                        "CONSUMER_DATABASE_URL",
                        &default_consumer_db_url(),
                    ),
                },
            },
        };

        Ok(config)
    }

    /// Parse hashids salts from environment.
    /// Supports comma-separated format: `HASHIDS_SALTS=salt1,salt2,salt3`
    fn parse_hashids_salts_from_env() -> Vec<String> {
        if let Ok(salts_str) = std::env::var("HASHIDS_SALTS") {
            let salts: Vec<String> = salts_str
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            if !salts.is_empty() {
                return salts;
            }
        }
        vec![]
    }

    /// Parse admin users from JSON environment variable.
    /// Expected format: `ADMIN_USERS='[{"username":"admin","password_hash":"$argon2..."}]'`
    fn parse_admin_users_from_env() -> Result<Vec<AdminUser>, ConfigError> {
        match std::env::var("ADMIN_USERS") {
            Ok(json_str) if !json_str.is_empty() => {
                serde_json::from_str(&json_str).map_err(|e| ConfigError::InvalidJson {
                    var: "ADMIN_USERS".to_string(),
                    message: e.to_string(),
                })
            }
            _ => Ok(vec![]),
        }
    }
}

// ---------------------------------------------------------------
// Environment variable helper functions
// ---------------------------------------------------------------

/// Get an environment variable or return a default value.
fn env_or_default(name: &str, default: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| default.to_string())
}

/// Parse an environment variable as a number, or return a default.
fn env_parse_or_default<T>(name: &str, default: T) -> T
where
    T: std::str::FromStr,
{
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

/// Parse an environment variable as a boolean.
/// Recognizes: "true", "1", "yes" (case-insensitive) as true.
fn env_parse_bool(name: &str, default: bool) -> bool {
    std::env::var(name)
        .ok()
        .map(|v| matches!(v.to_lowercase().as_str(), "true" | "1" | "yes"))
        .unwrap_or(default)
}

#[cfg(test)]
#[path = "config_test.rs"]
mod config_test;
