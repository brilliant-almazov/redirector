use chrono::NaiveDateTime;
use redirector::config::{ConsumerConfig, EventsConfig, PublisherConfig, RabbitMqConnectionConfig};
use redirector::events::storage_postgres::PostgresEventStorage;
use redirector::events::traits::EventStorage;
use redirector::events::{DataSource, EventBatch, RedirectEvent};
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres;

fn test_config(port: u16) -> EventsConfig {
    EventsConfig {
        enabled: true,
        rabbitmq: RabbitMqConnectionConfig::default(),
        publisher: PublisherConfig::default(),
        consumer: ConsumerConfig {
            prefetch_count: 10,
            database_url: format!("postgresql://postgres:postgres@localhost:{}/postgres", port),
        },
    }
}

fn make_event(
    url_id: i64,
    referer: Option<&str>,
    user_agent: Option<&str>,
    ip: Option<&str>,
) -> RedirectEvent {
    RedirectEvent {
        url_id,
        target_url: format!("https://example.com/{}", url_id),
        timestamp: NaiveDateTime::parse_from_str("2026-02-04 12:00:00", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        latency_micros: 150,
        source: DataSource::Cache,
        referer: referer.map(|s| s.to_string()),
        user_agent: user_agent.map(|s| s.to_string()),
        ip: ip.map(|s| s.to_string()),
    }
}

fn make_batch(events: Vec<RedirectEvent>) -> EventBatch {
    EventBatch::Redirect {
        events,
        batch_id: 1001,
        produced_at: NaiveDateTime::parse_from_str("2026-02-04 12:00:01", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
    }
}

async fn setup_storage(port: u16) -> PostgresEventStorage {
    let config = test_config(port);
    let storage = PostgresEventStorage::new(&config, None)
        .await
        .expect("Failed to create storage");
    storage
        .ensure_schema()
        .await
        .expect("Failed to run migrations");
    storage
}

// ── Schema & Migrations ─────────────────────────────────────────────

#[tokio::test]
async fn test_ensure_schema_creates_tables() {
    let postgres = Postgres::default()
        .start()
        .await
        .expect("Failed to start Postgres");
    let port = postgres.get_host_port_ipv4(5432).await.unwrap();

    let storage = setup_storage(port).await;

    // Running ensure_schema twice should be idempotent
    storage
        .ensure_schema()
        .await
        .expect("Second ensure_schema failed");
}

// ── insert_batch — basic insert ─────────────────────────────────────

#[tokio::test]
async fn test_insert_batch_single_event_no_enrichment() {
    let postgres = Postgres::default()
        .start()
        .await
        .expect("Failed to start Postgres");
    let port = postgres.get_host_port_ipv4(5432).await.unwrap();
    let storage = setup_storage(port).await;

    let batch = make_batch(vec![make_event(42, None, None, None)]);
    let count = storage
        .insert_batch(&batch)
        .await
        .expect("insert_batch failed");
    assert_eq!(count, 1);
}

#[tokio::test]
async fn test_insert_batch_multiple_events() {
    let postgres = Postgres::default()
        .start()
        .await
        .expect("Failed to start Postgres");
    let port = postgres.get_host_port_ipv4(5432).await.unwrap();
    let storage = setup_storage(port).await;

    let events = vec![
        make_event(1, None, None, None),
        make_event(2, None, None, None),
        make_event(3, None, None, None),
    ];
    let batch = make_batch(events);
    let count = storage
        .insert_batch(&batch)
        .await
        .expect("insert_batch failed");
    assert_eq!(count, 3);
}

// ── Referer resolution ──────────────────────────────────────────────

#[tokio::test]
async fn test_insert_with_referer_creates_reference() {
    let postgres = Postgres::default()
        .start()
        .await
        .expect("Failed to start Postgres");
    let port = postgres.get_host_port_ipv4(5432).await.unwrap();
    let storage = setup_storage(port).await;

    let batch = make_batch(vec![make_event(
        10,
        Some("https://www.google.com/search?q=rust"),
        None,
        None,
    )]);
    let count = storage
        .insert_batch(&batch)
        .await
        .expect("insert_batch failed");
    assert_eq!(count, 1);

    // Verify referer was stored in referers table
    let config = test_config(port);
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&config.consumer.database_url)
        .await
        .unwrap();

    let row: (i64, String) =
        sqlx::query_as("SELECT id, value FROM referers WHERE id > 1 ORDER BY id LIMIT 1")
            .fetch_one(&pool)
            .await
            .expect("referer not found");
    assert!(row.0 > 1);
    assert_eq!(row.1, "https://www.google.com/search?q=rust");
}

// ── Referer domain normalization ────────────────────────────────────

#[tokio::test]
async fn test_insert_with_referer_creates_normalized_domain() {
    let postgres = Postgres::default()
        .start()
        .await
        .expect("Failed to start Postgres");
    let port = postgres.get_host_port_ipv4(5432).await.unwrap();
    let storage = setup_storage(port).await;

    let batch = make_batch(vec![make_event(
        11,
        Some("https://WWW.Example.COM/path?q=1"),
        None,
        None,
    )]);
    storage
        .insert_batch(&batch)
        .await
        .expect("insert_batch failed");

    let config = test_config(port);
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&config.consumer.database_url)
        .await
        .unwrap();

    let row: (String,) =
        sqlx::query_as("SELECT domain FROM referer_domains WHERE domain != '(unknown)' LIMIT 1")
            .fetch_one(&pool)
            .await
            .expect("referer_domain not found");

    assert_eq!(row.0, "example.com");
}

#[tokio::test]
async fn test_duplicate_referer_domain_reuses_id() {
    let postgres = Postgres::default()
        .start()
        .await
        .expect("Failed to start Postgres");
    let port = postgres.get_host_port_ipv4(5432).await.unwrap();
    let storage = setup_storage(port).await;

    let events = vec![
        make_event(20, Some("https://example.com/a"), None, None),
        make_event(21, Some("https://example.com/b"), None, None),
        make_event(22, Some("https://www.example.com/c"), None, None),
    ];
    let batch = make_batch(events);
    storage
        .insert_batch(&batch)
        .await
        .expect("insert_batch failed");

    let config = test_config(port);
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&config.consumer.database_url)
        .await
        .unwrap();

    // All three should map to the same domain "example.com"
    let row: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM referer_domains WHERE domain = 'example.com'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(
        row.0, 1,
        "expected exactly one domain entry for example.com"
    );

    // All three redirect_events should have the same referer_domain_id
    let rows: Vec<(i64,)> = sqlx::query_as(
        "SELECT DISTINCT referer_domain_id FROM redirect_events WHERE url_id IN (20, 21, 22)",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(
        rows.len(),
        1,
        "all events should share the same referer_domain_id"
    );
}

// ── User-Agent parsing ──────────────────────────────────────────────

#[tokio::test]
async fn test_insert_with_user_agent_parses_fields() {
    let postgres = Postgres::default()
        .start()
        .await
        .expect("Failed to start Postgres");
    let port = postgres.get_host_port_ipv4(5432).await.unwrap();
    let storage = setup_storage(port).await;

    let ua = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";
    let batch = make_batch(vec![make_event(30, None, Some(ua), None)]);
    storage
        .insert_batch(&batch)
        .await
        .expect("insert_batch failed");

    let config = test_config(port);
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&config.consumer.database_url)
        .await
        .unwrap();

    let row: (
        String,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
    ) = sqlx::query_as(
        "SELECT value, browser, browser_version, os, device_type \
             FROM user_agents WHERE id > 1 ORDER BY id LIMIT 1",
    )
    .fetch_one(&pool)
    .await
    .expect("user_agent not found");

    assert_eq!(row.0, ua);
    // woothee should detect Chrome
    assert!(
        row.1.as_deref().unwrap_or("").contains("Chrome"),
        "expected browser to contain Chrome, got {:?}",
        row.1
    );
    assert!(row.2.is_some(), "browser_version should be set");
    assert!(row.3.is_some(), "os should be set");
    assert!(row.4.is_some(), "device_type should be set");
}

#[tokio::test]
async fn test_duplicate_user_agent_reuses_id() {
    let postgres = Postgres::default()
        .start()
        .await
        .expect("Failed to start Postgres");
    let port = postgres.get_host_port_ipv4(5432).await.unwrap();
    let storage = setup_storage(port).await;

    let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36";
    let events = vec![
        make_event(40, None, Some(ua), None),
        make_event(41, None, Some(ua), None),
    ];
    let batch = make_batch(events);
    storage
        .insert_batch(&batch)
        .await
        .expect("insert_batch failed");

    let config = test_config(port);
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&config.consumer.database_url)
        .await
        .unwrap();

    // The UA should only appear once in the user_agents table
    let hash = format!("{:x}", md5::compute(ua.as_bytes()));
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM user_agents WHERE hash = $1")
        .bind(&hash)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(row.0, 1, "duplicate UA should be deduplicated by hash");

    // Both events should share the same user_agent_id
    let rows: Vec<(i64,)> = sqlx::query_as(
        "SELECT DISTINCT user_agent_id FROM redirect_events WHERE url_id IN (40, 41)",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(
        rows.len(),
        1,
        "both events should share the same user_agent_id"
    );
}

// ── GeoIP — disabled (no .mmdb) ─────────────────────────────────────

#[tokio::test]
async fn test_insert_with_ip_but_no_geoip_uses_unknown() {
    let postgres = Postgres::default()
        .start()
        .await
        .expect("Failed to start Postgres");
    let port = postgres.get_host_port_ipv4(5432).await.unwrap();
    // No geoip_path → GeoIP disabled
    let storage = setup_storage(port).await;

    let batch = make_batch(vec![make_event(50, None, None, Some("8.8.8.8"))]);
    storage
        .insert_batch(&batch)
        .await
        .expect("insert_batch failed");

    let config = test_config(port);
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&config.consumer.database_url)
        .await
        .unwrap();

    // Without GeoIP DB, geo_location_id should be UNKNOWN (1)
    let row: (i64,) =
        sqlx::query_as("SELECT geo_location_id FROM redirect_events WHERE url_id = 50")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(
        row.0, 1,
        "geo_location_id should be UNKNOWN_REFERENCE_ID (1) when GeoIP is disabled"
    );
}

// ── UNKNOWN sentinel for missing fields ─────────────────────────────

#[tokio::test]
async fn test_missing_fields_use_unknown_sentinel() {
    let postgres = Postgres::default()
        .start()
        .await
        .expect("Failed to start Postgres");
    let port = postgres.get_host_port_ipv4(5432).await.unwrap();
    let storage = setup_storage(port).await;

    // Event with all optional fields as None
    let batch = make_batch(vec![make_event(60, None, None, None)]);
    storage
        .insert_batch(&batch)
        .await
        .expect("insert_batch failed");

    let config = test_config(port);
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&config.consumer.database_url)
        .await
        .unwrap();

    let row: (i64, i64, i64, i64) = sqlx::query_as(
        "SELECT referer_id, user_agent_id, referer_domain_id, geo_location_id \
         FROM redirect_events WHERE url_id = 60",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(row.0, 1, "referer_id should be UNKNOWN (1)");
    assert_eq!(row.1, 1, "user_agent_id should be UNKNOWN (1)");
    assert_eq!(row.2, 1, "referer_domain_id should be UNKNOWN (1)");
    assert_eq!(row.3, 1, "geo_location_id should be UNKNOWN (1)");
}

// ── Partitioning ────────────────────────────────────────────────────

#[tokio::test]
async fn test_events_across_months_create_partitions() {
    let postgres = Postgres::default()
        .start()
        .await
        .expect("Failed to start Postgres");
    let port = postgres.get_host_port_ipv4(5432).await.unwrap();
    let storage = setup_storage(port).await;

    let events = vec![
        RedirectEvent {
            url_id: 70,
            target_url: "https://example.com".to_string(),
            timestamp: NaiveDateTime::parse_from_str("2026-01-15 10:00:00", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
            latency_micros: 100,
            source: DataSource::Cache,
            referer: None,
            user_agent: None,
            ip: None,
        },
        RedirectEvent {
            url_id: 71,
            target_url: "https://example.com".to_string(),
            timestamp: NaiveDateTime::parse_from_str("2026-02-15 10:00:00", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
            latency_micros: 200,
            source: DataSource::Database,
            referer: None,
            user_agent: None,
            ip: None,
        },
        RedirectEvent {
            url_id: 72,
            target_url: "https://example.com".to_string(),
            timestamp: NaiveDateTime::parse_from_str("2026-03-15 10:00:00", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
            latency_micros: 300,
            source: DataSource::Cache,
            referer: None,
            user_agent: None,
            ip: None,
        },
    ];
    let batch = make_batch(events);
    let count = storage
        .insert_batch(&batch)
        .await
        .expect("insert_batch failed");
    assert_eq!(count, 3);

    let config = test_config(port);
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&config.consumer.database_url)
        .await
        .unwrap();

    // Verify partitions were created
    let partitions: Vec<(String,)> = sqlx::query_as(
        "SELECT tablename::text FROM pg_tables \
         WHERE tablename LIKE 'redirect_events_2026_%' ORDER BY tablename",
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    let names: Vec<&str> = partitions.iter().map(|r| r.0.as_str()).collect();
    assert!(
        names.contains(&"redirect_events_2026_01"),
        "January partition missing"
    );
    assert!(
        names.contains(&"redirect_events_2026_02"),
        "February partition missing"
    );
    assert!(
        names.contains(&"redirect_events_2026_03"),
        "March partition missing"
    );
}

#[tokio::test]
async fn test_partition_year_boundary_december_to_january() {
    let postgres = Postgres::default()
        .start()
        .await
        .expect("Failed to start Postgres");
    let port = postgres.get_host_port_ipv4(5432).await.unwrap();
    let storage = setup_storage(port).await;

    let events = vec![
        RedirectEvent {
            url_id: 200,
            target_url: "https://example.com".to_string(),
            timestamp: NaiveDateTime::parse_from_str("2025-12-31 23:59:59", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
            latency_micros: 100,
            source: DataSource::Cache,
            referer: None,
            user_agent: None,
            ip: None,
        },
        RedirectEvent {
            url_id: 201,
            target_url: "https://example.com".to_string(),
            timestamp: NaiveDateTime::parse_from_str("2026-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
            latency_micros: 100,
            source: DataSource::Cache,
            referer: None,
            user_agent: None,
            ip: None,
        },
    ];
    let batch = make_batch(events);
    let count = storage
        .insert_batch(&batch)
        .await
        .expect("insert_batch failed");
    assert_eq!(count, 2);

    let config = test_config(port);
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&config.consumer.database_url)
        .await
        .unwrap();

    // December 2025 partition should exist with correct range
    let dec_partition: Vec<(String,)> = sqlx::query_as(
        "SELECT tablename::text FROM pg_tables WHERE tablename = 'redirect_events_2025_12'",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(dec_partition.len(), 1, "December 2025 partition missing");

    // January 2026 partition should exist
    let jan_partition: Vec<(String,)> = sqlx::query_as(
        "SELECT tablename::text FROM pg_tables WHERE tablename = 'redirect_events_2026_01'",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(jan_partition.len(), 1, "January 2026 partition missing");

    // Verify events landed in correct partitions
    let dec_event: (i64,) =
        sqlx::query_as("SELECT url_id FROM redirect_events_2025_12 WHERE url_id = 200")
            .fetch_one(&pool)
            .await
            .expect("Dec event not in December partition");
    assert_eq!(dec_event.0, 200);

    let jan_event: (i64,) =
        sqlx::query_as("SELECT url_id FROM redirect_events_2026_01 WHERE url_id = 201")
            .fetch_one(&pool)
            .await
            .expect("Jan event not in January partition");
    assert_eq!(jan_event.0, 201);
}

#[tokio::test]
async fn test_partition_same_month_multiple_batches() {
    let postgres = Postgres::default()
        .start()
        .await
        .expect("Failed to start Postgres");
    let port = postgres.get_host_port_ipv4(5432).await.unwrap();
    let storage = setup_storage(port).await;

    // First batch — creates partition
    let batch1 = EventBatch::Redirect {
        events: vec![RedirectEvent {
            url_id: 210,
            target_url: "https://example.com".to_string(),
            timestamp: NaiveDateTime::parse_from_str("2026-06-10 12:00:00", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
            latency_micros: 100,
            source: DataSource::Cache,
            referer: None,
            user_agent: None,
            ip: None,
        }],
        batch_id: 3001,
        produced_at: NaiveDateTime::parse_from_str("2026-06-10 12:00:01", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
    };
    storage
        .insert_batch(&batch1)
        .await
        .expect("first batch failed");

    // Second batch — same month, should reuse partition (from cache)
    let batch2 = EventBatch::Redirect {
        events: vec![RedirectEvent {
            url_id: 211,
            target_url: "https://example.com".to_string(),
            timestamp: NaiveDateTime::parse_from_str("2026-06-20 18:00:00", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
            latency_micros: 200,
            source: DataSource::Database,
            referer: None,
            user_agent: None,
            ip: None,
        }],
        batch_id: 3002,
        produced_at: NaiveDateTime::parse_from_str("2026-06-20 18:00:01", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
    };
    storage
        .insert_batch(&batch2)
        .await
        .expect("second batch failed");

    let config = test_config(port);
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&config.consumer.database_url)
        .await
        .unwrap();

    // Only one June partition should exist
    let partitions: Vec<(String,)> = sqlx::query_as(
        "SELECT tablename::text FROM pg_tables WHERE tablename LIKE 'redirect_events_2026_06%'",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(partitions.len(), 1, "expected exactly one June partition");

    // Both events should be in the same partition
    let count: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM redirect_events_2026_06 WHERE url_id IN (210, 211)")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(count.0, 2);
}

#[tokio::test]
async fn test_partition_month_boundaries_first_and_last_day() {
    let postgres = Postgres::default()
        .start()
        .await
        .expect("Failed to start Postgres");
    let port = postgres.get_host_port_ipv4(5432).await.unwrap();
    let storage = setup_storage(port).await;

    let events = vec![
        // First moment of March
        RedirectEvent {
            url_id: 220,
            target_url: "https://example.com".to_string(),
            timestamp: NaiveDateTime::parse_from_str("2026-03-01 00:00:00", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
            latency_micros: 100,
            source: DataSource::Cache,
            referer: None,
            user_agent: None,
            ip: None,
        },
        // Last moment of March
        RedirectEvent {
            url_id: 221,
            target_url: "https://example.com".to_string(),
            timestamp: NaiveDateTime::parse_from_str("2026-03-31 23:59:59", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
            latency_micros: 100,
            source: DataSource::Cache,
            referer: None,
            user_agent: None,
            ip: None,
        },
        // First moment of April (should go to different partition)
        RedirectEvent {
            url_id: 222,
            target_url: "https://example.com".to_string(),
            timestamp: NaiveDateTime::parse_from_str("2026-04-01 00:00:00", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
            latency_micros: 100,
            source: DataSource::Cache,
            referer: None,
            user_agent: None,
            ip: None,
        },
    ];
    let batch = make_batch(events);
    storage
        .insert_batch(&batch)
        .await
        .expect("insert_batch failed");

    let config = test_config(port);
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&config.consumer.database_url)
        .await
        .unwrap();

    // Both March events in March partition
    let march_count: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM redirect_events_2026_03 WHERE url_id IN (220, 221)")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(march_count.0, 2, "both events should be in March partition");

    // April event in April partition
    let april_event: (i64,) =
        sqlx::query_as("SELECT url_id FROM redirect_events_2026_04 WHERE url_id = 222")
            .fetch_one(&pool)
            .await
            .expect("April event not in April partition");
    assert_eq!(april_event.0, 222);
}

#[tokio::test]
async fn test_partition_december_range_correct() {
    let postgres = Postgres::default()
        .start()
        .await
        .expect("Failed to start Postgres");
    let port = postgres.get_host_port_ipv4(5432).await.unwrap();
    let storage = setup_storage(port).await;

    // Insert event in December — this tests the if month == 12 branch
    let batch = make_batch(vec![RedirectEvent {
        url_id: 230,
        target_url: "https://example.com".to_string(),
        timestamp: NaiveDateTime::parse_from_str("2026-12-15 12:00:00", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        latency_micros: 100,
        source: DataSource::Cache,
        referer: None,
        user_agent: None,
        ip: None,
    }]);
    storage
        .insert_batch(&batch)
        .await
        .expect("insert_batch failed");

    let config = test_config(port);
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&config.consumer.database_url)
        .await
        .unwrap();

    // December partition should exist and contain the event
    let row: (i64,) =
        sqlx::query_as("SELECT url_id FROM redirect_events_2026_12 WHERE url_id = 230")
            .fetch_one(&pool)
            .await
            .expect("December event not found in partition");
    assert_eq!(row.0, 230);

    // Verify partition range is correct (FROM 2026-12-01 TO 2027-01-01)
    // by inserting a December 31st event — it should land in the same partition
    let batch2 = EventBatch::Redirect {
        events: vec![RedirectEvent {
            url_id: 231,
            target_url: "https://example.com".to_string(),
            timestamp: NaiveDateTime::parse_from_str("2026-12-31 23:59:59", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
            latency_micros: 100,
            source: DataSource::Cache,
            referer: None,
            user_agent: None,
            ip: None,
        }],
        batch_id: 4001,
        produced_at: NaiveDateTime::parse_from_str("2026-12-31 23:59:59", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
    };
    storage
        .insert_batch(&batch2)
        .await
        .expect("Dec 31 insert failed");

    let count: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM redirect_events_2026_12 WHERE url_id IN (230, 231)")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(
        count.0, 2,
        "both December events should be in same partition"
    );
}

// ── Data source enum ────────────────────────────────────────────────

#[tokio::test]
async fn test_data_source_stored_correctly() {
    let postgres = Postgres::default()
        .start()
        .await
        .expect("Failed to start Postgres");
    let port = postgres.get_host_port_ipv4(5432).await.unwrap();
    let storage = setup_storage(port).await;

    let events = vec![
        RedirectEvent {
            url_id: 80,
            target_url: "https://example.com".to_string(),
            timestamp: NaiveDateTime::parse_from_str("2026-02-04 12:00:00", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
            latency_micros: 100,
            source: DataSource::Cache,
            referer: None,
            user_agent: None,
            ip: None,
        },
        RedirectEvent {
            url_id: 81,
            target_url: "https://example.com".to_string(),
            timestamp: NaiveDateTime::parse_from_str("2026-02-04 12:00:01", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
            latency_micros: 5000,
            source: DataSource::Database,
            referer: None,
            user_agent: None,
            ip: None,
        },
    ];
    let batch = make_batch(events);
    storage
        .insert_batch(&batch)
        .await
        .expect("insert_batch failed");

    let config = test_config(port);
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&config.consumer.database_url)
        .await
        .unwrap();

    let row: (String,) =
        sqlx::query_as("SELECT source::text FROM redirect_events WHERE url_id = 80")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(row.0, "cache");

    let row: (String,) =
        sqlx::query_as("SELECT source::text FROM redirect_events WHERE url_id = 81")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(row.0, "database");
}

// ── Full enrichment flow ────────────────────────────────────────────

#[tokio::test]
async fn test_full_event_with_all_fields() {
    let postgres = Postgres::default()
        .start()
        .await
        .expect("Failed to start Postgres");
    let port = postgres.get_host_port_ipv4(5432).await.unwrap();
    let storage = setup_storage(port).await;

    let batch = make_batch(vec![RedirectEvent {
        url_id: 90,
        target_url: "https://example.com".to_string(),
        timestamp: NaiveDateTime::parse_from_str("2026-02-04 15:30:00", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        latency_micros: 250,
        source: DataSource::Database,
        referer: Some("https://news.ycombinator.com/item?id=123".to_string()),
        user_agent: Some(
            "Mozilla/5.0 (X11; Linux x86_64; rv:128.0) Gecko/20100101 Firefox/128.0".to_string(),
        ),
        ip: Some("1.2.3.4".to_string()),
    }]);

    let count = storage
        .insert_batch(&batch)
        .await
        .expect("insert_batch failed");
    assert_eq!(count, 1);

    let config = test_config(port);
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&config.consumer.database_url)
        .await
        .unwrap();

    // Check the event was inserted with all reference IDs populated
    let row: (i64, i64, i64, i64, i64, Option<String>, i64) = sqlx::query_as(
        "SELECT url_id, referer_id, user_agent_id, referer_domain_id, geo_location_id, host(ip), batch_id \
         FROM redirect_events WHERE url_id = 90",
    )
    .fetch_one(&pool)
    .await
    .expect("event not found");

    assert_eq!(row.0, 90);
    assert!(row.1 > 1, "referer_id should not be UNKNOWN");
    assert!(row.2 > 1, "user_agent_id should not be UNKNOWN");
    assert!(row.3 > 1, "referer_domain_id should not be UNKNOWN");
    // geo_location_id = 1 because no GeoIP DB loaded
    assert_eq!(
        row.4, 1,
        "geo_location_id should be UNKNOWN without GeoIP DB"
    );
    assert_eq!(row.5.as_deref(), Some("1.2.3.4"));
    assert_eq!(row.6, 1001);

    // Verify the referer domain was normalized
    let domain_row: (String,) = sqlx::query_as(
        "SELECT rd.domain FROM referer_domains rd \
         JOIN redirect_events re ON re.referer_domain_id = rd.id \
         WHERE re.url_id = 90",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(domain_row.0, "news.ycombinator.com");

    // Verify UA was parsed (Firefox on Linux)
    let ua_row: (Option<String>, Option<String>) = sqlx::query_as(
        "SELECT ua.browser, ua.os FROM user_agents ua \
         JOIN redirect_events re ON re.user_agent_id = ua.id \
         WHERE re.url_id = 90",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(
        ua_row.0.as_deref().unwrap_or("").contains("Firefox"),
        "expected Firefox, got {:?}",
        ua_row.0
    );
}

// ── Performance — insert timing ─────────────────────────────────────

#[tokio::test]
async fn test_insert_timing_single_and_batch() {
    let postgres = Postgres::default()
        .start()
        .await
        .expect("Failed to start Postgres");
    let port = postgres.get_host_port_ipv4(5432).await.unwrap();
    let storage = setup_storage(port).await;

    // Single event insert timing
    let single_event = make_batch(vec![RedirectEvent {
        url_id: 300,
        target_url: "https://example.com".to_string(),
        timestamp: NaiveDateTime::parse_from_str("2026-02-04 12:00:00", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        latency_micros: 100,
        source: DataSource::Cache,
        referer: Some("https://www.google.com/search?q=test".to_string()),
        user_agent: Some(
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) Chrome/120.0.0.0".to_string(),
        ),
        ip: Some("8.8.8.8".to_string()),
    }]);

    let start = std::time::Instant::now();
    storage
        .insert_batch(&single_event)
        .await
        .expect("single insert failed");
    let single_elapsed = start.elapsed();

    // Batch of 100 events (diverse data for realistic enrichment)
    let mut events = Vec::new();
    for i in 0..100u64 {
        events.push(RedirectEvent {
            url_id: 1000 + i as i64,
            target_url: format!("https://example.com/{}", 1000 + i),
            timestamp: NaiveDateTime::parse_from_str("2026-02-04 12:00:00", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
            latency_micros: 100 + i,
            source: if i % 2 == 0 {
                DataSource::Cache
            } else {
                DataSource::Database
            },
            referer: Some(format!("https://site{}.example.com/page/{}", i % 10, i)),
            user_agent: Some(format!(
                "Mozilla/5.0 (TestBot/{}.0) AppleWebKit/537.36",
                i % 5
            )),
            ip: Some(format!("10.0.{}.{}", i / 256, i % 256)),
        });
    }
    let batch100 = EventBatch::Redirect {
        events,
        batch_id: 5001,
        produced_at: NaiveDateTime::parse_from_str("2026-02-04 12:00:01", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
    };

    let start = std::time::Instant::now();
    let count = storage
        .insert_batch(&batch100)
        .await
        .expect("batch insert failed");
    let batch_elapsed = start.elapsed();

    assert_eq!(count, 100);

    eprintln!(
        "\n=== INSERT TIMING (testcontainers PostgreSQL) ===\n\
         Single event (with enrichment):  {:?}\n\
         Batch of 100 events:             {:?} ({:.1} µs/event)\n\
         ================================================",
        single_elapsed,
        batch_elapsed,
        batch_elapsed.as_micros() as f64 / 100.0,
    );
}

// ── Batch ID stored correctly ───────────────────────────────────────

#[tokio::test]
async fn test_batch_id_stored() {
    let postgres = Postgres::default()
        .start()
        .await
        .expect("Failed to start Postgres");
    let port = postgres.get_host_port_ipv4(5432).await.unwrap();
    let storage = setup_storage(port).await;

    let batch = EventBatch::Redirect {
        events: vec![make_event(100, None, None, None)],
        batch_id: 999_888_777,
        produced_at: NaiveDateTime::parse_from_str("2026-02-04 12:00:00", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
    };
    storage
        .insert_batch(&batch)
        .await
        .expect("insert_batch failed");

    let config = test_config(port);
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&config.consumer.database_url)
        .await
        .unwrap();

    let row: (i64,) = sqlx::query_as("SELECT batch_id FROM redirect_events WHERE url_id = 100")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(row.0, 999_888_777);
}
