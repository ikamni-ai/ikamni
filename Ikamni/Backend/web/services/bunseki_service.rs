use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use std::collections::HashMap;

/// 分析サービス
pub struct BunsekiService {
    event_sender: mpsc::Sender<BunsekiEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BunsekiEvent {
    event_shubetsu: String,
    jikoku: chrono::DateTime<chrono::Utc>,
    user_id: Option<String>,
    zokusei: HashMap<String, serde_json::Value>,
}

impl BunsekiService {
    pub async fn track_shiyou_moushikomi(&self, moushikomi: &ShiyouMoushikomi) {
        let event = BunsekiEvent {
            event_shubetsu: "shiyou_moushikomi".to_string(),
            jikoku: chrono::Utc::now(),
            user_id: Some(moushikomi.kaisha_mail.clone()),
            zokusei: HashMap::from([
                ("kaisha_mei".to_string(), json!(moushikomi.kaisha_mei)),
                ("kibou_agent".to_string(), json!(moushikomi.kibou_agent)),
            ]),
        };

        if let Err(e) = self.event_sender.send(event).await {
            log::error!("分析イベント送信失敗: {}", e);
        }
    }
} 