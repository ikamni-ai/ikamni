use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 会話コンテキスト管理
pub struct KaiwaContextManager {
    context_store: HashMap<String, KaiwaContext>,
    memory_manager: MemoryManager,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryManager {
    short_term: Vec<ContextMemory>,
    long_term: Vec<ContextMemory>,
    importance_threshold: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMemory {
    content: String,
    importance: f32,
    created_at: chrono::DateTime<chrono::Utc>,
    last_accessed: chrono::DateTime<chrono::Utc>,
    access_count: u32,
    related_topics: Vec<String>,
}

impl KaiwaContextManager {
    pub fn update_context(&mut self, context: &mut KaiwaContext, message: &KaiwaMessage) {
        // 短期記憶の更新
        self.memory_manager.add_to_short_term(ContextMemory {
            content: message.content.clone(),
            importance: self.calculate_importance(message),
            created_at: chrono::Utc::now(),
            last_accessed: chrono::Utc::now(),
            access_count: 1,
            related_topics: self.extract_topics(&message.content),
        });

        // 長期記憶への転送判断
        self.memory_manager.transfer_to_long_term();
        
        // コンテキストの更新
        context.messages.push(message.clone());
        context.current_topic = self.determine_current_topic(context);
    }

    fn calculate_importance(&self, message: &KaiwaMessage) -> f32 {
        // 重要度計算ロジック
        let base_importance = 0.5;
        let mut importance = base_importance;

        // キーワードベースの重要度調整
        if message.content.contains("緊急") || message.content.contains("重要") {
            importance += 0.3;
        }

        // メタデータベースの重要度調整
        if let Some(priority) = message.metadata.get("priority") {
            if let Some(priority_value) = priority.as_f64() {
                importance += priority_value as f32 * 0.2;
            }
        }

        importance.min(1.0)
    }
} 