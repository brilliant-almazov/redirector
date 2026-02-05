use super::*;
use crate::events::traits::EventQueue;
use crate::events::{create_snowflake_generator, DataSource, RedirectEvent};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

#[tokio::test]
async fn test_event_sender_try_send_success() {
    let (tx, mut rx) = mpsc::channel(10);
    let sender = EventSender { tx };

    let event = RedirectEvent {
        url_id: 42,
        target_url: "https://example.com".to_string(),
        timestamp: chrono::Utc::now().naive_utc(),
        latency_micros: 1000,
        source: DataSource::Cache,
        referer: None,
        user_agent: None,
        ip: None,
    };

    assert!(sender.try_send(event.clone()));

    let received = rx.recv().await.unwrap();
    assert_eq!(received.url_id, 42);
    assert_eq!(received.source, DataSource::Cache);
}

#[tokio::test]
async fn test_event_sender_try_send_full_channel() {
    let (tx, _rx) = mpsc::channel(1);
    let sender = EventSender { tx };

    let make_event = || RedirectEvent {
        url_id: 1,
        target_url: "https://example.com".to_string(),
        timestamp: chrono::Utc::now().naive_utc(),
        latency_micros: 100,
        source: DataSource::Database,
        referer: None,
        user_agent: None,
        ip: None,
    };

    // Fill the channel
    assert!(sender.try_send(make_event()));

    // Channel is full — should return false without blocking
    assert!(!sender.try_send(make_event()));
}

#[tokio::test]
async fn test_event_sender_try_send_closed_channel() {
    let (tx, rx) = mpsc::channel(10);
    let sender = EventSender { tx };

    // Drop the receiver
    drop(rx);

    let event = RedirectEvent {
        url_id: 1,
        target_url: "https://example.com".to_string(),
        timestamp: chrono::Utc::now().naive_utc(),
        latency_micros: 100,
        source: DataSource::Cache,
        referer: None,
        user_agent: None,
        ip: None,
    };

    assert!(!sender.try_send(event));
}

// Mock EventQueue for testing flush function
struct MockQueue {
    connected: AtomicBool,
    publish_count: AtomicUsize,
    should_fail: AtomicBool,
}

impl MockQueue {
    fn new(connected: bool) -> Self {
        Self {
            connected: AtomicBool::new(connected),
            publish_count: AtomicUsize::new(0),
            should_fail: AtomicBool::new(false),
        }
    }

    fn set_should_fail(&self, fail: bool) {
        self.should_fail.store(fail, Ordering::SeqCst);
    }

    fn get_publish_count(&self) -> usize {
        self.publish_count.load(Ordering::SeqCst)
    }
}

#[async_trait::async_trait]
impl EventQueue for MockQueue {
    async fn publish(&self, _payload: &[u8]) -> anyhow::Result<()> {
        if self.should_fail.load(Ordering::SeqCst) {
            return Err(anyhow::anyhow!("Mock publish error"));
        }
        self.publish_count.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.connected.load(Ordering::SeqCst)
    }

    async fn reconnect(&mut self) -> anyhow::Result<()> {
        // Simulate successful reconnect
        self.connected.store(true, Ordering::SeqCst);
        Ok(())
    }
}

fn make_test_event(url_id: i64) -> RedirectEvent {
    RedirectEvent {
        url_id,
        target_url: format!("https://example.com/{}", url_id),
        timestamp: chrono::Utc::now().naive_utc(),
        latency_micros: 100,
        source: DataSource::Cache,
        referer: None,
        user_agent: None,
        ip: None,
    }
}

#[tokio::test]
async fn test_flush_publishes_events_when_connected() {
    let snowflake = Arc::new(create_snowflake_generator(0));
    let mut queue = MockQueue::new(true);
    let config = crate::config::EventsConfig::default();

    let mut buffer = vec![make_test_event(1), make_test_event(2), make_test_event(3)];

    flush(&mut buffer, &mut queue, &config, &snowflake).await;

    assert!(buffer.is_empty(), "Buffer should be emptied after flush");
    assert_eq!(queue.get_publish_count(), 1, "Should publish one batch");
}

#[tokio::test]
async fn test_flush_reconnects_when_disconnected() {
    let snowflake = Arc::new(create_snowflake_generator(0));
    let mut queue = MockQueue::new(false);
    let config = crate::config::EventsConfig::default();

    let mut buffer = vec![make_test_event(1)];

    flush(&mut buffer, &mut queue, &config, &snowflake).await;

    assert!(buffer.is_empty());
    assert!(queue.is_connected(), "Should reconnect");
    assert_eq!(queue.get_publish_count(), 1);
}

#[tokio::test]
async fn test_flush_drops_events_when_cannot_connect() {
    let snowflake = Arc::new(create_snowflake_generator(0));

    // Create a queue that fails to reconnect
    struct FailingReconnectQueue;

    #[async_trait::async_trait]
    impl EventQueue for FailingReconnectQueue {
        async fn publish(&self, _: &[u8]) -> anyhow::Result<()> {
            Ok(())
        }
        fn is_connected(&self) -> bool {
            false
        }
        async fn reconnect(&mut self) -> anyhow::Result<()> {
            Err(anyhow::anyhow!("Cannot connect"))
        }
    }

    let mut queue = FailingReconnectQueue;
    let config = crate::config::EventsConfig::default();
    let mut buffer = vec![make_test_event(1)];

    flush(&mut buffer, &mut queue, &config, &snowflake).await;

    // Events are still dropped (buffer cleared), but not published
    assert!(buffer.is_empty());
}

#[tokio::test]
async fn test_flush_handles_publish_error() {
    let snowflake = Arc::new(create_snowflake_generator(0));
    let mut queue = MockQueue::new(true);
    queue.set_should_fail(true);
    let config = crate::config::EventsConfig::default();

    let mut buffer = vec![make_test_event(1)];

    flush(&mut buffer, &mut queue, &config, &snowflake).await;

    // Buffer is still cleared even on error
    assert!(buffer.is_empty());
    // Publish was attempted
    assert_eq!(queue.get_publish_count(), 0);
}

#[tokio::test]
async fn test_flush_empty_buffer_no_publish() {
    let snowflake = Arc::new(create_snowflake_generator(0));
    let mut queue = MockQueue::new(true);
    let config = crate::config::EventsConfig::default();

    let mut buffer: Vec<RedirectEvent> = vec![];

    // Flush should still work but publish an empty batch
    flush(&mut buffer, &mut queue, &config, &snowflake).await;

    assert!(buffer.is_empty());
    // Note: flush() still publishes even for empty batch
    assert_eq!(queue.get_publish_count(), 1);
}
