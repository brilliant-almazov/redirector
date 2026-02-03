#[cfg(test)]
mod tests {
    use crate::middleware::RateLimitLayer;

    #[test]
    fn test_rate_limiter_allows_requests() {
        let limiter = RateLimitLayer::new(10, 10);

        for _ in 0..10 {
            assert!(limiter.check());
        }
    }

    #[test]
    fn test_rate_limiter_blocks_excess() {
        let limiter = RateLimitLayer::new(1, 1);

        assert!(limiter.check());
        assert!(!limiter.check());
    }

    #[test]
    fn test_rate_limiter_with_burst() {
        let limiter = RateLimitLayer::new(1, 5);

        // Should allow burst
        for _ in 0..5 {
            assert!(limiter.check());
        }

        // Should block after burst
        assert!(!limiter.check());
    }

    #[test]
    fn test_rate_limiter_high_rate() {
        let limiter = RateLimitLayer::new(1000, 100);

        // Should allow many requests with high rate
        for _ in 0..100 {
            assert!(limiter.check());
        }
    }

    #[test]
    fn test_rate_limiter_clone() {
        let limiter1 = RateLimitLayer::new(10, 10);
        let limiter2 = limiter1.clone();

        // Both limiters share the same state
        for _ in 0..10 {
            limiter1.check();
        }
        // limiter2 should also be affected since they share Arc
        assert!(!limiter2.check());
    }

    #[test]
    fn test_rate_limiter_minimum_values() {
        let limiter = RateLimitLayer::new(1, 1);
        assert!(limiter.check());
        assert!(!limiter.check());
    }

    #[test]
    fn test_rate_limiter_large_burst() {
        let limiter = RateLimitLayer::new(100, 1000);
        for _ in 0..1000 {
            assert!(limiter.check());
        }
        assert!(!limiter.check());
    }
}

#[cfg(test)]
mod middleware_tests {
    use super::super::{rate_limit_middleware, RateLimitLayer};
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::get,
        Extension, Router,
    };
    use tower::ServiceExt;

    async fn test_handler() -> &'static str {
        "OK"
    }

    fn create_app(rps: u32, burst: u32) -> Router {
        let limiter = RateLimitLayer::new(rps, burst);
        Router::new()
            .route("/test", get(test_handler))
            .layer(axum::middleware::from_fn(rate_limit_middleware))
            .layer(Extension(limiter))
    }

    #[tokio::test]
    async fn test_middleware_allows_within_limit() {
        let app = create_app(100, 10);
        let request = Request::builder().uri("/test").body(Body::empty()).unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_middleware_blocks_excess_requests() {
        let limiter = RateLimitLayer::new(1, 1);
        let app = Router::new()
            .route("/test", get(test_handler))
            .layer(axum::middleware::from_fn(rate_limit_middleware))
            .layer(Extension(limiter.clone()));

        // First request should succeed
        let request1 = Request::builder().uri("/test").body(Body::empty()).unwrap();
        let response1 = app.clone().oneshot(request1).await.unwrap();
        assert_eq!(response1.status(), StatusCode::OK);

        // Second request should be rate limited
        let request2 = Request::builder().uri("/test").body(Body::empty()).unwrap();
        let response2 = app.oneshot(request2).await.unwrap();
        assert_eq!(response2.status(), StatusCode::TOO_MANY_REQUESTS);
    }

    #[tokio::test]
    async fn test_middleware_rate_limit_response_body() {
        let limiter = RateLimitLayer::new(1, 1);
        let app = Router::new()
            .route("/test", get(test_handler))
            .layer(axum::middleware::from_fn(rate_limit_middleware))
            .layer(Extension(limiter.clone()));

        // Exhaust the limit
        let _ = app
            .clone()
            .oneshot(Request::builder().uri("/test").body(Body::empty()).unwrap())
            .await;

        // Check rate limited response
        let request = Request::builder().uri("/test").body(Body::empty()).unwrap();
        let response = app.oneshot(request).await.unwrap();

        let body = axum::body::to_bytes(response.into_body(), 1024)
            .await
            .unwrap();
        assert_eq!(&body[..], b"Rate limit exceeded");
    }
}
