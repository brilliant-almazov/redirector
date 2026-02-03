#[cfg(test)]
mod tests {
    use crate::config::HashidsConfig;
    use crate::services::hashid::HashidService;
    use crate::services::HashidDecoder;
    use harsh::Harsh;

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

        assert_ne!(decoded, Some(42));
    }

    #[test]
    fn test_decode_with_multiple_salts_fallback() {
        let config = test_config(vec!["new_salt", "old_salt"]);
        let service = HashidService::new(&config);

        let encoder = Harsh::builder().salt("old_salt").length(6).build().unwrap();

        let hashid = encoder.encode(&[123]);
        let decoded = service.decode(&hashid);

        assert_eq!(decoded, Some(123));
    }

    #[test]
    fn test_decode_invalid_hashid() {
        let config = test_config(vec!["test_salt"]);
        let service = HashidService::new(&config);

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
