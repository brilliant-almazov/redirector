#[cfg(test)]
mod tests {
    use crate::admin::sse::{
        AppMetrics, MetricsPayload, RecentRedirectJson, SimulationStatus, SystemMetrics,
    };

    #[test]
    fn test_system_metrics_serialize() {
        let metrics = SystemMetrics {
            uptime_secs: 3600,
            cpu_percent: 45.5,
            memory_mb: 512,
        };
        let json = serde_json::to_string(&metrics).unwrap();
        assert!(json.contains("\"uptime_secs\":3600"));
        assert!(json.contains("\"cpu_percent\":45.5"));
        assert!(json.contains("\"memory_mb\":512"));
    }

    #[test]
    fn test_app_metrics_serialize() {
        let metrics = AppMetrics {
            rps: 1500.5,
            latency_p50_ms: 10.0,
            latency_p95_ms: 25.0,
            latency_p99_ms: 50.0,
            cache_hit_rate: 0.95,
            total_requests: 100000,
        };
        let json = serde_json::to_string(&metrics).unwrap();
        assert!(json.contains("\"rps\":1500.5"));
        assert!(json.contains("\"cache_hit_rate\":0.95"));
        assert!(json.contains("\"total_requests\":100000"));
    }

    #[test]
    fn test_recent_redirect_json_serialize() {
        let redirect = RecentRedirectJson {
            hashid: "abc123".to_string(),
            url: "https://example.com".to_string(),
            at: 1700000000,
        };
        let json = serde_json::to_string(&redirect).unwrap();
        assert!(json.contains("\"hashid\":\"abc123\""));
        assert!(json.contains("\"url\":\"https://example.com\""));
        assert!(json.contains("\"at\":1700000000"));
    }

    #[test]
    fn test_simulation_status_serialize() {
        let status = SimulationStatus {
            running: true,
            rps: 500,
        };
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("\"running\":true"));
        assert!(json.contains("\"rps\":500"));
    }

    #[test]
    fn test_metrics_payload_serialize() {
        let payload = MetricsPayload {
            timestamp: 1700000000,
            system: SystemMetrics {
                uptime_secs: 100,
                cpu_percent: 10.0,
                memory_mb: 256,
            },
            app: AppMetrics {
                rps: 0.0,
                latency_p50_ms: 0.0,
                latency_p95_ms: 0.0,
                latency_p99_ms: 0.0,
                cache_hit_rate: 0.0,
                total_requests: 0,
            },
            recent: vec![],
            simulation: SimulationStatus {
                running: false,
                rps: 0,
            },
        };
        let json = serde_json::to_string(&payload).unwrap();
        assert!(json.contains("\"timestamp\":1700000000"));
        assert!(json.contains("\"system\""));
        assert!(json.contains("\"app\""));
        assert!(json.contains("\"recent\":[]"));
        assert!(json.contains("\"simulation\""));
    }

    #[test]
    fn test_metrics_payload_with_recent_redirects() {
        let payload = MetricsPayload {
            timestamp: 1700000000,
            system: SystemMetrics {
                uptime_secs: 100,
                cpu_percent: 10.0,
                memory_mb: 256,
            },
            app: AppMetrics {
                rps: 100.0,
                latency_p50_ms: 5.0,
                latency_p95_ms: 15.0,
                latency_p99_ms: 30.0,
                cache_hit_rate: 0.95,
                total_requests: 5000,
            },
            recent: vec![
                RecentRedirectJson {
                    hashid: "first".to_string(),
                    url: "https://first.com".to_string(),
                    at: 1700000001,
                },
                RecentRedirectJson {
                    hashid: "second".to_string(),
                    url: "https://second.com".to_string(),
                    at: 1700000002,
                },
            ],
            simulation: SimulationStatus {
                running: true,
                rps: 500,
            },
        };
        let json = serde_json::to_string(&payload).unwrap();
        assert!(json.contains("\"first\""));
        assert!(json.contains("\"second\""));
    }
}
