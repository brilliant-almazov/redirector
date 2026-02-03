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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minify_html_removes_whitespace() {
        let html = "<html>  <body>   <p>  hello  </p>  </body>  </html>";
        let result = minify_html_str(html);
        assert!(result.len() < html.len());
        assert!(result.contains("hello"));
    }

    #[test]
    fn test_minify_html_empty_string() {
        let result = minify_html_str("");
        assert!(result.is_empty());
    }

    #[test]
    fn test_minify_html_preserves_content() {
        let html = "<p>Important content</p>";
        let result = minify_html_str(html);
        assert!(result.contains("Important content"));
    }

    #[test]
    fn test_minify_html_handles_css() {
        let html = "<style>  body {  color: red;  }  </style>";
        let result = minify_html_str(html);
        assert!(result.contains("color"));
    }

    #[test]
    fn test_minify_html_handles_js() {
        let html = "<script>  var x = 1;  </script>";
        let result = minify_html_str(html);
        assert!(result.contains("x"));
    }
}
