use axum::response::IntoResponse;
use metrics_exporter_prometheus::PrometheusHandle;

pub async fn metrics_handler(handle: PrometheusHandle) -> impl IntoResponse {
    // Update runtime metrics before rendering
    crate::metrics::update();
    handle.render()
}

#[cfg(test)]
#[path = "metrics_test.rs"]
mod metrics_test;
