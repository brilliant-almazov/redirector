pub mod consumer;
pub mod dispatcher;
pub mod publisher;
pub mod queue_rabbitmq;
mod snowflake;
pub mod storage_postgres;
pub mod traits;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

pub use snowflake::{create_generator as create_snowflake_generator, SnowflakeGenerator};

use crate::config::EventsConfig;

/// Create an event queue backend based on configuration.
pub async fn create_queue(config: &EventsConfig) -> Box<dyn traits::EventQueue> {
    Box::new(queue_rabbitmq::RabbitMqQueue::new(config).await)
}

/// Create an event storage backend based on configuration.
pub async fn create_storage(
    config: &EventsConfig,
    geoip_path: Option<&str>,
) -> anyhow::Result<Box<dyn traits::EventStorage>> {
    Ok(Box::new(
        storage_postgres::PostgresEventStorage::new(config, geoip_path).await?,
    ))
}

/// A redirect event sent to the event queue.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RedirectEvent {
    /// Decoded numeric URL ID (from hashid)
    pub url_id: i64,
    /// The resolved target URL
    pub target_url: String,
    /// Event timestamp (UTC)
    pub timestamp: NaiveDateTime,
    /// Request processing latency in microseconds
    pub latency_micros: u64,
    /// Where the URL was resolved from
    pub source: DataSource,
    /// HTTP Referer header
    pub referer: Option<String>,
    /// HTTP User-Agent header
    pub user_agent: Option<String>,
    /// Client IP address
    pub ip: Option<String>,
}

/// Source of the URL resolution.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DataSource {
    Cache,
    Database,
}

impl DataSource {
    /// Convert to smallint for PostgreSQL storage.
    pub fn as_i16(self) -> i16 {
        match self {
            DataSource::Cache => 0,
            DataSource::Database => 1,
        }
    }

    pub fn from_i16(val: i16) -> Self {
        match val {
            0 => DataSource::Cache,
            _ => DataSource::Database,
        }
    }
}

/// A batch of events sent as a single queue message.
/// Uses internally-tagged enum for type-safe event type discrimination.
/// Each variant carries its own event structure.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "event_type")]
#[serde(rename_all = "snake_case")]
pub enum EventBatch {
    Redirect {
        events: Vec<RedirectEvent>,
        /// Snowflake batch ID
        batch_id: i64,
        /// When the batch was produced (UTC)
        produced_at: NaiveDateTime,
    },
}

impl EventBatch {
    pub fn event_count(&self) -> usize {
        match self {
            EventBatch::Redirect { events, .. } => events.len(),
        }
    }

    pub fn batch_id(&self) -> i64 {
        match self {
            EventBatch::Redirect { batch_id, .. } => *batch_id,
        }
    }
}

#[cfg(test)]
#[path = "mod_test.rs"]
mod mod_test;
