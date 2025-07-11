use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SystemSetting {
    pub id: i64,
    pub setting_key: String,
    pub setting_value: String,
    pub setting_type: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemSettings {
    pub system_name: String,
    pub system_description: String,
    pub system_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UISettings {
    pub theme_color: String,
    pub language: String,
    pub page_size: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecuritySettings {
    pub enable_registration: bool,
    pub session_timeout: i32,
    pub max_login_attempts: i32,
    pub maintenance_mode: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllSettings {
    pub system: SystemSettings,
    pub ui: UISettings,
    pub security: SecuritySettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSettingRequest {
    pub setting_key: String,
    pub setting_value: String,
} 