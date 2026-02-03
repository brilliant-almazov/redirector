use axum::response::Html;

#[cfg(not(debug_assertions))]
use std::sync::LazyLock;

#[cfg(not(debug_assertions))]
use super::minify_html;

// In release mode, embed and minify at compile time
#[cfg(not(debug_assertions))]
static LOGIN_HTML: &str = include_str!("../../../templates/login.html");

#[cfg(not(debug_assertions))]
static LOGIN_MINIFIED: LazyLock<String> = LazyLock::new(|| minify_html(LOGIN_HTML));

#[cfg(not(debug_assertions))]
pub async fn login_page() -> Html<String> {
    Html(LOGIN_MINIFIED.clone())
}

// In debug mode, read from file at runtime for hot-reload
#[cfg(debug_assertions)]
pub async fn login_page() -> Html<String> {
    match std::fs::read_to_string("templates/login.html") {
        Ok(content) => Html(content),
        Err(e) => Html(format!("<h1>Error loading login page: {}</h1>", e)),
    }
}
