use super::*;

#[test]
fn test_hash_password_produces_argon2_hash() {
    let hash = hash_password("test123");
    assert!(hash.starts_with("$argon2"));
}

#[test]
fn test_hash_password_different_each_time() {
    let hash1 = hash_password("same_password");
    let hash2 = hash_password("same_password");
    assert_ne!(hash1, hash2);
}

#[test]
fn test_hash_password_not_empty() {
    let hash = hash_password("password");
    assert!(!hash.is_empty());
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

#[test]
fn test_verify_password_valid() {
    let password = "test_password_123";
    let hash = hash_password(password);
    assert!(verify_password(password, &hash));
}

#[test]
fn test_verify_password_invalid() {
    let hash = hash_password("correct_password");
    assert!(!verify_password("wrong_password", &hash));
}

#[test]
fn test_verify_password_invalid_hash_format() {
    assert!(!verify_password("password", "not_a_valid_hash"));
}

#[test]
fn test_verify_password_empty_password() {
    let hash = hash_password("");
    assert!(verify_password("", &hash));
}

#[test]
fn test_verify_password_special_chars() {
    let password = "p@$$w0rd!#$%^&*()";
    let hash = hash_password(password);
    assert!(verify_password(password, &hash));
}

#[test]
fn test_verify_password_unicode() {
    let password = "пароль密码";
    let hash = hash_password(password);
    assert!(verify_password(password, &hash));
}

#[test]
fn test_verify_password_wrong_hash_prefix() {
    assert!(!verify_password("password", "$invalid$hash"));
}

#[test]
fn test_login_form_deserialize() {
    let form = LoginForm {
        username: "admin".to_string(),
        password: "secret".to_string(),
    };
    assert_eq!(form.username, "admin");
    assert_eq!(form.password, "secret");
}

#[test]
fn test_session_cookie_constant() {
    assert_eq!(SESSION_COOKIE, "redirector_session");
}
