use axum::http::header::HeaderValue;
use axum::response::Response;

static VERSION: HeaderValue = HeaderValue::from_static(concat!(
    env!("CARGO_PKG_VERSION"),
    "+",
    env!("GIT_COMMIT_SHORT")
));

pub async fn set_version_header(mut response: Response) -> Response {
    response.headers_mut().insert("X-Version", VERSION.clone());
    response
}

#[cfg(test)]
#[path = "version_header_test.rs"]
mod version_header_test;
