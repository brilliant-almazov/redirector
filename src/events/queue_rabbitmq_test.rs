use crate::config::{ConsumerConfig, EventsConfig, PublisherConfig, RabbitMqConnectionConfig};
use crate::events::queue_rabbitmq::RabbitMqQueue;
use crate::events::traits::EventQueue;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::rabbitmq::RabbitMq;

fn test_config(port: u16) -> EventsConfig {
    EventsConfig {
        enabled: true,
        rabbitmq: RabbitMqConnectionConfig {
            url: format!("amqp://guest:guest@localhost:{}", port),
            queue: "test_events".to_string(),
        },
        publisher: PublisherConfig::default(),
        consumer: ConsumerConfig::default(),
    }
}

#[tokio::test]
async fn test_rabbitmq_connect_and_publish() {
    let rabbit = RabbitMq::default()
        .start()
        .await
        .expect("Failed to start RabbitMQ");
    let port = rabbit.get_host_port_ipv4(5672).await.unwrap();

    let config = test_config(port);
    let queue = RabbitMqQueue::new(&config).await;

    assert!(queue.is_connected(), "Should be connected to RabbitMQ");

    let payload = br#"{"test": "data"}"#;
    let result = queue.publish(payload).await;
    assert!(result.is_ok(), "Publish should succeed");
}

#[tokio::test]
async fn test_rabbitmq_reconnect() {
    let rabbit = RabbitMq::default()
        .start()
        .await
        .expect("Failed to start RabbitMQ");
    let port = rabbit.get_host_port_ipv4(5672).await.unwrap();

    let config = test_config(port);
    let mut queue = RabbitMqQueue::new(&config).await;

    assert!(queue.is_connected());

    // Reconnect should succeed
    let result = queue.reconnect().await;
    assert!(result.is_ok(), "Reconnect should succeed");
    assert!(queue.is_connected());
}

#[tokio::test]
async fn test_rabbitmq_publish_multiple() {
    let rabbit = RabbitMq::default()
        .start()
        .await
        .expect("Failed to start RabbitMQ");
    let port = rabbit.get_host_port_ipv4(5672).await.unwrap();

    let config = test_config(port);
    let queue = RabbitMqQueue::new(&config).await;

    for i in 0..10 {
        let payload = format!(r#"{{"id": {}}}"#, i);
        let result = queue.publish(payload.as_bytes()).await;
        assert!(result.is_ok(), "Publish {} should succeed", i);
    }
}

// Unit tests that don't require RabbitMQ
#[tokio::test]
async fn test_rabbitmq_not_connected_returns_error() {
    let config = test_config(59999); // Port that doesn't exist
    let queue = RabbitMqQueue::new(&config).await;

    assert!(!queue.is_connected(), "Should not be connected");

    let result = queue.publish(b"test").await;
    assert!(result.is_err(), "Publish should fail when not connected");
}

#[tokio::test]
async fn test_rabbitmq_reconnect_fails_without_server() {
    let config = test_config(59999);
    let mut queue = RabbitMqQueue::new(&config).await;

    let result = queue.reconnect().await;
    assert!(result.is_err(), "Reconnect should fail without server");
}
