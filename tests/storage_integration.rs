use redirector::config::{
    CircuitBreakerConfig, DatabaseConfig, DbRateLimitConfig, PoolConfig, QueryConfig,
};
use redirector::db::MainStorage;
use redirector::services::UrlStorage;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres;

const INIT_SQL: &str = r#"
CREATE SCHEMA IF NOT EXISTS dictionary;

CREATE TABLE dictionary.urls (
    id BIGINT PRIMARY KEY,
    name VARCHAR(4096) NOT NULL
);

INSERT INTO dictionary.urls (id, name) VALUES
    (1, 'https://example.com/page1'),
    (2, 'https://example.com/page2'),
    (42, 'https://test.com/answer'),
    (12345, 'https://long-id.com/path');
"#;

fn test_config(port: u16) -> DatabaseConfig {
    DatabaseConfig {
        url: format!("postgresql://postgres:postgres@localhost:{}/postgres", port),
        pool: PoolConfig {
            max_connections: 2,
            connect_timeout_seconds: 5,
        },
        rate_limit: DbRateLimitConfig {
            max_requests_per_second: 100,
        },
        circuit_breaker: CircuitBreakerConfig {
            failure_threshold: 3,
            reset_timeout_seconds: 10,
        },
        query: QueryConfig {
            table: "dictionary.urls".to_string(),
            id_column: "id".to_string(),
            url_column: "name".to_string(),
        },
    }
}

#[tokio::test]
async fn test_storage_get_existing_url() {
    let postgres = Postgres::default()
        .with_init_sql(INIT_SQL.as_bytes().to_vec())
        .start()
        .await
        .expect("Failed to start Postgres");

    let port = postgres.get_host_port_ipv4(5432).await.unwrap();
    let config = test_config(port);

    let storage = MainStorage::new(&config).await.expect("Failed to create storage");

    let result = storage.get_url_by_id(1).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some("https://example.com/page1".to_string()));
}

#[tokio::test]
async fn test_storage_get_another_url() {
    let postgres = Postgres::default()
        .with_init_sql(INIT_SQL.as_bytes().to_vec())
        .start()
        .await
        .expect("Failed to start Postgres");

    let port = postgres.get_host_port_ipv4(5432).await.unwrap();
    let config = test_config(port);

    let storage = MainStorage::new(&config).await.expect("Failed to create storage");

    let result = storage.get_url_by_id(42).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some("https://test.com/answer".to_string()));
}

#[tokio::test]
async fn test_storage_get_nonexistent_url() {
    let postgres = Postgres::default()
        .with_init_sql(INIT_SQL.as_bytes().to_vec())
        .start()
        .await
        .expect("Failed to start Postgres");

    let port = postgres.get_host_port_ipv4(5432).await.unwrap();
    let config = test_config(port);

    let storage = MainStorage::new(&config).await.expect("Failed to create storage");

    let result = storage.get_url_by_id(99999).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
}

#[tokio::test]
async fn test_storage_health_check() {
    let postgres = Postgres::default()
        .with_init_sql(INIT_SQL.as_bytes().to_vec())
        .start()
        .await
        .expect("Failed to start Postgres");

    let port = postgres.get_host_port_ipv4(5432).await.unwrap();
    let config = test_config(port);

    let storage = MainStorage::new(&config).await.expect("Failed to create storage");

    let result = storage.health_check().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_storage_multiple_queries() {
    let postgres = Postgres::default()
        .with_init_sql(INIT_SQL.as_bytes().to_vec())
        .start()
        .await
        .expect("Failed to start Postgres");

    let port = postgres.get_host_port_ipv4(5432).await.unwrap();
    let config = test_config(port);

    let storage = MainStorage::new(&config).await.expect("Failed to create storage");

    // Query multiple IDs
    for id in [1, 2, 42, 12345] {
        let result = storage.get_url_by_id(id).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }
}

#[tokio::test]
async fn test_storage_custom_table_config() {
    // Test with custom table/column names
    let init_sql = r#"
        CREATE TABLE custom_links (
            link_id BIGINT PRIMARY KEY,
            target_url VARCHAR(4096) NOT NULL
        );
        INSERT INTO custom_links (link_id, target_url) VALUES (100, 'https://custom.com/url');
    "#;

    let postgres = Postgres::default()
        .with_init_sql(init_sql.as_bytes().to_vec())
        .start()
        .await
        .expect("Failed to start Postgres");

    let port = postgres.get_host_port_ipv4(5432).await.unwrap();

    let config = DatabaseConfig {
        url: format!("postgresql://postgres:postgres@localhost:{}/postgres", port),
        pool: PoolConfig::default(),
        rate_limit: DbRateLimitConfig::default(),
        circuit_breaker: CircuitBreakerConfig::default(),
        query: QueryConfig {
            table: "custom_links".to_string(),
            id_column: "link_id".to_string(),
            url_column: "target_url".to_string(),
        },
    };

    let storage = MainStorage::new(&config).await.expect("Failed to create storage");

    let result = storage.get_url_by_id(100).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some("https://custom.com/url".to_string()));
}
