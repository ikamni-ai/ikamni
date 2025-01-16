use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

/// アナリティクスサービス
pub struct AnalyticsService {
    event_sender: mpsc::Sender<AnalyticsEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEvent {
    event_type: String,
    timestamp: chrono::DateTime<chrono::Utc>,
    user_id: Option<String>,
    properties: HashMap<String, serde_json::Value>,
}

impl AnalyticsService {
    pub async fn track_demo_request(&self, request: &DemoRequest) {
        let event = AnalyticsEvent {
            event_type: "demo_requested".to_string(),
            timestamp: chrono::Utc::now(),
            user_id: Some(request.company_email.clone()),
            properties: HashMap::from([
                ("company_name".to_string(), json!(request.company_name)),
                ("requested_agents".to_string(), json!(request.requested_agents)),
            ]),
        };

        if let Err(e) = self.event_sender.send(event).await {
            log::error!("Failed to send analytics event: {}", e);
        }
    }
} 