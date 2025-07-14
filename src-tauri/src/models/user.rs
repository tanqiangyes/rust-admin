use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, Default)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub avatar: Option<String>,
    pub role_id: i64,
    pub status: i32,
    pub failed_login_attempts: i32,
    pub last_failed_login: Option<DateTime<Utc>>,
    pub locked_until: Option<DateTime<Utc>>,
    pub last_login: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserWithRole {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub avatar: Option<String>,
    pub role_id: i64,
    pub role_name: String,
    pub permissions: String,
    pub status: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub avatar: Option<String>,
    pub role_id: i64,
    pub role_name: String,
    pub permissions: String,
    pub status: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub avatar: Option<String>,
    pub role_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub username: String,
    pub email: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub avatar: Option<String>,
    pub role_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserInfo {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub avatar: Option<String>,
    pub role_name: String,
    pub permissions: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LoginAttempt {
    pub id: i64,
    pub username: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub success: i32, // SQLite 中布尔值存储为整数
    pub failure_reason: Option<String>,
    pub created_at: DateTime<Utc>,
} 