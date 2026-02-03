pub mod auth;
pub mod components;
pub mod pages;
pub mod simulation;
pub mod sse;
pub mod state;

#[cfg(test)]
mod auth_test;
#[cfg(test)]
mod pages_test;
#[cfg(test)]
mod state_test;

use axum::{
    http::StatusCode,
    middleware,
    response::Redirect,
    routing::{get, post},
    Router,
};

use crate::admin::auth::auth_middleware;
use crate::admin::pages::{dashboard_page, login_page};
use crate::admin::sse::events_handler;

pub use state::AdminState;

/// Simulate a redirect request (for dashboard testing)
async fn simulate_handler() -> StatusCode {
    // Generate random latency between 1-50ms
    let latency = rand::random::<u64>() % 50000 + 1000; // 1-50ms in micros
    crate::metrics::record_request(latency);

    // 95% cache hit rate simulation
    if rand::random::<u8>() < 243 {
        // ~95%
        crate::metrics::record_cache_hit();
    } else {
        crate::metrics::record_cache_miss();
    }

    // Get random entry from real simulation data (loaded from binary file)
    let entry = simulation::get_random_entry();
    crate::metrics::record_recent_redirect(entry.hashid.clone(), entry.url.clone());

    StatusCode::OK
}

pub fn admin_routes(admin_state: AdminState) -> Router {
    let protected = Router::new()
        .route("/dashboard", get(dashboard_page))
        .route(
            "/dashboard/",
            get(|| async { Redirect::permanent("/admin/dashboard") }),
        )
        .route("/events", get(events_handler))
        .route("/simulate", post(simulate_handler))
        .route_layer(middleware::from_fn_with_state(
            admin_state.clone(),
            auth_middleware,
        ));

    Router::new()
        .route("/", get(login_page))
        .route("/login", post(auth::login_handler))
        .route("/logout", post(auth::logout_handler))
        .merge(protected)
        .with_state(admin_state)
}
