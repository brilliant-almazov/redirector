use super::EventDispatcher;
use crate::events::publisher::EventSender;
use crate::events::{DataSource, RedirectEvent};
use tokio::sync::mpsc;

fn make_test_event() -> RedirectEvent {
    RedirectEvent {
        url_id: 42,
        target_url: "https://example.com".to_string(),
        timestamp: chrono::Utc::now().naive_utc(),
        latency_micros: 100,
        source: DataSource::Cache,
        referer: None,
        user_agent: None,
        ip: None,
    }
}

#[tokio::test]
async fn test_dispatcher_new_sends_events() {
    let (tx, mut rx) = mpsc::channel(10);
    let sender = EventSender::new_for_test(tx);
    let dispatcher = EventDispatcher::new(sender);

    dispatcher.dispatch_redirect(make_test_event());

    let received = rx.recv().await.unwrap();
    assert_eq!(received.url_id, 42);
}

#[tokio::test]
async fn test_dispatcher_noop_discards_events() {
    let dispatcher = EventDispatcher::noop();

    // Should not panic
    dispatcher.dispatch_redirect(make_test_event());
    dispatcher.dispatch_redirect(make_test_event());
    dispatcher.dispatch_redirect(make_test_event());
}

#[tokio::test]
async fn test_dispatcher_clone() {
    let (tx, mut rx) = mpsc::channel(10);
    let sender = EventSender::new_for_test(tx);
    let dispatcher = EventDispatcher::new(sender);
    let dispatcher2 = dispatcher.clone();

    dispatcher.dispatch_redirect(make_test_event());
    dispatcher2.dispatch_redirect(make_test_event());

    let _ = rx.recv().await.unwrap();
    let _ = rx.recv().await.unwrap();
}

#[tokio::test]
async fn test_dispatcher_with_closed_channel() {
    let (tx, rx) = mpsc::channel(10);
    let sender = EventSender::new_for_test(tx);
    let dispatcher = EventDispatcher::new(sender);

    drop(rx);

    // Should not panic, just silently fail
    dispatcher.dispatch_redirect(make_test_event());
}
