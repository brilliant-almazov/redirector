use async_trait::async_trait;
use lapin::{
    options::*, types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties,
};

use super::traits::EventQueue;
use crate::config::EventsConfig;

/// RabbitMQ implementation of EventQueue.
pub struct RabbitMqQueue {
    channel: Option<Channel>,
    config: EventsConfig,
}

impl RabbitMqQueue {
    /// Create a new RabbitMqQueue and attempt initial connection.
    pub async fn new(config: &EventsConfig) -> Self {
        let channel = try_connect(config).await;
        Self {
            channel,
            config: config.clone(),
        }
    }
}

#[async_trait]
impl EventQueue for RabbitMqQueue {
    async fn publish(&self, payload: &[u8]) -> anyhow::Result<()> {
        let ch = self
            .channel
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No RabbitMQ connection"))?;

        let confirm = ch
            .basic_publish(
                "",
                &self.config.rabbitmq.queue,
                BasicPublishOptions::default(),
                payload,
                BasicProperties::default()
                    .with_content_type("application/json".into())
                    .with_delivery_mode(2), // persistent
            )
            .await?;

        confirm.await?;
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.channel.is_some()
    }

    async fn reconnect(&mut self) -> anyhow::Result<()> {
        self.channel = try_connect(&self.config).await;
        if self.channel.is_some() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to reconnect to RabbitMQ"))
        }
    }
}

#[cfg(test)]
#[path = "queue_rabbitmq_test.rs"]
mod queue_rabbitmq_test;

async fn try_connect(config: &EventsConfig) -> Option<Channel> {
    match Connection::connect(
        &config.rabbitmq.url,
        ConnectionProperties::default()
            .with_executor(tokio_executor_trait::Tokio::current())
            .with_reactor(tokio_reactor_trait::Tokio),
    )
    .await
    {
        Ok(conn) => match conn.create_channel().await {
            Ok(ch) => {
                if let Err(e) = ch
                    .queue_declare(
                        &config.rabbitmq.queue,
                        QueueDeclareOptions {
                            durable: true,
                            ..Default::default()
                        },
                        FieldTable::default(),
                    )
                    .await
                {
                    tracing::error!(error = %e, "Failed to declare queue");
                    return None;
                }
                tracing::info!(queue = %config.rabbitmq.queue, "Connected to RabbitMQ");
                metrics::gauge!("rabbitmq_connected").set(1.0);
                Some(ch)
            }
            Err(e) => {
                tracing::error!(error = %e, "Failed to create RabbitMQ channel");
                None
            }
        },
        Err(e) => {
            tracing::warn!(error = %e, "Failed to connect to RabbitMQ");
            metrics::gauge!("rabbitmq_connected").set(0.0);
            None
        }
    }
}
