use futures::StreamExt;
use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties};

use super::traits::EventStorage;
use super::EventBatch;
use crate::config::EventsConfig;

/// Result of processing a message.
#[derive(Debug, PartialEq)]
pub enum ProcessResult {
    /// Message processed successfully, should be acked.
    Success(usize),
    /// Storage error, should be nacked with requeue.
    StorageError,
    /// Invalid message format, should be nacked without requeue (poison message).
    InvalidMessage,
}

/// Process a raw message payload.
/// Returns ProcessResult indicating what action to take.
pub(crate) async fn process_message(data: &[u8], storage: &dyn EventStorage) -> ProcessResult {
    match serde_json::from_slice::<EventBatch>(data) {
        Ok(batch) => match storage.insert_batch(&batch).await {
            Ok(count) => {
                tracing::debug!(
                    batch_id = batch.batch_id(),
                    count = count,
                    "Inserted event batch"
                );
                ProcessResult::Success(count)
            }
            Err(e) => {
                tracing::error!(
                    error = %e,
                    batch_id = batch.batch_id(),
                    "Failed to insert batch"
                );
                ProcessResult::StorageError
            }
        },
        Err(e) => {
            tracing::error!(error = %e, "Failed to deserialize event batch");
            ProcessResult::InvalidMessage
        }
    }
}

/// Run the event consumer loop.
///
/// Connects to RabbitMQ, consumes messages, deserializes EventBatch,
/// and delegates to the provided EventStorage implementation.
pub async fn run(config: &EventsConfig, storage: &dyn EventStorage) -> anyhow::Result<()> {
    let conn = Connection::connect(
        &config.rabbitmq.url,
        ConnectionProperties::default()
            .with_executor(tokio_executor_trait::Tokio::current())
            .with_reactor(tokio_reactor_trait::Tokio),
    )
    .await?;

    let channel = conn.create_channel().await?;

    // Declare queue (idempotent, must match publisher)
    channel
        .queue_declare(
            &config.rabbitmq.queue,
            QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    channel
        .basic_qos(config.consumer.prefetch_count, BasicQosOptions::default())
        .await?;

    tracing::info!(queue = %config.rabbitmq.queue, "Starting consumer");

    let mut consumer = channel
        .basic_consume(
            &config.rabbitmq.queue,
            "event_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    while let Some(delivery) = consumer.next().await {
        match delivery {
            Ok(delivery) => {
                let result = process_message(&delivery.data, storage).await;
                handle_result(&delivery, result).await;
            }
            Err(e) => {
                tracing::error!(error = %e, "Consumer delivery error");
            }
        }
    }

    Ok(())
}

async fn handle_result(delivery: &lapin::message::Delivery, result: ProcessResult) {
    match result {
        ProcessResult::Success(_) => {
            delivery
                .ack(BasicAckOptions::default())
                .await
                .unwrap_or_else(|e| tracing::error!(error = %e, "Failed to ack"));
        }
        ProcessResult::StorageError => {
            delivery
                .nack(BasicNackOptions {
                    requeue: true,
                    ..Default::default()
                })
                .await
                .unwrap_or_else(|e| tracing::error!(error = %e, "Failed to nack"));
        }
        ProcessResult::InvalidMessage => {
            delivery
                .nack(BasicNackOptions {
                    requeue: false,
                    ..Default::default()
                })
                .await
                .unwrap_or_else(|e| tracing::error!(error = %e, "Failed to nack"));
        }
    }
}

#[cfg(test)]
#[path = "consumer_test.rs"]
mod consumer_test;
