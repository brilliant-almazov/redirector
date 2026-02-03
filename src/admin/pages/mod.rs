mod dashboard;
mod login;

pub use dashboard::dashboard_page;
pub use login::login_page;

pub fn minify_html(html: &str) -> String {
    crate::minify_html_str(html)
}
