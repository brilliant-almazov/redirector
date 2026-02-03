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
#[path = "rate_limit_test.rs"]
mod rate_limit_test;
