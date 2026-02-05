use async_trait::async_trait;

use super::EventBatch;

/// Trait for event queue backends (RabbitMQ, Kafka, Redis Streams, etc.).
#[async_trait]
pub trait EventQueue: Send + Sync {
    /// Publish a serialized payload to the configured queue/topic.
    async fn publish(&self, payload: &[u8]) -> anyhow::Result<()>;

    /// Check if the connection is alive.
    fn is_connected(&self) -> bool;

    /// Reconnect to the queue backend.
    async fn reconnect(&mut self) -> anyhow::Result<()>;
}

/// Trait for event storage backends (PostgreSQL, ClickHouse, MySQL, SQLite, etc.).
#[async_trait]
pub trait EventStorage: Send + Sync {
    /// Run migrations / ensure schema exists.
    async fn ensure_schema(&self) -> anyhow::Result<()>;

    /// Insert a batch of events. Returns count of inserted rows.
    async fn insert_batch(&self, batch: &EventBatch) -> anyhow::Result<usize>;
}
