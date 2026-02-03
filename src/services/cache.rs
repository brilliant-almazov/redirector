use crate::config::RedisConfig;
use crate::services::traits::Cache;
use async_trait::async_trait;
use deadpool_redis::{Config as PoolConfig, Pool, Runtime};
use redis::AsyncCommands;

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

    fn key(id: i64) -> String {
        format!("url:{}", id)
    }
}

#[async_trait]
impl Cache for CacheService {
    async fn get(&self, id: i64) -> Option<String> {
        let mut conn = self.pool.get().await.ok()?;
        let result: Option<String> = conn.get(Self::key(id)).await.ok()?;
        result
    }

    async fn set(&self, id: i64, url: &str) -> anyhow::Result<()> {
        let mut conn = self.pool.get().await?;
        let _: () = conn.set_ex(Self::key(id), url, self.ttl_seconds).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_key_format() {
        assert_eq!(CacheService::key(42), "url:42");
        assert_eq!(CacheService::key(0), "url:0");
        assert_eq!(CacheService::key(999999), "url:999999");
        assert_eq!(CacheService::key(-1), "url:-1");
    }
}
