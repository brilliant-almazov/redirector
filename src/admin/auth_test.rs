#[cfg(test)]
mod tests {
    use crate::admin::auth::hash_password;

    #[test]
    fn test_hash_password_produces_argon2_hash() {
        let hash = hash_password("test123");

        // Argon2 hashes start with $argon2
        assert!(hash.starts_with("$argon2"));
    }

    #[test]
    fn test_hash_password_different_each_time() {
        let hash1 = hash_password("same_password");
        let hash2 = hash_password("same_password");

        // Each hash should be different due to random salt
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_hash_password_not_empty() {
        let hash = hash_password("password");

        assert!(!hash.is_empty());
        // Argon2 hashes are typically 80+ characters
        assert!(hash.len() > 50);
    }

    #[test]
    fn test_hash_password_handles_special_chars() {
        let hash = hash_password("p@$$w0rd!#$%^&*()");

        assert!(hash.starts_with("$argon2"));
    }

    #[test]
    fn test_hash_password_handles_unicode() {
        let hash = hash_password("пароль密码パスワード");

        assert!(hash.starts_with("$argon2"));
    }

    #[test]
    fn test_hash_password_handles_empty_string() {
        let hash = hash_password("");

        assert!(hash.starts_with("$argon2"));
    }

    #[test]
    fn test_hash_password_handles_long_password() {
        let long_password = "a".repeat(1000);
        let hash = hash_password(&long_password);

        assert!(hash.starts_with("$argon2"));
    }
}
