use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

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

        (status, self.to_string()).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
