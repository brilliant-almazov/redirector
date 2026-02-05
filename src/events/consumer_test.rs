use super::{process_message, ProcessResult};
use crate::events::traits::EventStorage;
use crate::events::{DataSource, EventBatch, RedirectEvent};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

// Mock EventStorage for testing
struct MockStorage {
    insert_count: AtomicUsize,
    should_fail: AtomicBool,
}

impl MockStorage {
    fn new() -> Self {
        Self {
            insert_count: AtomicUsize::new(0),
            should_fail: AtomicBool::new(false),
        }
    }

    fn failing() -> Self {
        Self {
            insert_count: AtomicUsize::new(0),
            should_fail: AtomicBool::new(true),
        }
    }

    fn get_insert_count(&self) -> usize {
        self.insert_count.load(Ordering::SeqCst)
    }
}

#[async_trait::async_trait]
impl EventStorage for MockStorage {
    async fn ensure_schema(&self) -> anyhow::Result<()> {
        Ok(())
    }

    async fn insert_batch(&self, batch: &EventBatch) -> anyhow::Result<usize> {
        if self.should_fail.load(Ordering::SeqCst) {
            return Err(anyhow::anyhow!("Mock storage error"));
        }

        let count = batch.event_count();
        self.insert_count.fetch_add(count, Ordering::SeqCst);
        Ok(count)
    }
}

fn make_test_batch(batch_id: i64, event_count: usize) -> EventBatch {
    let events: Vec<RedirectEvent> = (0..event_count)
        .map(|i| RedirectEvent {
            url_id: i as i64,
            target_url: format!("https://example.com/{}", i),
            timestamp: chrono::Utc::now().naive_utc(),
            latency_micros: 100,
            source: DataSource::Cache,
            referer: None,
            user_agent: None,
            ip: None,
        })
        .collect();

    EventBatch::Redirect {
        events,
        batch_id,
        produced_at: chrono::Utc::now().naive_utc(),
    }
}

#[tokio::test]
async fn test_process_message_success() {
    let storage = MockStorage::new();
    let batch = make_test_batch(12345, 5);
    let payload = serde_json::to_vec(&batch).unwrap();

    let result = process_message(&payload, &storage).await;

    assert_eq!(result, ProcessResult::Success(5));
    assert_eq!(storage.get_insert_count(), 5);
}

#[tokio::test]
async fn test_process_message_empty_batch() {
    let storage = MockStorage::new();
    let batch = make_test_batch(12345, 0);
    let payload = serde_json::to_vec(&batch).unwrap();

    let result = process_message(&payload, &storage).await;

    assert_eq!(result, ProcessResult::Success(0));
    assert_eq!(storage.get_insert_count(), 0);
}

#[tokio::test]
async fn test_process_message_storage_error() {
    let storage = MockStorage::failing();
    let batch = make_test_batch(12345, 3);
    let payload = serde_json::to_vec(&batch).unwrap();

    let result = process_message(&payload, &storage).await;

    assert_eq!(result, ProcessResult::StorageError);
    assert_eq!(storage.get_insert_count(), 0);
}

#[tokio::test]
async fn test_process_message_invalid_json() {
    let storage = MockStorage::new();
    let invalid_payload = b"not valid json at all {{{";

    let result = process_message(invalid_payload, &storage).await;

    assert_eq!(result, ProcessResult::InvalidMessage);
    assert_eq!(storage.get_insert_count(), 0);
}

#[tokio::test]
async fn test_process_message_wrong_schema() {
    let storage = MockStorage::new();
    // Valid JSON but wrong schema
    let payload = br#"{"wrong": "schema"}"#;

    let result = process_message(payload, &storage).await;

    assert_eq!(result, ProcessResult::InvalidMessage);
}

#[tokio::test]
async fn test_process_message_large_batch() {
    let storage = MockStorage::new();
    let batch = make_test_batch(99999, 100);
    let payload = serde_json::to_vec(&batch).unwrap();

    let result = process_message(&payload, &storage).await;

    assert_eq!(result, ProcessResult::Success(100));
    assert_eq!(storage.get_insert_count(), 100);
}

#[tokio::test]
async fn test_process_result_enum() {
    // Test enum variants exist and are debuggable
    let success = ProcessResult::Success(10);
    let storage_err = ProcessResult::StorageError;
    let invalid = ProcessResult::InvalidMessage;

    assert_eq!(format!("{:?}", success), "Success(10)");
    assert_eq!(format!("{:?}", storage_err), "StorageError");
    assert_eq!(format!("{:?}", invalid), "InvalidMessage");

    // Test PartialEq
    assert_eq!(ProcessResult::Success(5), ProcessResult::Success(5));
    assert_ne!(ProcessResult::Success(5), ProcessResult::Success(10));
    assert_ne!(ProcessResult::Success(5), ProcessResult::StorageError);
}
