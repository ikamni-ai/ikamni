use crate::agents::AgentKanrisha;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 試用環境管理
pub struct ShiyouKanri {
    agent_kanrisha: AgentKanrisha,
    shiyou_session: HashMap<String, ShiyouSession>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShiyouSession {
    session_id: String,
    kaisha_mail: String,
    koudou_agent: Vec<String>,
    sakusei_datetime: chrono::DateTime<chrono::Utc>,
    shiyou_kigen: chrono::DateTime<chrono::Utc>,
}

impl ShiyouKanri {
    pub async fn create_shiyou_session(&mut self, moushikomi: ShiyouMoushikomi) -> Result<ShiyouSession, ShiyouError> {
        let session = ShiyouSession {
            session_id: Uuid::new_v4().to_string(),
            kaisha_mail: moushikomi.kaisha_mail,
            koudou_agent: moushikomi.kibou_agent,
            sakusei_datetime: chrono::Utc::now(),
            shiyou_kigen: chrono::Utc::now() + chrono::Duration::days(14),
        };

        self.shiyou_session.insert(session.session_id.clone(), session.clone());
        Ok(session)
    }
} 
