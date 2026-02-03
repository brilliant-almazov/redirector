#[cfg(test)]
mod tests {
    use crate::handlers::redirect::{InterstitialTemplate, NotFoundTemplate};
    use askama::Template;

    #[test]
    fn test_interstitial_template_renders() {
        let template = InterstitialTemplate {
            target_url: "https://example.com/path".to_string(),
            target_domain: "example.com".to_string(),
            delay_seconds: 5,
        };

        let rendered = template.render().unwrap();
        assert!(rendered.contains("example.com"));
        assert!(rendered.contains("5"));
    }

    #[test]
    fn test_not_found_template_renders() {
        let template = NotFoundTemplate {};
        let rendered = template.render().unwrap();
        assert!(!rendered.is_empty());
    }

    #[test]
    fn test_interstitial_template_contains_button() {
        let template = InterstitialTemplate {
            target_url: "https://example.com".to_string(),
            target_domain: "example.com".to_string(),
            delay_seconds: 3,
        };

        let rendered = template.render().unwrap();
        assert!(rendered.contains("Go now") || rendered.contains("button"));
    }

    #[test]
    fn test_interstitial_template_contains_countdown() {
        let template = InterstitialTemplate {
            target_url: "https://example.com".to_string(),
            target_domain: "example.com".to_string(),
            delay_seconds: 10,
        };

        let rendered = template.render().unwrap();
        assert!(rendered.contains("10"));
    }

    #[test]
    fn test_interstitial_contains_target_url() {
        let template = InterstitialTemplate {
            target_url: "https://test.example.org/some/path".to_string(),
            target_domain: "test.example.org".to_string(),
            delay_seconds: 5,
        };

        let rendered = template.render().unwrap();
        assert!(rendered.contains("https://test.example.org/some/path"));
    }

    #[test]
    fn test_not_found_contains_404() {
        let template = NotFoundTemplate {};
        let rendered = template.render().unwrap();
        assert!(rendered.contains("404"));
    }
}
