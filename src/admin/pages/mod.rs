mod dashboard;
mod login;

pub use dashboard::dashboard_page;
pub use login::login_page;

use minify_html::{minify, Cfg};
use std::sync::LazyLock;

pub fn minify_html(html: &str) -> String {
    static CFG: LazyLock<Cfg> = LazyLock::new(|| Cfg {
        minify_js: true,
        minify_css: true,
        ..Cfg::default()
    });
    String::from_utf8(minify(html.as_bytes(), &CFG)).unwrap_or_else(|_| html.to_string())
}
