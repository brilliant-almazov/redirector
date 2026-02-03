use crate::config::DatabaseConfig;
use crate::error::{AppError, Result};
use crate::services::traits::UrlStorage;
use async_trait::async_trait;
use failsafe::{failure_policy, Config as CircuitConfig, StateMachine};
use governor::{Quota, RateLimiter};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::{Duration, Instant};

type FailurePolicy = failure_policy::ConsecutiveFailures<failsafe::backoff::Constant>;
type CircuitBreaker = StateMachine<FailurePolicy, ()>;

pub struct MainStorage {
    pool: PgPool,
    rate_limiter: Arc<
        RateLimiter<
            governor::state::NotKeyed,
            governor::state::InMemoryState,
            governor::clock::DefaultClock,
        >,
    >,
    circuit_breaker: Arc<tokio::sync::Mutex<CircuitBreaker>>,
}

impl MainStorage {
    pub async fn new(config: &DatabaseConfig) -> anyhow::Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(config.pool.max_connections)
            .acquire_timeout(Duration::from_secs(config.pool.connect_timeout_seconds))
            .connect(&config.url)
            .await?;

        let quota = Quota::per_second(
            NonZeroU32::new(config.rate_limit.max_requests_per_second)
                .unwrap_or(NonZeroU32::new(50).unwrap()),
        );
        let rate_limiter = Arc::new(RateLimiter::direct(quota));

        let circuit_breaker: CircuitBreaker = CircuitConfig::new()
            .failure_policy(failure_policy::consecutive_failures(
                config.circuit_breaker.failure_threshold,
                failsafe::backoff::constant(Duration::from_secs(
                    config.circuit_breaker.reset_timeout_seconds,
                )),
            ))
            .build();

        Ok(Self {
            pool,
            rate_limiter,
            circuit_breaker: Arc::new(tokio::sync::Mutex::new(circuit_breaker)),
        })
    }

    async fn query_url(&self, id: i64) -> Result<Option<String>> {
        let record: Option<(String, String)> = sqlx::query_as(
            r#"
            SELECT d.name as domain, u.name as path
            FROM dictionary.urls u
            JOIN dictionary.domains d ON u.domain_id = d.id
            WHERE u.id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(record.map(|(domain, path)| format!("{}{}", domain, path)))
    }

    pub async fn health_check(&self) -> Result<()> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;
        Ok(())
    }
}

#[async_trait]
impl UrlStorage for MainStorage {
    async fn get_url_by_id(&self, id: i64) -> Result<Option<String>> {
        {
            let cb = self.circuit_breaker.lock().await;
            if !cb.is_call_permitted() {
                tracing::warn!("Circuit breaker open, rejecting request");
                metrics::counter!("circuit_breaker_rejections").increment(1);
                return Err(AppError::ServiceUnavailable);
            }
        }

        if self.rate_limiter.check().is_err() {
            tracing::warn!("Database rate limit exceeded");
            metrics::counter!("db_rate_limit_exceeded").increment(1);
            return Err(AppError::RateLimitExceeded);
        }

        metrics::counter!("db_queries").increment(1);

        let start = Instant::now();
        let result = self.query_url(id).await;
        let duration = start.elapsed();

        metrics::histogram!("db_query_duration_seconds").record(duration.as_secs_f64());

        {
            let cb = self.circuit_breaker.lock().await;
            match &result {
                Ok(Some(_)) => {
                    cb.on_success();
                    metrics::counter!("db_hits").increment(1);
                }
                Ok(None) => {
                    cb.on_success();
                    metrics::counter!("db_misses").increment(1);
                }
                Err(_) => {
                    cb.on_error();
                    metrics::counter!("db_errors").increment(1);
                }
            }
        }

        result
    }
}
