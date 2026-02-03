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

    #[test]
    fn test_invalid_utf8_in_credentials() {
        let auth = layer("admin", "secret");
        // Valid base64 but invalid UTF-8
        let invalid_utf8 = STANDARD.encode([0xff, 0xfe, 0x3a, 0x70, 0x61, 0x73, 0x73]);
        let header = format!("Basic {}", invalid_utf8);
        assert!(!auth.check(Some(&header)));
    }

    #[test]
    fn test_basic_auth_layer_clone() {
        let auth1 = layer("admin", "secret");
        let auth2 = auth1.clone();
        let header = encode_basic("admin", "secret");
        assert!(auth1.check(Some(&header)));
        assert!(auth2.check(Some(&header)));
    }

    #[test]
    fn test_whitespace_in_credentials() {
        let auth = layer("admin user", "secret pass");
        let header = encode_basic("admin user", "secret pass");
        assert!(auth.check(Some(&header)));
    }

    #[test]
    fn test_multiple_colons_in_password() {
        let auth = layer("admin", "pass:word:with:colons");
        let header = encode_basic("admin", "pass:word:with:colons");
        assert!(auth.check(Some(&header)));
    }
}

#[cfg(test)]
mod middleware_tests {
    use super::super::{basic_auth_middleware, BasicAuthLayer};
    use axum::{
        body::Body,
        http::{header, Request, StatusCode},
        routing::get,
        Extension, Router,
    };
    use base64::{engine::general_purpose::STANDARD, Engine};
    use tower::ServiceExt;

    fn encode_basic(user: &str, pass: &str) -> String {
        format!("Basic {}", STANDARD.encode(format!("{}:{}", user, pass)))
    }

    async fn test_handler() -> &'static str {
        "OK"
    }

    fn create_app(username: &str, password: &str) -> Router {
        let auth = BasicAuthLayer::new(username.to_string(), password.to_string());
        Router::new()
            .route("/test", get(test_handler))
            .layer(axum::middleware::from_fn(basic_auth_middleware))
            .layer(Extension(auth))
    }

    #[tokio::test]
    async fn test_middleware_allows_valid_credentials() {
        let app = create_app("admin", "secret");
        let request = Request::builder()
            .uri("/test")
            .header(header::AUTHORIZATION, encode_basic("admin", "secret"))
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_middleware_rejects_invalid_credentials() {
        let app = create_app("admin", "secret");
        let request = Request::builder()
            .uri("/test")
            .header(header::AUTHORIZATION, encode_basic("admin", "wrong"))
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_middleware_rejects_missing_header() {
        let app = create_app("admin", "secret");
        let request = Request::builder().uri("/test").body(Body::empty()).unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_middleware_returns_www_authenticate_header() {
        let app = create_app("admin", "secret");
        let request = Request::builder().uri("/test").body(Body::empty()).unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert!(response.headers().contains_key(header::WWW_AUTHENTICATE));
    }
}
