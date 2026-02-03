use crate::error::{AppError, Result};
use crate::services::traits::{Cache, HashidDecoder, UrlStorage};
use std::sync::Arc;

pub struct UrlResolver<H, C, S>
where
    H: HashidDecoder,
    C: Cache,
    S: UrlStorage,
{
    hashid_decoder: Arc<H>,
    cache: Arc<C>,
    storage: Arc<S>,
}

impl<H, C, S> UrlResolver<H, C, S>
where
    H: HashidDecoder,
    C: Cache,
    S: UrlStorage,
{
    pub fn new(hashid_decoder: Arc<H>, cache: Arc<C>, storage: Arc<S>) -> Self {
        Self {
            hashid_decoder,
            cache,
            storage,
        }
    }

    pub async fn resolve(&self, hashid: &str) -> Result<ResolvedUrl> {
        // 1. Decode hashid
        let id = self
            .hashid_decoder
            .decode(hashid)
            .ok_or(AppError::InvalidHashid)?;

        tracing::debug!(hashid = %hashid, id = %id, "Decoded hashid");

        // 2. Check cache
        if let Some(url) = self.cache.get(id).await {
            tracing::debug!(id = %id, "Cache hit");
            metrics::counter!("cache_hits").increment(1);
            return Ok(ResolvedUrl::new(url));
        }

        tracing::debug!(id = %id, "Cache miss, querying storage");
        metrics::counter!("cache_misses").increment(1);

        // 3. Query storage
        let url = self
            .storage
            .get_url_by_id(id)
            .await?
            .ok_or(AppError::NotFound)?;

        // 4. Cache result
        if let Err(e) = self.cache.set(id, &url).await {
            tracing::warn!(error = %e, "Failed to cache URL");
        }

        Ok(ResolvedUrl::new(url))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedUrl {
    pub full_url: String,
    pub domain: String,
}

impl ResolvedUrl {
    pub fn new(full_url: String) -> Self {
        let domain = extract_domain(&full_url).unwrap_or_else(|| full_url.clone());
        Self { full_url, domain }
    }
}

fn extract_domain(url: &str) -> Option<String> {
    // Simple domain extraction
    let url = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))?;
    let domain = url.split('/').next()?;
    Some(domain.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::AppError;
    use async_trait::async_trait;
    use std::collections::HashMap;
    use std::sync::Mutex;

    // Mock implementations for testing

    struct MockHashidDecoder {
        mappings: HashMap<String, i64>,
    }

    impl MockHashidDecoder {
        fn new(mappings: Vec<(&str, i64)>) -> Self {
            Self {
                mappings: mappings
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), v))
                    .collect(),
            }
        }
    }

    impl HashidDecoder for MockHashidDecoder {
        fn decode(&self, hashid: &str) -> Option<i64> {
            self.mappings.get(hashid).copied()
        }
    }

    struct MockCache {
        data: Mutex<HashMap<i64, String>>,
    }

    impl MockCache {
        fn new() -> Self {
            Self {
                data: Mutex::new(HashMap::new()),
            }
        }

        fn with_data(data: Vec<(i64, &str)>) -> Self {
            let map = data.into_iter().map(|(k, v)| (k, v.to_string())).collect();
            Self {
                data: Mutex::new(map),
            }
        }
    }

    #[async_trait]
    impl Cache for MockCache {
        async fn get(&self, id: i64) -> Option<String> {
            self.data.lock().unwrap().get(&id).cloned()
        }

        async fn set(&self, id: i64, url: &str) -> anyhow::Result<()> {
            self.data.lock().unwrap().insert(id, url.to_string());
            Ok(())
        }
    }

    struct MockStorage {
        data: HashMap<i64, String>,
    }

    impl MockStorage {
        fn new(data: Vec<(i64, &str)>) -> Self {
            Self {
                data: data.into_iter().map(|(k, v)| (k, v.to_string())).collect(),
            }
        }

        fn empty() -> Self {
            Self {
                data: HashMap::new(),
            }
        }
    }

    #[async_trait]
    impl UrlStorage for MockStorage {
        async fn get_url_by_id(&self, id: i64) -> crate::error::Result<Option<String>> {
            Ok(self.data.get(&id).cloned())
        }
    }

    struct FailingStorage;

    #[async_trait]
    impl UrlStorage for FailingStorage {
        async fn get_url_by_id(&self, _id: i64) -> crate::error::Result<Option<String>> {
            Err(AppError::ServiceUnavailable)
        }
    }

    #[test]
    fn test_extract_domain_https() {
        assert_eq!(
            extract_domain("https://example.com/path/to/page"),
            Some("example.com".to_string())
        );
    }

    #[test]
    fn test_extract_domain_http() {
        assert_eq!(
            extract_domain("http://example.com/path"),
            Some("example.com".to_string())
        );
    }

    #[test]
    fn test_extract_domain_with_port() {
        assert_eq!(
            extract_domain("https://example.com:8080/path"),
            Some("example.com:8080".to_string())
        );
    }

    #[test]
    fn test_extract_domain_no_protocol() {
        assert_eq!(extract_domain("example.com/path"), None);
    }

    #[test]
    fn test_extract_domain_root_only() {
        assert_eq!(
            extract_domain("https://example.com"),
            Some("example.com".to_string())
        );
    }

    #[test]
    fn test_resolved_url_new() {
        let resolved = ResolvedUrl::new("https://example.com/path".to_string());
        assert_eq!(resolved.full_url, "https://example.com/path");
        assert_eq!(resolved.domain, "example.com");
    }

    #[test]
    fn test_resolved_url_no_protocol() {
        let resolved = ResolvedUrl::new("example.com/path".to_string());
        assert_eq!(resolved.full_url, "example.com/path");
        assert_eq!(resolved.domain, "example.com/path"); // fallback to full URL
    }

    #[tokio::test]
    async fn test_resolve_cache_hit() {
        let decoder = Arc::new(MockHashidDecoder::new(vec![("abc123", 42)]));
        let cache = Arc::new(MockCache::with_data(vec![(
            42,
            "https://example.com/cached",
        )]));
        let storage = Arc::new(MockStorage::empty());

        let resolver = UrlResolver::new(decoder, cache, storage);
        let result = resolver.resolve("abc123").await.unwrap();

        assert_eq!(result.full_url, "https://example.com/cached");
        assert_eq!(result.domain, "example.com");
    }

    #[tokio::test]
    async fn test_resolve_cache_miss_storage_hit() {
        let decoder = Arc::new(MockHashidDecoder::new(vec![("abc123", 42)]));
        let cache = Arc::new(MockCache::new());
        let storage = Arc::new(MockStorage::new(vec![(42, "https://example.com/from-db")]));

        let resolver = UrlResolver::new(decoder, cache.clone(), storage);
        let result = resolver.resolve("abc123").await.unwrap();

        assert_eq!(result.full_url, "https://example.com/from-db");
        assert_eq!(result.domain, "example.com");

        // Verify it was cached
        assert_eq!(
            cache.get(42).await,
            Some("https://example.com/from-db".to_string())
        );
    }

    #[tokio::test]
    async fn test_resolve_invalid_hashid() {
        let decoder = Arc::new(MockHashidDecoder::new(vec![])); // no mappings
        let cache = Arc::new(MockCache::new());
        let storage = Arc::new(MockStorage::empty());

        let resolver = UrlResolver::new(decoder, cache, storage);
        let result = resolver.resolve("invalid").await;

        assert!(matches!(result, Err(AppError::InvalidHashid)));
    }

    #[tokio::test]
    async fn test_resolve_not_found() {
        let decoder = Arc::new(MockHashidDecoder::new(vec![("abc123", 42)]));
        let cache = Arc::new(MockCache::new());
        let storage = Arc::new(MockStorage::empty()); // no data

        let resolver = UrlResolver::new(decoder, cache, storage);
        let result = resolver.resolve("abc123").await;

        assert!(matches!(result, Err(AppError::NotFound)));
    }

    #[tokio::test]
    async fn test_resolve_storage_error() {
        let decoder = Arc::new(MockHashidDecoder::new(vec![("abc123", 42)]));
        let cache = Arc::new(MockCache::new());
        let storage = Arc::new(FailingStorage);

        let resolver = UrlResolver::new(decoder, cache, storage);
        let result = resolver.resolve("abc123").await;

        assert!(matches!(result, Err(AppError::ServiceUnavailable)));
    }
}
