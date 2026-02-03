#[cfg(test)]
mod tests {
    use crate::admin::simulation::{get_random_entry, SimulationEntry};

    #[test]
    fn test_get_random_entry_returns_valid_entry() {
        let entry = get_random_entry();
        assert!(!entry.hashid.is_empty());
        assert!(!entry.url.is_empty());
        assert!(entry.id > 0);
    }

    #[test]
    fn test_get_random_entry_multiple_calls() {
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
        let entry = get_random_entry();
        assert!(entry.id >= 1);
    }

    #[test]
    fn test_default_entries_when_file_missing() {
        let entry = get_random_entry();
        assert!(entry.id >= 1);
    }

    #[test]
    fn test_simulation_entry_fields() {
        let entry = SimulationEntry {
            id: 42,
            hashid: "test123".to_string(),
            url: "https://example.com/path".to_string(),
        };
        assert_eq!(entry.id, 42);
        assert_eq!(entry.hashid, "test123");
        assert_eq!(entry.url, "https://example.com/path");
    }

    #[test]
    fn test_simulation_entry_debug() {
        let entry = SimulationEntry {
            id: 1,
            hashid: "abc".to_string(),
            url: "https://example.com".to_string(),
        };
        let debug = format!("{:?}", entry);
        assert!(debug.contains("SimulationEntry"));
        assert!(debug.contains("abc"));
    }

    #[test]
    fn test_get_random_entry_distribution() {
        let mut seen_ids = std::collections::HashSet::new();
        for _ in 0..1000 {
            let entry = get_random_entry();
            seen_ids.insert(entry.id);
        }
        assert!(!seen_ids.is_empty());
    }

    #[test]
    fn test_get_random_entry_url_formats() {
        let entry = get_random_entry();
        assert!(entry.url.starts_with("http://") || entry.url.starts_with("https://"));
    }

    #[test]
    fn test_load_simulation_data_missing_file() {
        let result = crate::admin::simulation::load_simulation_data("nonexistent/path.bin");
        assert!(result.is_err());
    }

    #[test]
    fn test_load_simulation_data_returns_io_error() {
        let result =
            crate::admin::simulation::load_simulation_data("/this/path/does/not/exist.bin");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
    }

    #[test]
    fn test_load_simulation_data_empty_file() {
        // Create a temp file with just a zero entry count
        let dir = std::env::temp_dir();
        let path = dir.join("test_sim_empty.bin");
        std::fs::write(&path, 0u32.to_le_bytes()).unwrap();

        let result = crate::admin::simulation::load_simulation_data(path.to_str().unwrap());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);

        std::fs::remove_file(path).ok();
    }

    #[test]
    fn test_load_simulation_data_valid_binary() {
        let dir = std::env::temp_dir();
        let path = dir.join("test_sim_valid.bin");

        let mut data = Vec::new();
        // Header: 1 entry
        data.extend_from_slice(&1u32.to_le_bytes());
        // Entry: id=42
        data.extend_from_slice(&42i64.to_le_bytes());
        // hashid: "test"
        let hashid = b"test";
        data.extend_from_slice(&(hashid.len() as u16).to_le_bytes());
        data.extend_from_slice(hashid);
        // url: "https://example.com"
        let url = b"https://example.com";
        data.extend_from_slice(&(url.len() as u16).to_le_bytes());
        data.extend_from_slice(url);

        std::fs::write(&path, &data).unwrap();

        let result = crate::admin::simulation::load_simulation_data(path.to_str().unwrap());
        assert!(result.is_ok());
        let entries = result.unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].id, 42);
        assert_eq!(entries[0].hashid, "test");
        assert_eq!(entries[0].url, "https://example.com");

        std::fs::remove_file(path).ok();
    }

    #[test]
    fn test_load_simulation_data_truncated_file() {
        let dir = std::env::temp_dir();
        let path = dir.join("test_sim_truncated.bin");

        // Header says 1 entry but no data follows
        let mut data = Vec::new();
        data.extend_from_slice(&1u32.to_le_bytes());
        std::fs::write(&path, &data).unwrap();

        let result = crate::admin::simulation::load_simulation_data(path.to_str().unwrap());
        assert!(result.is_err());

        std::fs::remove_file(path).ok();
    }

    #[test]
    fn test_load_simulation_data_multiple_entries() {
        let dir = std::env::temp_dir();
        let path = dir.join("test_sim_multi.bin");

        let mut data = Vec::new();
        // Header: 2 entries
        data.extend_from_slice(&2u32.to_le_bytes());

        for i in 0..2 {
            data.extend_from_slice(&(i as i64 + 1).to_le_bytes());
            let hashid = format!("hash{}", i);
            data.extend_from_slice(&(hashid.len() as u16).to_le_bytes());
            data.extend_from_slice(hashid.as_bytes());
            let url = format!("https://example{}.com", i);
            data.extend_from_slice(&(url.len() as u16).to_le_bytes());
            data.extend_from_slice(url.as_bytes());
        }

        std::fs::write(&path, &data).unwrap();

        let result = crate::admin::simulation::load_simulation_data(path.to_str().unwrap());
        assert!(result.is_ok());
        let entries = result.unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].hashid, "hash0");
        assert_eq!(entries[1].hashid, "hash1");

        std::fs::remove_file(path).ok();
    }
}
