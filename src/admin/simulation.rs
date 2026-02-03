use once_cell::sync::Lazy;
use std::fs::File;
use std::io::{BufReader, Read};
use std::sync::atomic::Ordering;
use std::time::Duration;
use tokio::time::interval;

use crate::admin::state::AdminState;

#[derive(Clone, Debug)]
pub struct SimulationEntry {
    pub id: i64,
    pub hashid: String,
    pub url: String,
}

static SIMULATION_DATA: Lazy<Vec<SimulationEntry>> = Lazy::new(|| {
    load_simulation_data("static/simulation_data.bin").unwrap_or_else(|e| {
        tracing::warn!("Failed to load simulation data: {}, using defaults", e);
        vec![
            SimulationEntry {
                id: 1,
                hashid: "abc123".to_string(),
                url: "https://example.com".to_string(),
            },
            SimulationEntry {
                id: 2,
                hashid: "xyz789".to_string(),
                url: "https://github.com".to_string(),
            },
        ]
    })
});

pub(crate) fn load_simulation_data(path: &str) -> Result<Vec<SimulationEntry>, std::io::Error> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    // Read header: number of entries (u32)
    let mut num_entries_bytes = [0u8; 4];
    reader.read_exact(&mut num_entries_bytes)?;
    let num_entries = u32::from_le_bytes(num_entries_bytes) as usize;

    let mut entries = Vec::with_capacity(num_entries);

    for _ in 0..num_entries {
        // Read ID (i64)
        let mut id_bytes = [0u8; 8];
        reader.read_exact(&mut id_bytes)?;
        let id = i64::from_le_bytes(id_bytes);

        // Read hashid length and bytes
        let mut hashid_len_bytes = [0u8; 2];
        reader.read_exact(&mut hashid_len_bytes)?;
        let hashid_len = u16::from_le_bytes(hashid_len_bytes) as usize;

        let mut hashid_bytes = vec![0u8; hashid_len];
        reader.read_exact(&mut hashid_bytes)?;
        let hashid = String::from_utf8_lossy(&hashid_bytes).to_string();

        // Read URL length and bytes
        let mut url_len_bytes = [0u8; 2];
        reader.read_exact(&mut url_len_bytes)?;
        let url_len = u16::from_le_bytes(url_len_bytes) as usize;

        let mut url_bytes = vec![0u8; url_len];
        reader.read_exact(&mut url_bytes)?;
        let url = String::from_utf8_lossy(&url_bytes).to_string();

        entries.push(SimulationEntry { id, hashid, url });
    }

    tracing::info!(
        "Loaded {} simulation entries from binary file",
        entries.len()
    );
    Ok(entries)
}

pub fn get_random_entry() -> &'static SimulationEntry {
    let idx = rand::random::<usize>() % SIMULATION_DATA.len();
    &SIMULATION_DATA[idx]
}

/// Spawn a background task that simulates traffic
pub fn spawn_simulation_task(admin_state: AdminState) {
    tracing::info!("Spawning simulation background task");
    tokio::spawn(async move {
        let mut tick_interval = interval(Duration::from_millis(10)); // Check every 10ms
        let mut request_accumulator: f64 = 0.0;
        let mut was_running = false;

        loop {
            tick_interval.tick().await;

            let is_running = admin_state.simulation.running.load(Ordering::SeqCst);

            if is_running && !was_running {
                tracing::info!("Simulation started");
            } else if !is_running && was_running {
                tracing::info!("Simulation stopped");
            }
            was_running = is_running;

            if !is_running {
                // Not running, reset accumulator and wait
                request_accumulator = 0.0;
                continue;
            }

            let rps = admin_state.simulation.rps.load(Ordering::SeqCst) as f64;
            // Calculate how many requests per 10ms tick
            request_accumulator += rps / 100.0; // 100 ticks per second

            while request_accumulator >= 1.0 {
                request_accumulator -= 1.0;

                // Generate simulated request
                let latency = rand::random::<u64>() % 50000 + 1000; // 1-50ms in micros
                crate::metrics::record_request(latency);

                // 95% cache hit rate simulation
                if rand::random::<u8>() < 243 {
                    crate::metrics::record_cache_hit();
                } else {
                    crate::metrics::record_cache_miss();
                }

                // Record random redirect entry
                let entry = get_random_entry();
                crate::metrics::record_recent_redirect(entry.hashid.clone(), entry.url.clone());
            }
        }
    });
}
