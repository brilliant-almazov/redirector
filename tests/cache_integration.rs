use redirector::config::RedisConfig;
use redirector::services::{Cache, CacheService};
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::redis::Redis;

fn test_config(port: u16) -> RedisConfig {
    RedisConfig {
        url: format!("redis://localhost:{}", port),
        cache_ttl_seconds: 60,
    }
}

#[tokio::test]
async fn test_cache_set_and_get() {
    let redis = Redis::default()
        .start()
        .await
        .expect("Failed to start Redis");

    let port = redis.get_host_port_ipv4(6379).await.unwrap();
    let config = test_config(port);

    let cache = CacheService::new(&config).expect("Failed to create cache");

    // Set a value
    let set_result = cache.set(1, "https://example.com/page1").await;
    assert!(set_result.is_ok());

    // Get the value back
    let get_result = cache.get(1).await;
    assert_eq!(get_result, Some("https://example.com/page1".to_string()));
}

#[tokio::test]
async fn test_cache_get_nonexistent() {
    let redis = Redis::default()
        .start()
        .await
        .expect("Failed to start Redis");

    let port = redis.get_host_port_ipv4(6379).await.unwrap();
    let config = test_config(port);

    let cache = CacheService::new(&config).expect("Failed to create cache");

    // Get nonexistent key
    let result = cache.get(99999).await;
    assert_eq!(result, None);
}

#[tokio::test]
async fn test_cache_overwrite() {
    let redis = Redis::default()
        .start()
        .await
        .expect("Failed to start Redis");

    let port = redis.get_host_port_ipv4(6379).await.unwrap();
    let config = test_config(port);

    let cache = CacheService::new(&config).expect("Failed to create cache");

    // Set initial value
    cache.set(1, "https://old.com").await.unwrap();
    assert_eq!(cache.get(1).await, Some("https://old.com".to_string()));

    // Overwrite
    cache.set(1, "https://new.com").await.unwrap();
    assert_eq!(cache.get(1).await, Some("https://new.com".to_string()));
}

#[tokio::test]
async fn test_cache_multiple_keys() {
    let redis = Redis::default()
        .start()
        .await
        .expect("Failed to start Redis");

    let port = redis.get_host_port_ipv4(6379).await.unwrap();
    let config = test_config(port);

    let cache = CacheService::new(&config).expect("Failed to create cache");

    // Set multiple keys
    for id in 1..=10 {
        cache
            .set(id, &format!("https://example.com/{}", id))
            .await
            .unwrap();
    }

    // Verify all keys
    for id in 1..=10 {
        let result = cache.get(id).await;
        assert_eq!(result, Some(format!("https://example.com/{}", id)));
    }
}

#[tokio::test]
async fn test_cache_large_url() {
    let redis = Redis::default()
        .start()
        .await
        .expect("Failed to start Redis");

    let port = redis.get_host_port_ipv4(6379).await.unwrap();
    let config = test_config(port);

    let cache = CacheService::new(&config).expect("Failed to create cache");

    // Create a large URL (4KB)
    let large_url = format!("https://example.com/{}", "a".repeat(4000));

    cache.set(1, &large_url).await.unwrap();
    let result = cache.get(1).await;
    assert_eq!(result, Some(large_url));
}

#[tokio::test]
async fn test_cache_special_characters() {
    let redis = Redis::default()
        .start()
        .await
        .expect("Failed to start Redis");

    let port = redis.get_host_port_ipv4(6379).await.unwrap();
    let config = test_config(port);

    let cache = CacheService::new(&config).expect("Failed to create cache");

    let url_with_special = "https://example.com/path?query=value&foo=bar#section";
    cache.set(1, url_with_special).await.unwrap();

    let result = cache.get(1).await;
    assert_eq!(result, Some(url_with_special.to_string()));
}

#[tokio::test]
async fn test_cache_unicode_url() {
    let redis = Redis::default()
        .start()
        .await
        .expect("Failed to start Redis");

    let port = redis.get_host_port_ipv4(6379).await.unwrap();
    let config = test_config(port);

    let cache = CacheService::new(&config).expect("Failed to create cache");

    let unicode_url = "https://example.com/путь/到/ページ";
    cache.set(1, unicode_url).await.unwrap();

    let result = cache.get(1).await;
    assert_eq!(result, Some(unicode_url.to_string()));
}
