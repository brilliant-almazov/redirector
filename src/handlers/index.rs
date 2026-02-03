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
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_index_handler_returns_html() {
        let response = index_handler().await;
        assert!(!response.0.is_empty());
    }
}
