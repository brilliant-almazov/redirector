use axum::response::Html;

#[cfg(not(debug_assertions))]
use std::sync::LazyLock;

#[cfg(not(debug_assertions))]
static DASHBOARD_HTML: &str = include_str!("../../../templates/dashboard.html");

#[cfg(not(debug_assertions))]
static DASHBOARD_MINIFIED: LazyLock<String> =
    LazyLock::new(|| crate::minify_html_str(DASHBOARD_HTML));

#[cfg(not(debug_assertions))]
pub async fn dashboard_page() -> Html<String> {
    Html(DASHBOARD_MINIFIED.clone())
}

#[cfg(debug_assertions)]
pub async fn dashboard_page() -> Html<String> {
    match std::fs::read_to_string("templates/dashboard.html") {
        Ok(content) => Html(crate::minify_html_str(&content)),
        Err(e) => Html(format!("<h1>Error loading dashboard: {}</h1>", e)),
    }
}
