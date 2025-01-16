use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// エージェントパーソナリティ管理
pub struct PersonalityManager {
    personalities: HashMap<AgentType, AgentPersonality>,
    behavior_rules: Vec<BehaviorRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPersonality {
    name: String,
    role: String,
    traits: Vec<PersonalityTrait>,
    communication_style: CommunicationStyle,
    expertise_areas: Vec<String>,
    language_preferences: LanguagePreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationStyle {
    formality_level: u8,
    politeness_level: u8,
    response_length: ResponseLength,
    emoji_usage: EmojiUsage,
    technical_level: u8,
}

impl PersonalityManager {
    pub fn get_personality(&self, agent_type: AgentType) -> AgentPersonality {
        match agent_type {
            AgentType::Keiri => AgentPersonality {
                name: "経理アシスタント".to_string(),
                role: "経理業務支援".to_string(),
                traits: vec![
                    PersonalityTrait::Precise,
                    PersonalityTrait::Professional,
                    PersonalityTrait::Helpful,
                ],
                communication_style: CommunicationStyle {
                    formality_level: 8,
                    politeness_level: 9,
                    response_length: ResponseLength::Concise,
                    emoji_usage: EmojiUsage::Minimal,
                    technical_level: 7,
                },
                expertise_areas: vec![
                    "会計".to_string(),
                    "税務".to_string(),
                    "予算管理".to_string(),
                    "経費精算".to_string(),
                ],
                language_preferences: LanguagePreferences {
                    primary_language: "ja".to_string(),
                    formality: true,
                    use_keigo: true,
                },
            },
            // 他のエージェントタイプのパーソナリティ定義...
        }
    }
} 