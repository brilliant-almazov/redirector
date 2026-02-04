#[cfg(test)]
mod tests {
    use axum::{body::Body, http::Request, routing::get, Router};
    use tower::ServiceExt;

    use crate::middleware::version_header::set_version_header;

    #[tokio::test]
    async fn test_version_header_present() {
        let app = Router::new()
            .route("/test", get(|| async { "hello" }))
            .layer(axum::middleware::map_response(set_version_header));

        let response = app
            .oneshot(Request::builder().uri("/test").body(Body::empty()).unwrap())
            .await
            .unwrap();

        let version = response
            .headers()
            .get("X-Version")
            .unwrap()
            .to_str()
            .unwrap();
        assert!(
            version.starts_with(env!("CARGO_PKG_VERSION")),
            "expected version to start with {}, got {}",
            env!("CARGO_PKG_VERSION"),
            version
        );
        assert!(version.contains('+'), "expected '+' separator in {version}");
    }

    #[tokio::test]
    async fn test_version_header_on_404() {
        let app = Router::new()
            .route("/test", get(|| async { "hello" }))
            .layer(axum::middleware::map_response(set_version_header));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/nonexistent")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let version = response
            .headers()
            .get("X-Version")
            .unwrap()
            .to_str()
            .unwrap();
        assert!(version.starts_with(env!("CARGO_PKG_VERSION")));
    }
}
