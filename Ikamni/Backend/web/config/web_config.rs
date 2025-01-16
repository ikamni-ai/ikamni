use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebConfig {
    /// サーバー設定
    pub server: ServerConfig,
    /// メール設定
    pub email: EmailConfig,
    /// デモ環境設定
    pub demo: DemoConfig,
    /// セキュリティ設定
    pub security: SecurityConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub sender_email: String,
    pub template_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemoConfig {
    pub max_concurrent_sessions: usize,
    pub session_duration_days: u32,
    pub allowed_domains: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub jwt_secret: String,
    pub cors_allowed_origins: Vec<String>,
    pub rate_limit_requests: u32,
    pub rate_limit_duration_secs: u32,
} 