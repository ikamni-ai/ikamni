use crate::agents::{AgentBehavior, AgentError};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use uuid::Uuid;

/// 会話管理システム
pub struct KaiwaKanri {
    active_sessions: HashMap<String, KaiwaSession>,
    message_queue: mpsc::Sender<KaiwaMessage>,
    context_manager: KaiwaContextManager,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KaiwaSession {
    session_id: String,
    user_id: String,
    agent_type: AgentType,
    context: KaiwaContext,
    started_at: chrono::DateTime<chrono::Utc>,
    last_activity: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KaiwaContext {
    /// 会話履歴
    messages: Vec<KaiwaMessage>,
    /// 現在の文脈
    current_topic: Option<String>,
    /// ユーザー設定
    user_preferences: HashMap<String, String>,
    /// エージェント状態
    agent_state: AgentState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KaiwaMessage {
    id: Uuid,
    session_id: String,
    sender_type: SenderType,
    content: String,
    attachments: Vec<Attachment>,
    timestamp: chrono::DateTime<chrono::Utc>,
    metadata: HashMap<String, serde_json::Value>,
}

impl KaiwaKanri {
    pub async fn process_message(&mut self, message: KaiwaMessage) -> Result<KaiwaMessage, KaiwaError> {
        let session = self.get_or_create_session(&message.session_id)?;
        
        // コンテキストの更新
        self.context_manager.update_context(&mut session.context, &message);
        
        // エージェントによるメッセージ処理
        let response = match session.agent_type {
            AgentType::Keiri => self.process_keiri_message(&session, &message).await?,
            AgentType::Jinji => self.process_jinji_message(&session, &message).await?,
            AgentType::Eigyo => self.process_eigyo_message(&session, &message).await?,
            // 他のエージェントタイプ...
        };

        Ok(response)
    }
} 