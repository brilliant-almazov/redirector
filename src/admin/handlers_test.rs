#[cfg(test)]
mod tests {
    use axum::http::StatusCode;

    use crate::admin::state::AdminState;
    use crate::admin::{
        simulate_handler, simulation_start_handler, simulation_status_handler,
        simulation_stop_handler, SimulationStartRequest, SimulationStatusResponse,
    };
    use crate::config::AdminUser;
    use axum::extract::State;
    use axum::Json;

    fn create_test_state() -> AdminState {
        let users = vec![AdminUser {
            username: "admin".to_string(),
            password_hash: "$argon2id$v=19$m=19456,t=2,p=1$test$test".to_string(),
        }];
        AdminState::new(24, users)
    }

    #[tokio::test]
    async fn test_simulate_handler_returns_ok() {
        let status = simulate_handler().await;
        assert_eq!(status, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_simulate_handler_records_metrics() {
        let before = crate::metrics::get_total_requests();
        let _ = simulate_handler().await;
        let after = crate::metrics::get_total_requests();
        assert!(after > before);
    }

    #[tokio::test]
    async fn test_simulate_handler_multiple_calls() {
        for _ in 0..10 {
            let status = simulate_handler().await;
            assert_eq!(status, StatusCode::OK);
        }
    }

    #[tokio::test]
    async fn test_simulation_start_handler() {
        let state = create_test_state();
        let req = SimulationStartRequest { rps: 500 };

        let Json(response) = simulation_start_handler(State(state.clone()), Json(req)).await;

        assert!(response.running);
        assert_eq!(response.rps, 500);
        assert!(state.is_simulation_running());
        assert_eq!(state.get_simulation_rps(), 500);

        // Cleanup
        state.stop_simulation();
    }

    #[tokio::test]
    async fn test_simulation_start_handler_clamps_rps() {
        let state = create_test_state();

        // RPS too high should be clamped to 100000
        let req = SimulationStartRequest { rps: 999999 };
        let Json(response) = simulation_start_handler(State(state.clone()), Json(req)).await;
        assert_eq!(response.rps, 100000);

        // RPS 0 should be clamped to 1
        let req = SimulationStartRequest { rps: 0 };
        let Json(response) = simulation_start_handler(State(state.clone()), Json(req)).await;
        assert_eq!(response.rps, 1);

        state.stop_simulation();
    }

    #[tokio::test]
    async fn test_simulation_stop_handler() {
        let state = create_test_state();
        state.start_simulation(100);
        assert!(state.is_simulation_running());

        let Json(response) = simulation_stop_handler(State(state.clone())).await;

        assert!(!response.running);
        assert!(!state.is_simulation_running());
    }

    #[tokio::test]
    async fn test_simulation_status_handler_stopped() {
        let state = create_test_state();

        let Json(response) = simulation_status_handler(State(state)).await;

        assert!(!response.running);
        assert_eq!(response.rps, 0);
    }

    #[tokio::test]
    async fn test_simulation_status_handler_running() {
        let state = create_test_state();
        state.start_simulation(250);

        let Json(response) = simulation_status_handler(State(state.clone())).await;

        assert!(response.running);
        assert_eq!(response.rps, 250);

        state.stop_simulation();
    }

    #[test]
    fn test_simulation_start_request_debug() {
        let req = SimulationStartRequest { rps: 100 };
        let debug = format!("{:?}", req);
        assert!(debug.contains("SimulationStartRequest"));
        assert!(debug.contains("100"));
    }

    #[test]
    fn test_simulation_status_response_serialize() {
        let response = SimulationStatusResponse {
            running: true,
            rps: 500,
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"running\":true"));
        assert!(json.contains("\"rps\":500"));
    }
}
