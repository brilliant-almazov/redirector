#[cfg(test)]
mod tests {
    use crate::handlers::healthz_handler;

    #[tokio::test]
    async fn test_healthz_returns_ok() {
        let response = healthz_handler().await;
        assert_eq!(response, "ok");
    }
}
