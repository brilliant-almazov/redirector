#[cfg(test)]
mod tests {
    use crate::admin::state::AdminState;
    use crate::config::AdminUser;

    fn create_test_state() -> AdminState {
        let users = vec![AdminUser {
            username: "admin".to_string(),
            password_hash: "$argon2id$v=19$m=19456,t=2,p=1$test$test".to_string(),
        }];
        AdminState::new(24, users)
    }

    #[tokio::test]
    async fn test_create_session() {
        let state = create_test_state();
        let token = state.create_session("admin".to_string()).await;

        assert!(!token.is_empty());
        assert_eq!(token.len(), 36); // UUID format
    }

    #[tokio::test]
    async fn test_validate_session() {
        let state = create_test_state();
        let token = state.create_session("admin".to_string()).await;

        let username = state.validate_session(&token).await;
        assert_eq!(username, Some("admin".to_string()));
    }

    #[tokio::test]
    async fn test_validate_invalid_session() {
        let state = create_test_state();

        let username = state.validate_session("invalid-token").await;
        assert_eq!(username, None);
    }

    #[tokio::test]
    async fn test_remove_session() {
        let state = create_test_state();
        let token = state.create_session("admin".to_string()).await;

        // Session exists
        assert!(state.validate_session(&token).await.is_some());

        // Remove session
        state.remove_session(&token).await;

        // Session no longer exists
        assert!(state.validate_session(&token).await.is_none());
    }

    #[tokio::test]
    async fn test_record_redirect() {
        let state = create_test_state();

        state
            .record_redirect("abc123".to_string(), "https://example.com".to_string())
            .await;

        let recent = state.get_recent_redirects().await;
        assert_eq!(recent.len(), 1);
        assert_eq!(recent[0].hashid, "abc123");
        assert_eq!(recent[0].url, "https://example.com");
    }

    #[tokio::test]
    async fn test_recent_redirects_order() {
        let state = create_test_state();

        state
            .record_redirect("first".to_string(), "https://first.com".to_string())
            .await;
        state
            .record_redirect("second".to_string(), "https://second.com".to_string())
            .await;
        state
            .record_redirect("third".to_string(), "https://third.com".to_string())
            .await;

        let recent = state.get_recent_redirects().await;
        assert_eq!(recent.len(), 3);
        // Most recent first
        assert_eq!(recent[0].hashid, "third");
        assert_eq!(recent[1].hashid, "second");
        assert_eq!(recent[2].hashid, "first");
    }

    #[tokio::test]
    async fn test_find_user() {
        let state = create_test_state();

        let user = state.find_user("admin");
        assert!(user.is_some());
        assert_eq!(user.unwrap().username, "admin");

        let not_found = state.find_user("nonexistent");
        assert!(not_found.is_none());
    }

    #[tokio::test]
    async fn test_multiple_sessions() {
        let state = create_test_state();

        let token1 = state.create_session("admin".to_string()).await;
        let token2 = state.create_session("admin".to_string()).await;

        // Both tokens are different
        assert_ne!(token1, token2);

        // Both sessions are valid
        assert!(state.validate_session(&token1).await.is_some());
        assert!(state.validate_session(&token2).await.is_some());
    }

    #[tokio::test]
    async fn test_cleanup_sessions_keeps_valid() {
        let state = create_test_state();
        let token = state.create_session("admin".to_string()).await;

        // Cleanup should keep valid session
        state.cleanup_sessions().await;

        assert!(state.validate_session(&token).await.is_some());
    }

    #[test]
    fn test_start_simulation() {
        let state = create_test_state();
        assert!(!state.is_simulation_running());
        assert_eq!(state.get_simulation_rps(), 0);

        state.start_simulation(500);
        assert!(state.is_simulation_running());
        assert_eq!(state.get_simulation_rps(), 500);
    }

    #[test]
    fn test_stop_simulation() {
        let state = create_test_state();
        state.start_simulation(100);
        assert!(state.is_simulation_running());

        state.stop_simulation();
        assert!(!state.is_simulation_running());
    }

    #[test]
    fn test_simulation_rps_update() {
        let state = create_test_state();
        state.start_simulation(100);
        assert_eq!(state.get_simulation_rps(), 100);

        // Start again with different RPS
        state.start_simulation(999);
        assert_eq!(state.get_simulation_rps(), 999);
        assert!(state.is_simulation_running());
    }

    #[test]
    fn test_simulation_default_state() {
        let state = create_test_state();
        assert!(!state.is_simulation_running());
        assert_eq!(state.get_simulation_rps(), 0);
    }

    #[tokio::test]
    async fn test_recent_redirects_max_capacity() {
        let state = create_test_state();

        // Fill beyond max_recent (50)
        for i in 0..60 {
            state
                .record_redirect(format!("hash{}", i), format!("https://url{}.com", i))
                .await;
        }

        let recent = state.get_recent_redirects().await;
        assert_eq!(recent.len(), 50);
        // Most recent should be hash59
        assert_eq!(recent[0].hashid, "hash59");
    }

    #[tokio::test]
    async fn test_empty_recent_redirects() {
        let state = create_test_state();
        let recent = state.get_recent_redirects().await;
        assert!(recent.is_empty());
    }

    #[test]
    fn test_admin_state_clone() {
        let state = create_test_state();
        state.start_simulation(42);

        let cloned = state.clone();
        assert!(cloned.is_simulation_running());
        assert_eq!(cloned.get_simulation_rps(), 42);

        // They share the same underlying state
        cloned.stop_simulation();
        assert!(!state.is_simulation_running());
    }

    #[test]
    fn test_find_user_not_found() {
        let state = create_test_state();
        assert!(state.find_user("nobody").is_none());
    }
}
