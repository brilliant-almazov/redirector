use crate::error::Result;
use async_trait::async_trait;

/// Trait for URL storage operations
#[async_trait]
pub trait UrlStorage: Send + Sync {
    async fn get_url_by_id(&self, id: i64) -> Result<Option<String>>;
}

/// Trait for cache operations
#[async_trait]
pub trait Cache: Send + Sync {
    async fn get(&self, id: i64) -> Option<String>;
    async fn set(&self, id: i64, url: &str) -> anyhow::Result<()>;
}

/// Trait for hashid decoding
pub trait HashidDecoder: Send + Sync {
    fn decode(&self, hashid: &str) -> Option<i64>;
}
