use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use governor::{
    clock::DefaultClock,
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter,
};
use std::{num::NonZeroU32, sync::Arc};

pub type SharedRateLimiter = Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>;

#[derive(Clone)]
pub struct RateLimitLayer {
    limiter: SharedRateLimiter,
}

impl RateLimitLayer {
    pub fn new(requests_per_second: u32, burst: u32) -> Self {
        let quota = Quota::per_second(NonZeroU32::new(requests_per_second).unwrap())
            .allow_burst(NonZeroU32::new(burst).unwrap());

        Self {
            limiter: Arc::new(RateLimiter::direct(quota)),
        }
    }

    pub fn check(&self) -> bool {
        self.limiter.check().is_ok()
    }
}

pub async fn rate_limit_middleware(
    limiter: axum::Extension<RateLimitLayer>,
    request: Request<Body>,
    next: Next,
) -> Response {
    if limiter.check() {
        next.run(request).await
    } else {
        metrics::counter!("rate_limit_exceeded").increment(1);
        (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded").into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limiter_allows_requests() {
        let limiter = RateLimitLayer::new(10, 10);

        // First few requests should pass
        for _ in 0..10 {
            assert!(limiter.check());
        }
    }

    #[test]
    fn test_rate_limiter_blocks_excess() {
        let limiter = RateLimitLayer::new(1, 1);

        // First request passes
        assert!(limiter.check());

        // Second immediate request should fail
        assert!(!limiter.check());
    }
}
