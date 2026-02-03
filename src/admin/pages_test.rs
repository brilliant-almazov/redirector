#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn test_login_template_exists() {
        let content = fs::read_to_string("templates/login.html");
        assert!(content.is_ok(), "login.html should exist in templates/");
    }

    #[test]
    fn test_login_template_contains_form() {
        let content = fs::read_to_string("templates/login.html").unwrap();

        assert!(
            content.contains("<form"),
            "login.html should contain a form"
        );
        assert!(
            content.contains("action=\"/admin/login\""),
            "login.html should submit to /admin/login"
        );
        assert!(
            content.contains("method=\"POST\""),
            "login.html should use POST method"
        );
    }

    #[test]
    fn test_login_template_contains_inputs() {
        let content = fs::read_to_string("templates/login.html").unwrap();

        assert!(
            content.contains("name=\"username\""),
            "login.html should have username input"
        );
        assert!(
            content.contains("name=\"password\""),
            "login.html should have password input"
        );
        assert!(
            content.contains("type=\"password\""),
            "login.html should have password type input"
        );
    }

    #[test]
    fn test_login_template_has_theme_switcher() {
        let content = fs::read_to_string("templates/login.html").unwrap();

        assert!(
            content.contains("applyTheme"),
            "login.html should have theme switching function"
        );
        assert!(
            content.contains("data-theme=\"light\""),
            "login.html should have light theme option"
        );
        assert!(
            content.contains("data-theme=\"dark\""),
            "login.html should have dark theme option"
        );
        assert!(
            content.contains("data-theme=\"warm\""),
            "login.html should have warm theme option"
        );
    }

    #[test]
    fn test_dashboard_template_exists() {
        let content = fs::read_to_string("templates/dashboard.html");
        assert!(content.is_ok(), "dashboard.html should exist in templates/");
    }

    #[test]
    fn test_dashboard_template_contains_charts() {
        let content = fs::read_to_string("templates/dashboard.html").unwrap();

        assert!(
            content.contains("rpsChart"),
            "dashboard.html should have RPS chart"
        );
        assert!(
            content.contains("latencyChart"),
            "dashboard.html should have latency chart"
        );
        assert!(
            content.contains("Chart.js") || content.contains("chart.min.js"),
            "dashboard.html should include Chart.js"
        );
    }

    #[test]
    fn test_dashboard_template_contains_stats() {
        let content = fs::read_to_string("templates/dashboard.html").unwrap();

        assert!(
            content.contains("stat-uptime"),
            "dashboard.html should have uptime stat"
        );
        assert!(
            content.contains("stat-cpu"),
            "dashboard.html should have CPU stat"
        );
        assert!(
            content.contains("stat-memory"),
            "dashboard.html should have memory stat"
        );
        assert!(
            content.contains("stat-rps"),
            "dashboard.html should have RPS stat"
        );
        assert!(
            content.contains("stat-cache"),
            "dashboard.html should have cache stat"
        );
    }

    #[test]
    fn test_dashboard_template_has_sse_connection() {
        let content = fs::read_to_string("templates/dashboard.html").unwrap();

        assert!(
            content.contains("EventSource"),
            "dashboard.html should use SSE"
        );
        assert!(
            content.contains("/admin/events"),
            "dashboard.html should connect to /admin/events"
        );
    }

    #[test]
    fn test_dashboard_template_has_simulation_controls() {
        let content = fs::read_to_string("templates/dashboard.html").unwrap();

        assert!(
            content.contains("sim-rps"),
            "dashboard.html should have simulation RPS control"
        );
        assert!(
            content.contains("toggleSimulation"),
            "dashboard.html should have simulation toggle"
        );
        assert!(
            content.contains("/admin/simulate"),
            "dashboard.html should call /admin/simulate"
        );
    }

    #[test]
    fn test_dashboard_template_has_theme_switcher() {
        let content = fs::read_to_string("templates/dashboard.html").unwrap();

        assert!(
            content.contains("applyTheme"),
            "dashboard.html should have theme switching function"
        );
        assert!(
            content.contains("data-theme=\"light\""),
            "dashboard.html should have light theme option"
        );
        assert!(
            content.contains("data-theme=\"dark\""),
            "dashboard.html should have dark theme option"
        );
        assert!(
            content.contains("data-theme=\"warm\""),
            "dashboard.html should have warm theme option"
        );
    }

    #[test]
    fn test_dashboard_template_has_logout() {
        let content = fs::read_to_string("templates/dashboard.html").unwrap();

        assert!(
            content.contains("/admin/logout"),
            "dashboard.html should have logout link"
        );
    }

    #[test]
    fn test_dashboard_template_has_css_variables() {
        let content = fs::read_to_string("templates/dashboard.html").unwrap();

        assert!(
            content.contains("--bg-primary"),
            "dashboard.html should use CSS variables for theming"
        );
        assert!(
            content.contains("--accent"),
            "dashboard.html should have accent color variable"
        );
        assert!(
            content.contains(".theme-warm"),
            "dashboard.html should have warm theme styles"
        );
    }

    #[test]
    fn test_login_template_has_css_variables() {
        let content = fs::read_to_string("templates/login.html").unwrap();

        assert!(
            content.contains("--bg-primary"),
            "login.html should use CSS variables for theming"
        );
        assert!(
            content.contains(".theme-warm"),
            "login.html should have warm theme styles"
        );
    }

    #[test]
    fn test_templates_use_same_theme_names() {
        let login = fs::read_to_string("templates/login.html").unwrap();
        let dashboard = fs::read_to_string("templates/dashboard.html").unwrap();

        // Both should use the same theme names for consistency
        assert!(login.contains("'light'") && dashboard.contains("'light'"));
        assert!(login.contains("'dark'") && dashboard.contains("'dark'"));
        assert!(login.contains("'warm'") && dashboard.contains("'warm'"));
    }

    #[test]
    fn test_templates_use_tailwind() {
        let login = fs::read_to_string("templates/login.html").unwrap();
        let dashboard = fs::read_to_string("templates/dashboard.html").unwrap();

        assert!(
            login.contains("tailwind"),
            "login.html should use Tailwind CSS"
        );
        assert!(
            dashboard.contains("tailwind"),
            "dashboard.html should use Tailwind CSS"
        );
    }
}
