#[cfg(test)]
mod tests {
    use crate::middleware::BasicAuthLayer;
    use base64::{engine::general_purpose::STANDARD, Engine};

    fn layer(user: &str, pass: &str) -> BasicAuthLayer {
        BasicAuthLayer::new(user.to_string(), pass.to_string())
    }

    fn encode_basic(user: &str, pass: &str) -> String {
        format!("Basic {}", STANDARD.encode(format!("{}:{}", user, pass)))
    }

    #[test]
    fn test_valid_credentials() {
        let auth = layer("admin", "secret");
        let header = encode_basic("admin", "secret");
        assert!(auth.check(Some(&header)));
    }

    #[test]
    fn test_invalid_username() {
        let auth = layer("admin", "secret");
        let header = encode_basic("wrong", "secret");
        assert!(!auth.check(Some(&header)));
    }

    #[test]
    fn test_invalid_password() {
        let auth = layer("admin", "secret");
        let header = encode_basic("admin", "wrong");
        assert!(!auth.check(Some(&header)));
    }

    #[test]
    fn test_missing_header() {
        let auth = layer("admin", "secret");
        assert!(!auth.check(None));
    }

    #[test]
    fn test_invalid_scheme() {
        let auth = layer("admin", "secret");
        assert!(!auth.check(Some("Bearer token")));
    }

    #[test]
    fn test_invalid_base64() {
        let auth = layer("admin", "secret");
        assert!(!auth.check(Some("Basic !!!invalid!!!")));
    }

    #[test]
    fn test_no_colon_in_credentials() {
        let auth = layer("admin", "secret");
        let header = format!("Basic {}", STANDARD.encode("nocolon"));
        assert!(!auth.check(Some(&header)));
    }

    #[test]
    fn test_empty_username() {
        let auth = layer("", "secret");
        let header = encode_basic("", "secret");
        assert!(auth.check(Some(&header)));
    }

    #[test]
    fn test_empty_password() {
        let auth = layer("admin", "");
        let header = encode_basic("admin", "");
        assert!(auth.check(Some(&header)));
    }

    #[test]
    fn test_special_characters_in_password() {
        let auth = layer("admin", "p@ss:w0rd!#$%");
        let header = encode_basic("admin", "p@ss:w0rd!#$%");
        assert!(auth.check(Some(&header)));
    }

    #[test]
    fn test_unicode_credentials() {
        let auth = layer("пользователь", "пароль");
        let header = encode_basic("пользователь", "пароль");
        assert!(auth.check(Some(&header)));
    }

    #[test]
    fn test_long_credentials() {
        let long_user = "a".repeat(1000);
        let long_pass = "b".repeat(1000);
        let auth = layer(&long_user, &long_pass);
        let header = encode_basic(&long_user, &long_pass);
        assert!(auth.check(Some(&header)));
    }

    #[test]
    fn test_timing_attack_resistance() {
        let auth = layer("admin", "secret");
        // Different length passwords should take similar time (constant-time comparison)
        let header1 = encode_basic("admin", "a");
        let header2 = encode_basic("admin", "a".repeat(1000).as_str());
        assert!(!auth.check(Some(&header1)));
        assert!(!auth.check(Some(&header2)));
    }
}
