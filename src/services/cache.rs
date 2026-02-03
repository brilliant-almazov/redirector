use crate::config::RedisConfig;
use crate::services::traits::Cache;
use async_trait::async_trait;
use deadpool_redis::{Config as PoolConfig, Pool, Runtime};
use redis::AsyncCommands;
use std::time::Instant;

pub struct CacheService {
    pool: Pool,
    ttl_seconds: u64,
}

impl CacheService {
    pub fn new(config: &RedisConfig) -> anyhow::Result<Self> {
        let pool_config = PoolConfig::from_url(&config.url);
        let pool = pool_config.create_pool(Some(Runtime::Tokio1))?;

        Ok(Self {
            pool,
            ttl_seconds: config.cache_ttl_seconds,
        })
    }

    pub(crate) fn key(id: i64) -> String {
        format!("url:{}", id)
    }
}

#[async_trait]
impl Cache for CacheService {
    async fn get(&self, id: i64) -> Option<String> {
        let start = Instant::now();
        let mut conn = self.pool.get().await.ok()?;
        let result: Option<String> = conn.get(Self::key(id)).await.ok()?;
        let duration = start.elapsed();

        metrics::histogram!("cache_get_duration_seconds").record(duration.as_secs_f64());
        result
    }

    async fn set(&self, id: i64, url: &str) -> anyhow::Result<()> {
        let start = Instant::now();
        let mut conn = self.pool.get().await?;
        let _: () = conn.set_ex(Self::key(id), url, self.ttl_seconds).await?;
        let duration = start.elapsed();

        metrics::histogram!("cache_set_duration_seconds").record(duration.as_secs_f64());
        Ok(())
    }
}

#[cfg(test)]
#[path = "cache_test.rs"]
mod cache_test;
