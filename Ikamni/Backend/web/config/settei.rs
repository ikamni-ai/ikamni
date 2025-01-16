use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSettei {
    /// サーバー設定
    pub server: ServerSettei,
    /// メール設定
    pub mail: MailSettei,
    /// 試用環境設定
    pub shiyou: ShiyouSettei,
    /// セキュリティ設定
    pub security: SecuritySettei,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSettei {
    pub host: String,
    pub port: u16,
    pub worker_kazu: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailSettei {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_pass: String,
    pub sender_mail: String,
    pub template_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShiyouSettei {
    pub max_douji_session: usize,
    pub session_kikan_days: u32,
    pub kyoka_domain: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySettei {
    pub jwt_secret: String,
    pub cors_kyoka_origin: Vec<String>,
    pub rate_limit_yokyu: u32,
    pub rate_limit_kikan_byou: u32,
} 