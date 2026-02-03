#[cfg(test)]
mod tests {
    use crate::handlers::index_handler;

    #[tokio::test]
    async fn test_index_handler_returns_html() {
        let response = index_handler().await;
        assert!(!response.0.is_empty());
    }

    #[tokio::test]
    async fn test_index_handler_contains_package_name() {
        let response = index_handler().await;
        assert!(response.0.contains("redirector"));
    }
}
