use axum::{
    body::Body,
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use base64::{engine::general_purpose::STANDARD, Engine};

#[derive(Clone)]
pub struct BasicAuthLayer {
    username: String,
    password: String,
}

impl BasicAuthLayer {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }

    pub fn check(&self, auth_header: Option<&str>) -> bool {
        let Some(auth) = auth_header else {
            return false;
        };

        let Some(credentials) = auth.strip_prefix("Basic ") else {
            return false;
        };

        let Ok(decoded) = STANDARD.decode(credentials) else {
            return false;
        };

        let Ok(decoded_str) = String::from_utf8(decoded) else {
            return false;
        };

        let Some((user, pass)) = decoded_str.split_once(':') else {
            return false;
        };

        use subtle::ConstantTimeEq;
        let user_match = user.as_bytes().ct_eq(self.username.as_bytes());
        let pass_match = pass.as_bytes().ct_eq(self.password.as_bytes());

        user_match.into() && pass_match.into()
    }
}

pub async fn basic_auth_middleware(
    auth: axum::Extension<BasicAuthLayer>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok());

    if auth.check(auth_header) {
        next.run(request).await
    } else {
        (
            StatusCode::UNAUTHORIZED,
            [(header::WWW_AUTHENTICATE, "Basic realm=\"metrics\"")],
            "Unauthorized",
        )
            .into_response()
    }
}

#[cfg(test)]
#[path = "basic_auth_test.rs"]
mod basic_auth_test;
