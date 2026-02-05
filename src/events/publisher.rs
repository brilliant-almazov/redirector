use crate::config::EventsConfig;
use crate::events::traits::EventQueue;
use crate::events::{create_snowflake_generator, EventBatch, RedirectEvent, SnowflakeGenerator};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;

/// Handle for sending events from the redirect handler.
/// Cheaply cloneable (wraps mpsc::Sender).
#[derive(Clone)]
pub struct EventSender {
    tx: mpsc::Sender<RedirectEvent>,
}

impl EventSender {
    /// Create an EventSender for testing with a custom channel.
    #[cfg(test)]
    pub(crate) fn new_for_test(tx: mpsc::Sender<RedirectEvent>) -> Self {
        Self { tx }
    }

    /// Try to send an event without blocking.
    /// Returns false if the channel is full or closed (event dropped).
    pub fn try_send(&self, event: RedirectEvent) -> bool {
        match self.tx.try_send(event) {
            Ok(()) => true,
            Err(mpsc::error::TrySendError::Full(_)) => {
                tracing::warn!("Event channel full, dropping event");
                metrics::counter!("events_dropped").increment(1);
                false
            }
            Err(mpsc::error::TrySendError::Closed(_)) => {
                tracing::error!("Event channel closed");
                metrics::counter!("events_dropped").increment(1);
                false
            }
        }
    }
}

/// Start the event publisher background task.
/// Returns an EventSender that can be cloned into handler state.
pub fn start_publisher(config: &EventsConfig) -> EventSender {
    let (tx, rx) = mpsc::channel(config.publisher.channel_buffer_size);

    let config = config.clone();
    tokio::spawn(async move {
        publisher_loop(rx, &config).await;
    });

    EventSender { tx }
}

async fn publisher_loop(mut rx: mpsc::Receiver<RedirectEvent>, config: &EventsConfig) {
    let snowflake = Arc::new(create_snowflake_generator(0));
    let mut buffer: Vec<RedirectEvent> = Vec::with_capacity(config.publisher.batch_size);
    let mut flush_interval =
        tokio::time::interval(Duration::from_millis(config.publisher.flush_interval_ms));

    let mut queue = crate::events::create_queue(config).await;

    loop {
        tokio::select! {
            event = rx.recv() => {
                match event {
                    Some(evt) => {
                        buffer.push(evt);
                        if buffer.len() >= config.publisher.batch_size {
                            flush(&mut buffer, &mut *queue, config, &snowflake).await;
                        }
                    }
                    None => {
                        // Channel closed, flush remaining and exit
                        if !buffer.is_empty() {
                            flush(&mut buffer, &mut *queue, config, &snowflake).await;
                        }
                        tracing::info!("Event publisher shutting down");
                        return;
                    }
                }
            }
            _ = flush_interval.tick() => {
                if !buffer.is_empty() {
                    flush(&mut buffer, &mut *queue, config, &snowflake).await;
                }
            }
        }
    }
}

async fn flush(
    buffer: &mut Vec<RedirectEvent>,
    queue: &mut dyn EventQueue,
    _config: &EventsConfig,
    snowflake: &SnowflakeGenerator,
) {
    let batch = EventBatch::Redirect {
        events: std::mem::take(buffer),
        batch_id: snowflake.generate(),
        produced_at: chrono::Utc::now().naive_utc(),
    };

    let event_count = batch.event_count() as u64;

    let payload = match serde_json::to_vec(&batch) {
        Ok(p) => p,
        Err(e) => {
            tracing::error!(error = %e, "Failed to serialize event batch");
            metrics::counter!("events_serialize_errors").increment(1);
            return;
        }
    };

    // Ensure connection
    if !queue.is_connected() {
        if let Err(e) = queue.reconnect().await {
            tracing::warn!(error = %e, "Failed to reconnect to event queue");
        }
    }

    if queue.is_connected() {
        match queue.publish(&payload).await {
            Ok(()) => {
                metrics::counter!("events_published").increment(event_count);
                tracing::debug!(batch_size = event_count, "Event batch published");
            }
            Err(e) => {
                tracing::warn!(error = %e, "Failed to publish event batch");
                metrics::counter!("events_publish_errors").increment(1);
                // Force reconnect on next flush
                let _ = queue.reconnect().await;
            }
        }
    } else {
        tracing::warn!(
            dropped_count = event_count,
            "No event queue connection, dropping event batch"
        );
        metrics::counter!("events_dropped").increment(event_count);
    }
}

#[cfg(test)]
#[path = "publisher_test.rs"]
mod publisher_test;
