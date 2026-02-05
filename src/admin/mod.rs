pub mod auth;
pub mod components;
pub mod pages;
pub mod simulation;
pub mod sse;
pub mod state;

#[cfg(test)]
mod handlers_test;
#[cfg(test)]
mod pages_test;
#[cfg(test)]
mod simulation_test;
#[cfg(test)]
mod sse_test;
#[cfg(test)]
mod state_test;

use axum::{
    extract::State,
    http::StatusCode,
    middleware,
    response::{Json, Redirect},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};

use crate::admin::auth::auth_middleware;
use crate::admin::pages::{dashboard_page, login_page};
use crate::admin::sse::events_handler;

pub use state::AdminState;

#[derive(Debug, Deserialize)]
pub struct SimulationStartRequest {
    pub rps: u32,
}

#[derive(Debug, Serialize)]
pub struct SimulationStatusResponse {
    pub running: bool,
    pub rps: u32,
}

/// Start background simulation
pub(crate) async fn simulation_start_handler(
    State(admin_state): State<AdminState>,
    Json(req): Json<SimulationStartRequest>,
) -> Json<SimulationStatusResponse> {
    let rps = req.rps.clamp(1, 100000); // Limit to 1-100k RPS
    tracing::info!(rps = rps, "Starting simulation");
    admin_state.start_simulation(rps);
    Json(SimulationStatusResponse { running: true, rps })
}

/// Stop background simulation
pub(crate) async fn simulation_stop_handler(
    State(admin_state): State<AdminState>,
) -> Json<SimulationStatusResponse> {
    admin_state.stop_simulation();
    Json(SimulationStatusResponse {
        running: false,
        rps: admin_state.get_simulation_rps(),
    })
}

/// Get simulation status
pub(crate) async fn simulation_status_handler(
    State(admin_state): State<AdminState>,
) -> Json<SimulationStatusResponse> {
    Json(SimulationStatusResponse {
        running: admin_state.is_simulation_running(),
        rps: admin_state.get_simulation_rps(),
    })
}

/// Simulate a single redirect request (for manual testing)
pub(crate) async fn simulate_handler() -> StatusCode {
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
    // Spawn background simulation task
    simulation::spawn_simulation_task(admin_state.clone());

    let protected = Router::new()
        .route("/dashboard", get(dashboard_page))
        .route(
            "/dashboard/",
            get(|| async { Redirect::permanent("/admin/dashboard") }),
        )
        .route("/events", get(events_handler))
        .route("/simulate", post(simulate_handler))
        .route("/simulate/start", post(simulation_start_handler))
        .route("/simulate/stop", post(simulation_stop_handler))
        .route("/simulate/status", get(simulation_status_handler))
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
