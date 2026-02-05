use super::*;

#[test]
fn test_normalize_domain_basic() {
    assert_eq!(normalize_domain("https://example.com/path"), "example.com");
}

#[test]
fn test_normalize_domain_www() {
    assert_eq!(
        normalize_domain("https://www.example.com/path"),
        "example.com"
    );
}

#[test]
fn test_normalize_domain_uppercase() {
    assert_eq!(
        normalize_domain("https://WWW.Example.COM/path"),
        "example.com"
    );
}

#[test]
fn test_normalize_domain_trailing_dot() {
    assert_eq!(normalize_domain("https://example.com./path"), "example.com");
}

#[test]
fn test_normalize_domain_with_port() {
    assert_eq!(
        normalize_domain("https://example.com:8080/path"),
        "example.com"
    );
}

#[test]
fn test_normalize_domain_with_query() {
    assert_eq!(
        normalize_domain("https://example.com?q=test"),
        "example.com"
    );
}

#[test]
fn test_normalize_domain_no_protocol() {
    assert_eq!(normalize_domain("example.com/path"), "example.com");
}

#[test]
fn test_normalize_domain_empty() {
    assert_eq!(normalize_domain(""), "(unknown)");
}

#[test]
fn test_normalize_domain_just_protocol() {
    assert_eq!(normalize_domain("https://"), "(unknown)");
}

#[test]
fn test_normalize_domain_http() {
    assert_eq!(
        normalize_domain("http://www.google.com/search?q=rust"),
        "google.com"
    );
}

#[test]
fn test_normalize_domain_subdomain() {
    assert_eq!(
        normalize_domain("https://blog.example.com/post/123"),
        "blog.example.com"
    );
}
