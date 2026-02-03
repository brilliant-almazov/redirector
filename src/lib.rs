pub mod admin;
pub mod config;
pub mod db;
pub mod error;
pub mod handlers;
pub mod metrics;
pub mod middleware;
pub mod services;

pub use admin::AdminState;
pub use config::{AdminUser, Config};
pub use error::{AppError, Result};
pub use services::{Cache, HashidDecoder, UrlStorage};

use minify_html::{minify, Cfg};
use std::sync::LazyLock;

static MINIFY_CFG: LazyLock<Cfg> = LazyLock::new(|| Cfg {
    minify_js: true,
    minify_css: true,
    ..Cfg::default()
});

pub fn minify_html_str(html: &str) -> String {
    String::from_utf8(minify(html.as_bytes(), &MINIFY_CFG)).unwrap_or_else(|_| html.to_string())
}
