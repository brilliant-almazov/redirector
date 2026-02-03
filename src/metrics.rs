use once_cell::sync::Lazy;
use std::time::Instant;

static START_TIME: Lazy<Instant> = Lazy::new(Instant::now);

/// Initialize service metrics
pub fn init() {
    // Record start time to calculate uptime
    let _ = *START_TIME;

    // Build info as gauge
    metrics::gauge!("redirector_build_info",
        "version" => env!("CARGO_PKG_VERSION")
    ).set(1.0);

    // Service up indicator
    metrics::gauge!("redirector_up").set(1.0);

    // Initial uptime
    metrics::gauge!("redirector_uptime_seconds").set(0.0);
}

/// Update runtime metrics - should be called from metrics endpoint
pub fn update() {
    let uptime = START_TIME.elapsed().as_secs_f64();
    metrics::gauge!("redirector_uptime_seconds").set(uptime);
}
