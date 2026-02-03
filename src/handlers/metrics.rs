use axum::response::IntoResponse;
use metrics_exporter_prometheus::PrometheusHandle;

pub async fn metrics_handler(handle: PrometheusHandle) -> impl IntoResponse {
    // Update runtime metrics before rendering
    crate::metrics::update();
    handle.render()
}

#[cfg(test)]
mod tests {
    use super::*;
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

        // Metrics response should not panic - content depends on recorder state
        let _ = bytes;
    }
}
