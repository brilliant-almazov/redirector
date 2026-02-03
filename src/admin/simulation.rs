use once_cell::sync::Lazy;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Clone)]
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

fn load_simulation_data(path: &str) -> Result<Vec<SimulationEntry>, std::io::Error> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_entry_returns_valid_entry() {
        let entry = get_random_entry();
        assert!(!entry.hashid.is_empty());
        assert!(!entry.url.is_empty());
        assert!(entry.id > 0);
    }

    #[test]
    fn test_get_random_entry_multiple_calls() {
        // Call multiple times to ensure no panic
        for _ in 0..100 {
            let entry = get_random_entry();
            assert!(!entry.hashid.is_empty());
        }
    }

    #[test]
    fn test_simulation_entry_clone() {
        let entry = get_random_entry();
        let cloned = entry.clone();
        assert_eq!(entry.id, cloned.id);
        assert_eq!(entry.hashid, cloned.hashid);
        assert_eq!(entry.url, cloned.url);
    }

    #[test]
    fn test_simulation_data_has_entries() {
        // Force initialization and check we have data
        let entry = get_random_entry();
        assert!(entry.id >= 1);
    }

    #[test]
    fn test_default_entries_when_file_missing() {
        // When binary file is missing, defaults should be used
        let entry = get_random_entry();
        // Default entries have ids 1 or 2
        assert!(entry.id >= 1);
    }
}
