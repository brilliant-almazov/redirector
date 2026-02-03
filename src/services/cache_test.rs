#[cfg(test)]
mod tests {
    use crate::services::cache::CacheService;

    #[test]
    fn test_cache_key_format() {
        assert_eq!(CacheService::key(42), "url:42");
        assert_eq!(CacheService::key(0), "url:0");
        assert_eq!(CacheService::key(999999), "url:999999");
        assert_eq!(CacheService::key(-1), "url:-1");
    }
}
