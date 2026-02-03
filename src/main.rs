use axum::{extract::Extension, middleware as axum_middleware, routing::get, Router};
use metrics_exporter_prometheus::PrometheusBuilder;
use redirector::{
    config::Config,
    db::MainStorage,
    handlers::{index_handler, metrics_handler, redirect_handler, RedirectState},
    middleware::{
        basic_auth::basic_auth_middleware, rate_limit::rate_limit_middleware, BasicAuthLayer,
        RateLimitLayer,
    },
    services::{CacheService, HashidService, UrlResolver},
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file if exists
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,redirector=debug".into()))
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    // Load configuration
    let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config.yaml".to_string());
    let config = Config::load(&config_path)?;

    tracing::info!(config_path = %config_path, "Configuration loaded");

    // Initialize Prometheus metrics
    let prometheus_handle = PrometheusBuilder::new()
        .install_recorder()
        .expect("Failed to install Prometheus recorder");

    // Initialize services
    let hashid_service = Arc::new(HashidService::new(&config.hashids));
    let cache_service = Arc::new(CacheService::new(&config.redis)?);
    let main_storage = Arc::new(MainStorage::new(&config.database).await?);

    let url_resolver = Arc::new(UrlResolver::new(
        hashid_service,
        cache_service,
        main_storage,
    ));

    let redirect_state = Arc::new(RedirectState {
        resolver: url_resolver,
        delay_seconds: config.interstitial.delay_seconds,
    });

    // Build rate limiter
    let rate_limiter = RateLimitLayer::new(
        config.rate_limit.requests_per_second,
        config.rate_limit.burst,
    );

    // Build basic auth for metrics
    let metrics_auth = BasicAuthLayer::new(
        config.metrics.basic_auth.username.clone(),
        config.metrics.basic_auth.password.clone(),
    );

    // Metrics route with basic auth
    let metrics_router = Router::new()
        .route(
            "/metrics",
            get({
                let handle = prometheus_handle.clone();
                move || async move { metrics_handler(handle).await }
            }),
        )
        .layer(axum_middleware::from_fn(basic_auth_middleware))
        .layer(Extension(metrics_auth));

    // Build main router
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/{hashid}", get(redirect_handler))
        .merge(metrics_router)
        .with_state(redirect_state)
        .layer(axum_middleware::from_fn(rate_limit_middleware))
        .layer(Extension(rate_limiter))
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http());

    // Start server
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = TcpListener::bind(&addr).await?;

    tracing::info!(address = %addr, "Server starting");

    axum::serve(listener, app).await?;

    Ok(())
}
