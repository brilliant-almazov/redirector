use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub name: &'static str,
    pub version: &'static str,
}

pub async fn index_handler() -> Html<String> {
    let template = IndexTemplate {
        name: env!("CARGO_PKG_NAME"),
        version: env!("CARGO_PKG_VERSION"),
    };

    Html(
        template
            .render()
            .unwrap_or_else(|_| "Redirector".to_string()),
    )
}

#[cfg(test)]
#[path = "index_test.rs"]
mod index_test;
