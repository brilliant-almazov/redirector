use axum::{
    extract::Extension, middleware as axum_middleware, response::Redirect, routing::get, Router,
};
use metrics_exporter_prometheus::PrometheusBuilder;
use redirector::{
    admin::{admin_routes, AdminState},
    config::Config,
    db::MainStorage,
    handlers::{index_handler, metrics_handler, redirect_handler, RedirectState},
    metrics as service_metrics,
    middleware::{
        basic_auth::basic_auth_middleware, rate_limit::rate_limit_middleware, BasicAuthLayer,
        RateLimitLayer,
    },
    services::{CacheService, HashidService, UrlResolver},
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::{compression::CompressionLayer, services::ServeDir, trace::TraceLayer};
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

    // Load configuration: CONFIG_BASE64 takes priority over CONFIG_PATH
    let config = if let Ok(encoded) = std::env::var("CONFIG_BASE64") {
        tracing::info!("Loading configuration from CONFIG_BASE64");
        Config::load_from_base64(&encoded)?
    } else {
        let config_path =
            std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config.yaml".to_string());
        tracing::info!(config_path = %config_path, "Loading configuration from file");
        Config::load(&config_path)?
    };

    // Initialize Prometheus metrics
    let prometheus_handle = PrometheusBuilder::new()
        .install_recorder()
        .expect("Failed to install Prometheus recorder");

    // Initialize service metrics
    service_metrics::init();

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

    // Build admin router if enabled
    let admin_router = if config.admin.enabled {
        let admin_state =
            AdminState::new(config.admin.session_ttl_hours, config.admin.users.clone());
        tracing::info!("Admin dashboard enabled at /admin");
        Some(admin_routes(admin_state))
    } else {
        None
    };

    // Build main router
    let mut app = Router::new()
        .route("/", get(index_handler))
        .route("/r/{hashid}", get(redirect_handler))
        .route("/d/{hashid}", get(redirect_handler))
        .merge(metrics_router)
        .with_state(redirect_state)
        .layer(axum_middleware::from_fn(rate_limit_middleware))
        .layer(Extension(rate_limiter))
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http());

    // Add admin routes if enabled
    if let Some(admin) = admin_router {
        app = app
            .nest("/admin", admin)
            .route("/admin/", get(|| async { Redirect::permanent("/admin") }))
            .nest_service("/static", ServeDir::new("static"));
    }

    // Start server
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = TcpListener::bind(&addr).await?;

    tracing::info!(address = %addr, "Server starting");

    axum::serve(listener, app).await?;

    Ok(())
}
