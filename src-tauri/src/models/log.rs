use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Log {
    pub id: i64,
    pub user_id: Option<i64>,
    pub action: String,
    pub description: String,
    pub ip_address: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLogRequest {
    pub user_id: Option<i64>,
    pub action: String,
    pub description: String,
    pub ip_address: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LogWithUser {
    pub id: i64,
    pub user_id: Option<i64>,
    pub username: Option<String>,
    pub action: String,
    pub resource: String,
    pub details: Option<String>,
    pub created_at: DateTime<Utc>,
} 