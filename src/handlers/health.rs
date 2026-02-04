pub async fn healthz_handler() -> &'static str {
    "ok"
}

#[cfg(test)]
#[path = "health_test.rs"]
mod health_test;
