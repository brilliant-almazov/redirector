use askama::Template;
use axum::response::Html;
use std::sync::LazyLock;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub name: &'static str,
    pub version: &'static str,
}

static INDEX_HTML: LazyLock<String> = LazyLock::new(|| {
    let template = IndexTemplate {
        name: env!("CARGO_PKG_NAME"),
        version: env!("CARGO_PKG_VERSION"),
    };
    let html = template
        .render()
        .unwrap_or_else(|_| "Redirector".to_string());
    crate::minify_html_str(&html)
});

pub async fn index_handler() -> Html<String> {
    Html(INDEX_HTML.clone())
}

#[cfg(test)]
#[path = "index_test.rs"]
mod index_test;
