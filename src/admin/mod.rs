pub mod auth;
pub mod components;
pub mod pages;
pub mod sse;
pub mod state;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::admin::auth::auth_middleware;
use crate::admin::pages::{dashboard_page, login_page};
use crate::admin::sse::events_handler;

pub use state::AdminState;

pub fn admin_routes(admin_state: AdminState) -> Router {
    let protected = Router::new()
        .route("/dashboard", get(dashboard_page))
        .route("/events", get(events_handler))
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
