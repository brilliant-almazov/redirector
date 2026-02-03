use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use thiserror::Error;

#[cfg(not(debug_assertions))]
use std::sync::LazyLock;

#[cfg(not(debug_assertions))]
static NOT_FOUND_HTML: &str = include_str!("../templates/not_found.html");

#[cfg(not(debug_assertions))]
static NOT_FOUND_MINIFIED: LazyLock<String> = LazyLock::new(|| {
    use minify_html::{minify, Cfg};
    let cfg = Cfg::spec_compliant();
    String::from_utf8(minify(NOT_FOUND_HTML.as_bytes(), &cfg))
        .unwrap_or_else(|_| NOT_FOUND_HTML.to_string())
});

#[derive(Error, Debug)]
pub enum AppError {
    #[error("URL not found")]
    NotFound,

    #[error("Invalid hashid")]
    InvalidHashid,

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("Service unavailable")]
    ServiceUnavailable,

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

fn not_found_page() -> Html<String> {
    #[cfg(debug_assertions)]
    {
        match std::fs::read_to_string("templates/not_found.html") {
            Ok(content) => Html(content),
            Err(e) => Html(format!(
                "<h1>404 - Not Found</h1><p>Error loading template: {}</p>",
                e
            )),
        }
    }
    #[cfg(not(debug_assertions))]
    {
        Html(NOT_FOUND_MINIFIED.clone())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::NotFound | AppError::InvalidHashid => StatusCode::NOT_FOUND,
            AppError::RateLimitExceeded => StatusCode::TOO_MANY_REQUESTS,
            AppError::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
            AppError::Database(_) | AppError::Redis(_) | AppError::Internal(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        tracing::error!(error = %self, status = %status, "Request error");

        match &self {
            AppError::NotFound | AppError::InvalidHashid => {
                (status, not_found_page()).into_response()
            }
            _ => (status, self.to_string()).into_response(),
        }
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
