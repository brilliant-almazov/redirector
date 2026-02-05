use redirector::config::EventsConfig;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,event_consumer=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    let config = config_from_env();
    let geoip_path = std::env::var("GEOIP_DB_PATH").ok();

    tracing::info!(
        database_url = %mask_url(&config.consumer.database_url),
        rabbitmq_url = %mask_url(&config.rabbitmq.url),
        queue = %config.rabbitmq.queue,
        prefetch_count = config.consumer.prefetch_count,
        geoip_db = geoip_path.as_deref().unwrap_or("(disabled)"),
        "Starting event consumer"
    );

    let storage = redirector::events::create_storage(&config, geoip_path.as_deref()).await?;
    storage.ensure_schema().await?;

    redirector::events::consumer::run(&config, &*storage).await
}

fn config_from_env() -> EventsConfig {
    EventsConfig {
        enabled: true,
        rabbitmq: redirector::config::RabbitMqConnectionConfig {
            url: std::env::var("RABBITMQ_URL")
                .unwrap_or_else(|_| "amqp://guest:guest@localhost:5672/%2f".to_string()),
            queue: std::env::var("QUEUE").unwrap_or_else(|_| "events.analytics".to_string()),
        },
        publisher: Default::default(),
        consumer: redirector::config::ConsumerConfig {
            prefetch_count: std::env::var("PREFETCH_COUNT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(10),
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://localhost/redirector_analytics".to_string()),
        },
    }
}

/// Mask password in URL for safe logging.
fn mask_url(url: &str) -> String {
    if let Some(at) = url.find('@') {
        if let Some(scheme_end) = url.find("://") {
            let prefix = &url[..scheme_end + 3];
            let suffix = &url[at..];
            return format!("{prefix}***{suffix}");
        }
    }
    url.to_string()
}
