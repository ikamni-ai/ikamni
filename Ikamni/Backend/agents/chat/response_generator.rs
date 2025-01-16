use crate::templates::TemplateEngine;
use serde::{Deserialize, Serialize};

/// レスポンス生成システム
pub struct ResponseGenerator {
    template_engine: TemplateEngine,
    language_model: LanguageModel,
    personality_manager: PersonalityManager,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseTemplate {
    template_id: String,
    content: String,
    variables: Vec<String>,
    conditions: Vec<TemplateCondition>,
    style: ResponseStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseStyle {
    formality_level: FormalityLevel,
    tone: Tone,
    language: Language,
    honorifics: bool,
}

impl ResponseGenerator {
    pub async fn generate(&self, processed: ProcessedMessage) -> Result<KaiwaMessage, GenerationError> {
        // パーソナリティの取得
        let personality = self.personality_manager.get_personality(processed.agent_type);
        
        // テンプレートの選択
        let template = self.select_template(&processed, &personality)?;
        
        // 変数の置換
        let content = self.template_engine.render(&template, &processed)?;
        
        // 言語モデルによる洗練
        let refined_content = self.language_model.refine(content, &personality).await?;
        
        Ok(KaiwaMessage {
            id: Uuid::new_v4(),
            session_id: processed.original.session_id,
            sender_type: SenderType::Agent,
            content: refined_content,
            attachments: vec![],
            timestamp: chrono::Utc::now(),
            metadata: self.generate_metadata(&processed),
        })
    }

    fn select_template(&self, processed: &ProcessedMessage, personality: &AgentPersonality) -> Result<ResponseTemplate, GenerationError> {
        let templates = match processed.intent.primary.as_str() {
            "予算照会" => self.template_engine.get_templates("budget_query"),
            "経費申請" => self.template_engine.get_templates("expense_claim"),
            "決算報告" => self.template_engine.get_templates("financial_report"),
            _ => self.template_engine.get_templates("general"),
        }?;

        // 最適なテンプレートの選択
        templates.into_iter()
            .filter(|t| self.matches_conditions(t, processed))
            .max_by_key(|t| self.calculate_template_score(t, processed, personality))
            .ok_or(GenerationError::NoSuitableTemplate)
    }
} 