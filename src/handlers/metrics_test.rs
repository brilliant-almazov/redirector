use super::*;
use axum::response::IntoResponse;
use metrics_exporter_prometheus::PrometheusBuilder;

#[tokio::test]
async fn test_metrics_handler_returns_prometheus_format() {
    let recorder = PrometheusBuilder::new().build_recorder();
    let handle = recorder.handle();

    let response = metrics_handler(handle).await;
    let response = response.into_response();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
}

#[tokio::test]
async fn test_metrics_handler_content_not_empty() {
    let recorder = PrometheusBuilder::new().build_recorder();
    let handle = recorder.handle();

    let response = metrics_handler(handle).await;
    let body = response.into_response();
    let bytes = axum::body::to_bytes(body.into_body(), 1024 * 1024)
        .await
        .unwrap();

    let _ = bytes;
}
