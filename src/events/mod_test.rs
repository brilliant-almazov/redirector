use super::*;

#[test]
fn test_redirect_event_serialization_roundtrip() {
    let event = RedirectEvent {
        url_id: 42,
        target_url: "https://example.com".to_string(),
        timestamp: chrono::NaiveDate::from_ymd_opt(2026, 1, 15)
            .unwrap()
            .and_hms_opt(12, 30, 0)
            .unwrap(),
        latency_micros: 1500,
        source: DataSource::Cache,
        referer: Some("https://example.com".to_string()),
        user_agent: Some("Mozilla/5.0".to_string()),
        ip: Some("192.168.1.1".to_string()),
    };

    let json = serde_json::to_string(&event).unwrap();
    let deserialized: RedirectEvent = serde_json::from_str(&json).unwrap();
    assert_eq!(event, deserialized);
}

#[test]
fn test_event_with_none_fields() {
    let event = RedirectEvent {
        url_id: 1,
        target_url: "https://example.com".to_string(),
        timestamp: chrono::NaiveDate::from_ymd_opt(2026, 2, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
        latency_micros: 500,
        source: DataSource::Database,
        referer: None,
        user_agent: None,
        ip: None,
    };

    let json = serde_json::to_string(&event).unwrap();
    assert!(json.contains("\"referer\":null"));
    assert!(json.contains("\"user_agent\":null"));

    let deserialized: RedirectEvent = serde_json::from_str(&json).unwrap();
    assert_eq!(event, deserialized);
}

#[test]
fn test_data_source_serde_rename() {
    let json = serde_json::to_string(&DataSource::Cache).unwrap();
    assert_eq!(json, "\"cache\"");

    let json = serde_json::to_string(&DataSource::Database).unwrap();
    assert_eq!(json, "\"database\"");
}

#[test]
fn test_data_source_as_i16() {
    assert_eq!(DataSource::Cache.as_i16(), 0);
    assert_eq!(DataSource::Database.as_i16(), 1);
}

#[test]
fn test_data_source_from_i16() {
    assert_eq!(DataSource::from_i16(0), DataSource::Cache);
    assert_eq!(DataSource::from_i16(1), DataSource::Database);
    assert_eq!(DataSource::from_i16(99), DataSource::Database);
}

#[test]
fn test_event_batch_serialization() {
    let batch = EventBatch::Redirect {
        events: vec![
            RedirectEvent {
                url_id: 1,
                target_url: "https://example.com".to_string(),
                timestamp: chrono::NaiveDate::from_ymd_opt(2026, 1, 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                latency_micros: 100,
                source: DataSource::Cache,
                referer: None,
                user_agent: None,
                ip: None,
            },
            RedirectEvent {
                url_id: 2,
                target_url: "https://example.com".to_string(),
                timestamp: chrono::NaiveDate::from_ymd_opt(2026, 1, 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 1)
                    .unwrap(),
                latency_micros: 200,
                source: DataSource::Database,
                referer: Some("https://test.com".to_string()),
                user_agent: Some("curl/8.0".to_string()),
                ip: Some("10.0.0.1".to_string()),
            },
        ],
        batch_id: 123456789,
        produced_at: chrono::NaiveDate::from_ymd_opt(2026, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 2)
            .unwrap(),
    };

    let json = serde_json::to_string(&batch).unwrap();
    assert!(json.contains("\"event_type\":\"redirect\""));

    let deserialized: EventBatch = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.event_count(), 2);
    assert_eq!(deserialized.batch_id(), 123456789);
}
