use crate::config::HashidsConfig;
use crate::services::traits::HashidDecoder;
use harsh::Harsh;

pub struct HashidService {
    decoders: Vec<Harsh>,
}

impl HashidService {
    pub fn new(config: &HashidsConfig) -> Self {
        let decoders = config
            .salts
            .iter()
            .map(|salt| {
                Harsh::builder()
                    .salt(salt.clone())
                    .length(config.min_length)
                    .build()
                    .expect("Failed to build hashid decoder")
            })
            .collect();

        Self { decoders }
    }
}

impl HashidDecoder for HashidService {
    /// Decode hashid trying each salt in order.
    /// Returns the first successful decode result (single u64 ID).
    fn decode(&self, hashid: &str) -> Option<i64> {
        for decoder in &self.decoders {
            if let Ok(ids) = decoder.decode(hashid) {
                if ids.len() == 1 {
                    return Some(ids[0] as i64);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config(salts: Vec<&str>) -> HashidsConfig {
        HashidsConfig {
            salts: salts.into_iter().map(String::from).collect(),
            min_length: 6,
        }
    }

    #[test]
    fn test_decode_with_correct_salt() {
        let config = test_config(vec!["test_salt"]);
        let service = HashidService::new(&config);

        let encoder = Harsh::builder()
            .salt("test_salt")
            .length(6)
            .build()
            .unwrap();

        let hashid = encoder.encode(&[42]);
        let decoded = service.decode(&hashid);

        assert_eq!(decoded, Some(42));
    }

    #[test]
    fn test_decode_with_wrong_salt() {
        let config = test_config(vec!["correct_salt"]);
        let service = HashidService::new(&config);

        let encoder = Harsh::builder()
            .salt("wrong_salt")
            .length(6)
            .build()
            .unwrap();

        let hashid = encoder.encode(&[42]);
        let decoded = service.decode(&hashid);

        // Wrong salt will decode to a different number or fail
        assert_ne!(decoded, Some(42));
    }

    #[test]
    fn test_decode_with_multiple_salts_fallback() {
        let config = test_config(vec!["new_salt", "old_salt"]);
        let service = HashidService::new(&config);

        // Encode with old salt
        let encoder = Harsh::builder().salt("old_salt").length(6).build().unwrap();

        let hashid = encoder.encode(&[123]);
        let decoded = service.decode(&hashid);

        assert_eq!(decoded, Some(123));
    }

    #[test]
    fn test_decode_invalid_hashid() {
        let config = test_config(vec!["test_salt"]);
        let service = HashidService::new(&config);

        // Invalid characters for hashid
        let decoded = service.decode("!!invalid!!");

        assert!(decoded.is_none());
    }

    #[test]
    fn test_decode_empty_string() {
        let config = test_config(vec!["test_salt"]);
        let service = HashidService::new(&config);

        let decoded = service.decode("");
        assert!(decoded.is_none());
    }

    #[test]
    fn test_decode_with_first_salt_priority() {
        let config = test_config(vec!["first_salt", "second_salt"]);
        let service = HashidService::new(&config);

        // Encode with first salt
        let encoder = Harsh::builder()
            .salt("first_salt")
            .length(6)
            .build()
            .unwrap();

        let hashid = encoder.encode(&[999]);
        let decoded = service.decode(&hashid);

        assert_eq!(decoded, Some(999));
    }
}
