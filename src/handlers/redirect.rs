use crate::error::AppError;
use crate::services::{Cache, HashidDecoder, UrlResolver, UrlStorage};
use askama::Template;
use axum::{
    extract::{Path, State},
    response::Html,
};
use std::sync::Arc;

#[derive(Template)]
#[template(path = "interstitial.html")]
pub struct InterstitialTemplate {
    pub target_url: String,
    pub target_domain: String,
    pub delay_seconds: u32,
}

#[derive(Template)]
#[template(path = "not_found.html")]
pub struct NotFoundTemplate {}

pub struct RedirectState<H, C, S>
where
    H: HashidDecoder,
    C: Cache,
    S: UrlStorage,
{
    pub resolver: Arc<UrlResolver<H, C, S>>,
    pub delay_seconds: u32,
}

pub async fn redirect_handler<H, C, S>(
    State(state): State<Arc<RedirectState<H, C, S>>>,
    Path(hashid): Path<String>,
) -> Result<Html<String>, AppError>
where
    H: HashidDecoder + 'static,
    C: Cache + 'static,
    S: UrlStorage + 'static,
{
    metrics::counter!("redirect_requests").increment(1);

    match state.resolver.resolve(&hashid).await {
        Ok(resolved) => {
            tracing::info!(
                hashid = %hashid,
                target = %resolved.full_url,
                "Redirect resolved"
            );

            let template = InterstitialTemplate {
                target_url: resolved.full_url,
                target_domain: resolved.domain,
                delay_seconds: state.delay_seconds,
            };

            Ok(Html(
                template
                    .render()
                    .map_err(|e| AppError::Internal(e.into()))?,
            ))
        }
        Err(AppError::NotFound | AppError::InvalidHashid) => {
            tracing::info!(hashid = %hashid, "URL not found");
            metrics::counter!("not_found_requests").increment(1);
            Err(AppError::NotFound)
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
#[path = "redirect_test.rs"]
mod redirect_test;
