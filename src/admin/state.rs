use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use crate::config::AdminUser;

/// Recent redirect entry
#[derive(Clone, Debug)]
pub struct RecentRedirect {
    pub hashid: String,
    pub url: String,
    pub timestamp: u64,
}

/// User session
#[derive(Clone, Debug)]
pub struct Session {
    pub username: String,
    pub created_at: Instant,
}

/// Simulation state
#[derive(Default)]
pub struct SimulationState {
    pub running: AtomicBool,
    pub rps: AtomicU32,
}

/// Shared admin state
#[derive(Clone)]
pub struct AdminState {
    pub sessions: Arc<RwLock<HashMap<String, Session>>>,
    pub recent_redirects: Arc<RwLock<VecDeque<RecentRedirect>>>,
    pub users: Vec<AdminUser>,
    pub session_ttl: Duration,
    pub max_recent: usize,
    pub simulation: Arc<SimulationState>,
}

impl AdminState {
    pub fn new(session_ttl_hours: u64, users: Vec<AdminUser>) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            recent_redirects: Arc::new(RwLock::new(VecDeque::new())),
            users,
            session_ttl: Duration::from_secs(session_ttl_hours * 3600),
            max_recent: 50,
            simulation: Arc::new(SimulationState::default()),
        }
    }

    /// Start simulation with given RPS
    pub fn start_simulation(&self, rps: u32) {
        self.simulation.rps.store(rps, Ordering::SeqCst);
        self.simulation.running.store(true, Ordering::SeqCst);
    }

    /// Stop simulation
    pub fn stop_simulation(&self) {
        self.simulation.running.store(false, Ordering::SeqCst);
    }

    /// Check if simulation is running
    pub fn is_simulation_running(&self) -> bool {
        self.simulation.running.load(Ordering::SeqCst)
    }

    /// Get simulation RPS
    pub fn get_simulation_rps(&self) -> u32 {
        self.simulation.rps.load(Ordering::SeqCst)
    }

    /// Add a new session, returns session token
    pub async fn create_session(&self, username: String) -> String {
        let token = uuid::Uuid::new_v4().to_string();
        let session = Session {
            username,
            created_at: Instant::now(),
        };
        self.sessions.write().await.insert(token.clone(), session);
        token
    }

    /// Validate session token, returns username if valid
    pub async fn validate_session(&self, token: &str) -> Option<String> {
        let sessions = self.sessions.read().await;
        if let Some(session) = sessions.get(token) {
            if session.created_at.elapsed() < self.session_ttl {
                return Some(session.username.clone());
            }
        }
        None
    }

    /// Remove session
    pub async fn remove_session(&self, token: &str) {
        self.sessions.write().await.remove(token);
    }

    /// Clean up expired sessions
    pub async fn cleanup_sessions(&self) {
        let mut sessions = self.sessions.write().await;
        sessions.retain(|_, session| session.created_at.elapsed() < self.session_ttl);
    }

    /// Record a redirect
    pub async fn record_redirect(&self, hashid: String, url: String) {
        let entry = RecentRedirect {
            hashid,
            url,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        };

        let mut recent = self.recent_redirects.write().await;
        recent.push_front(entry);
        while recent.len() > self.max_recent {
            recent.pop_back();
        }
    }

    /// Get recent redirects
    pub async fn get_recent_redirects(&self) -> Vec<RecentRedirect> {
        self.recent_redirects.read().await.iter().cloned().collect()
    }

    /// Find user by username
    pub fn find_user(&self, username: &str) -> Option<&AdminUser> {
        self.users.iter().find(|u| u.username == username)
    }
}
