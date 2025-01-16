use crate::agents::AgentBehavior;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

/// メッセージ処理システム
pub struct MessageProcessor {
    processors: HashMap<AgentType, Box<dyn MessageHandler>>,
    nlp_engine: NLPEngine,
    response_generator: ResponseGenerator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedMessage {
    original: KaiwaMessage,
    intent: MessageIntent,
    entities: Vec<Entity>,
    sentiment: Sentiment,
    priority: Priority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageIntent {
    primary: String,
    confidence: f32,
    secondary: Vec<(String, f32)>,
}

impl MessageProcessor {
    pub async fn process(&self, message: KaiwaMessage, agent_type: AgentType) -> Result<ProcessedMessage, ProcessError> {
        // 自然言語処理
        let nlp_result = self.nlp_engine.analyze(&message.content).await?;
        
        // エージェント固有の処理
        let processor = self.processors.get(&agent_type)
            .ok_or(ProcessError::UnsupportedAgent)?;
            
        let processed = processor.process(message, nlp_result).await?;

        // レスポンス生成
        let response = self.response_generator.generate(processed).await?;

        Ok(response)
    }

    async fn handle_keiri_message(&self, message: &ProcessedMessage) -> Result<KaiwaMessage, ProcessError> {
        match message.intent.primary.as_str() {
            "予算照会" => self.handle_budget_query(message).await,
            "経費申請" => self.handle_expense_claim(message).await,
            "決算報告" => self.handle_financial_report(message).await,
            _ => self.handle_general_query(message).await,
        }
    }
} 