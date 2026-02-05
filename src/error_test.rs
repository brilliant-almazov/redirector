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
