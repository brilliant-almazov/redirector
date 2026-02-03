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
#[path = "hashid_test.rs"]
mod hashid_test;
