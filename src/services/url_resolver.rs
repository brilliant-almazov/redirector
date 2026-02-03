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
        let id = self
            .hashid_decoder
            .decode(hashid)
            .ok_or(AppError::InvalidHashid)?;

        tracing::debug!(hashid = %hashid, id = %id, "Decoded hashid");

        if let Some(url) = self.cache.get(id).await {
            tracing::debug!(id = %id, "Cache hit");
            metrics::counter!("cache_hits").increment(1);
            return Ok(ResolvedUrl::new(url));
        }

        tracing::debug!(id = %id, "Cache miss, querying storage");
        metrics::counter!("cache_misses").increment(1);

        let url = self
            .storage
            .get_url_by_id(id)
            .await?
            .ok_or(AppError::NotFound)?;

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

pub(crate) fn extract_domain(url: &str) -> Option<String> {
    let url = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))?;
    let domain = url.split('/').next()?;
    Some(domain.to_string())
}

#[cfg(test)]
#[path = "url_resolver_test.rs"]
mod url_resolver_test;
