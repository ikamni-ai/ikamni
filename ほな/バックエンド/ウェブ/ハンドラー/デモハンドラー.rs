use crate::agents::AgentManager;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

/// デモ環境管理ハンドラー
pub struct DemoHandler {
    agent_manager: AgentManager,
    demo_sessions: HashMap<String, DemoSession>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemoSession {
    session_id: String,
    company_email: String,
    active_agents: Vec<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    expires_at: chrono::DateTime<chrono::Utc>,
}

impl DemoHandler {
    pub async fn create_demo_session(&mut self, request: DemoRequest) -> Result<DemoSession, DemoError> {
        let session = DemoSession {
            session_id: Uuid::new_v4().to_string(),
            company_email: request.company_email,
            active_agents: request.requested_agents,
            created_at: chrono::Utc::now(),
            expires_at: chrono::Utc::now() + chrono::Duration::days(14),
        };

        self.demo_sessions.insert(session.session_id.clone(), session.clone());
        Ok(session)
    }
} 