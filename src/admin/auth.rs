use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
    body::Body,
    extract::State,
    http::Request,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
    Form,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use serde::Deserialize;

use crate::admin::AdminState;

const SESSION_COOKIE: &str = "redirector_session";

#[derive(Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

/// Login handler
pub async fn login_handler(
    State(state): State<AdminState>,
    jar: CookieJar,
    Form(form): Form<LoginForm>,
) -> impl IntoResponse {
    // Find user and verify password
    if let Some(user) = state.find_user(&form.username) {
        if verify_password(&form.password, &user.password_hash) {
            // Create session
            let token = state.create_session(form.username).await;

            // Set cookie
            let cookie = Cookie::build((SESSION_COOKIE, token))
                .path("/admin")
                .http_only(true)
                .secure(false) // Set to true in production with HTTPS
                .max_age(time::Duration::hours(24))
                .build();

            return (jar.add(cookie), Redirect::to("/admin/dashboard")).into_response();
        }
    }

    // Login failed - redirect back with error
    Redirect::to("/admin/?error=invalid").into_response()
}

/// Logout handler
pub async fn logout_handler(State(state): State<AdminState>, jar: CookieJar) -> impl IntoResponse {
    if let Some(cookie) = jar.get(SESSION_COOKIE) {
        state.remove_session(cookie.value()).await;
    }

    let cookie = Cookie::build((SESSION_COOKIE, ""))
        .path("/admin")
        .max_age(time::Duration::seconds(0))
        .build();

    (jar.remove(cookie), Redirect::to("/admin/"))
}

/// Auth middleware - checks session cookie
pub async fn auth_middleware(
    State(state): State<AdminState>,
    jar: CookieJar,
    request: Request<Body>,
    next: Next,
) -> Response {
    if let Some(cookie) = jar.get(SESSION_COOKIE) {
        if state.validate_session(cookie.value()).await.is_some() {
            return next.run(request).await;
        }
    }

    Redirect::to("/admin/").into_response()
}

/// Verify password against Argon2 hash
fn verify_password(password: &str, hash: &str) -> bool {
    let Ok(parsed_hash) = PasswordHash::new(hash) else {
        return false;
    };

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

/// Hash a password (for CLI tool)
pub fn hash_password(password: &str) -> String {
    use argon2::{password_hash::SaltString, PasswordHasher};
    use rand::rngs::OsRng;

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string()
}
