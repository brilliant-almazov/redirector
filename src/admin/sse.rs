use axum::{
    extract::State,
    response::sse::{Event, KeepAlive, Sse},
};
use futures::stream::{self, Stream};
use serde::Serialize;
use std::convert::Infallible;
use std::time::Duration;
use sysinfo::System;

use crate::admin::state::AdminState;

/// System metrics
#[derive(Serialize)]
pub struct SystemMetrics {
    pub uptime_secs: u64,
    pub cpu_percent: f32,
    pub memory_mb: u64,
}

/// App metrics
#[derive(Serialize)]
pub struct AppMetrics {
    pub rps: f64,
    pub latency_p50_ms: f64,
    pub latency_p95_ms: f64,
    pub latency_p99_ms: f64,
    pub cache_hit_rate: f64,
    pub total_requests: u64,
}

/// Recent redirect for JSON
#[derive(Serialize)]
pub struct RecentRedirectJson {
    pub hashid: String,
    pub url: String,
    pub at: u64,
}

/// Simulation status
#[derive(Serialize)]
pub struct SimulationStatus {
    pub running: bool,
    pub rps: u32,
}

/// Full metrics payload
#[derive(Serialize)]
pub struct MetricsPayload {
    pub timestamp: u64,
    pub system: SystemMetrics,
    pub app: AppMetrics,
    pub recent: Vec<RecentRedirectJson>,
    pub simulation: SimulationStatus,
}

/// SSE events handler
pub async fn events_handler(
    State(admin_state): State<AdminState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::unfold(
        (
            admin_state,
            System::new_all(),
            None::<(u64, std::time::Instant)>,
        ),
        |(state, mut sys, prev_requests)| async move {
            tokio::time::sleep(Duration::from_millis(500)).await;

            // Refresh system info
            sys.refresh_cpu_usage();
            sys.refresh_memory();

            // Get system metrics
            let system = SystemMetrics {
                uptime_secs: crate::metrics::uptime_secs(),
                cpu_percent: sys.global_cpu_usage(),
                memory_mb: sys.used_memory() / 1024 / 1024,
            };

            // Get app metrics from prometheus
            let (total_requests, rps, prev) = calculate_rps(prev_requests);

            let app = AppMetrics {
                rps,
                latency_p50_ms: get_latency_percentile(0.5),
                latency_p95_ms: get_latency_percentile(0.95),
                latency_p99_ms: get_latency_percentile(0.99),
                cache_hit_rate: get_cache_hit_rate(),
                total_requests,
            };

            // Get recent redirects from global metrics
            let recent: Vec<RecentRedirectJson> = crate::metrics::get_recent_redirects()
                .into_iter()
                .take(10)
                .map(|r| RecentRedirectJson {
                    hashid: r.hashid,
                    url: r.url,
                    at: r.timestamp,
                })
                .collect();

            // Get simulation status
            let simulation = SimulationStatus {
                running: state.is_simulation_running(),
                rps: state.get_simulation_rps(),
            };

            let payload = MetricsPayload {
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
                system,
                app,
                recent,
                simulation,
            };

            let json = serde_json::to_string(&payload).unwrap_or_default();
            let event = Event::default().data(json);

            Some((Ok(event), (state, sys, prev)))
        },
    );

    Sse::new(stream).keep_alive(KeepAlive::default())
}

/// Calculate RPS from counter delta
fn calculate_rps(
    prev: Option<(u64, std::time::Instant)>,
) -> (u64, f64, Option<(u64, std::time::Instant)>) {
    // TODO: Get actual counter from metrics
    let total = get_total_requests();
    let now = std::time::Instant::now();

    let rps = if let Some((prev_count, prev_time)) = prev {
        let delta_count = total.saturating_sub(prev_count);
        let delta_secs = now.duration_since(prev_time).as_secs_f64();
        if delta_secs > 0.0 {
            delta_count as f64 / delta_secs
        } else {
            0.0
        }
    } else {
        0.0
    };

    (total, rps, Some((total, now)))
}

// Get metrics from global counters
fn get_total_requests() -> u64 {
    crate::metrics::get_total_requests()
}

fn get_latency_percentile(_percentile: f64) -> f64 {
    // Using average latency as approximation (proper percentiles need histogram)
    crate::metrics::get_avg_latency_ms()
}

fn get_cache_hit_rate() -> f64 {
    crate::metrics::get_cache_hit_rate()
}
