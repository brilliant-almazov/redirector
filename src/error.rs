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
static NOT_FOUND_MINIFIED: LazyLock<String> =
    LazyLock::new(|| crate::minify_html_str(NOT_FOUND_HTML));

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
            Ok(content) => Html(crate::minify_html_str(&content)),
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::response::IntoResponse;

    #[test]
    fn test_app_error_not_found_status() {
        let error = AppError::NotFound;
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_app_error_invalid_hashid_status() {
        let error = AppError::InvalidHashid;
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_app_error_rate_limit_status() {
        let error = AppError::RateLimitExceeded;
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
    }

    #[test]
    fn test_app_error_service_unavailable_status() {
        let error = AppError::ServiceUnavailable;
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }

    #[test]
    fn test_app_error_internal_status() {
        let error = AppError::Internal(anyhow::anyhow!("test error"));
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_app_error_display() {
        assert_eq!(AppError::NotFound.to_string(), "URL not found");
        assert_eq!(AppError::InvalidHashid.to_string(), "Invalid hashid");
        assert_eq!(
            AppError::RateLimitExceeded.to_string(),
            "Rate limit exceeded"
        );
        assert_eq!(
            AppError::ServiceUnavailable.to_string(),
            "Service unavailable"
        );
    }

    #[test]
    fn test_app_error_database_status() {
        let error = AppError::Database(sqlx::Error::RowNotFound);
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_app_error_database_display() {
        let error = AppError::Database(sqlx::Error::RowNotFound);
        assert!(error.to_string().contains("Database error"));
    }

    #[test]
    fn test_app_error_internal_display() {
        let error = AppError::Internal(anyhow::anyhow!("something went wrong"));
        assert!(error.to_string().contains("Internal error"));
        assert!(error.to_string().contains("something went wrong"));
    }

    #[test]
    fn test_app_error_debug() {
        let error = AppError::NotFound;
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("NotFound"));
    }

    #[test]
    fn test_app_error_from_anyhow() {
        let anyhow_err = anyhow::anyhow!("test");
        let app_err: AppError = anyhow_err.into();
        assert!(matches!(app_err, AppError::Internal(_)));
    }

    #[test]
    fn test_app_error_from_sqlx() {
        let sqlx_err = sqlx::Error::RowNotFound;
        let app_err: AppError = sqlx_err.into();
        assert!(matches!(app_err, AppError::Database(_)));
    }
}
