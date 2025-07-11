use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: 0,
            username: String::new(),
            email: String::new(),
            password_hash: String::new(),
            phone: None,
            address: None,
            avatar: None,
            role_id: 3, // 默认为普通用户
            status: 1,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserWithRole {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub avatar: Option<String>,
    pub role_id: i64,
    pub role_name: String,
    pub status: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for UserWithRole {
    fn default() -> Self {
        Self {
            id: 0,
            username: String::new(),
            email: String::new(),
            phone: None,
            address: None,
            avatar: None,
            role_id: 3,
            role_name: "普通用户".to_string(),
            status: 1,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
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
    pub status: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub avatar: Option<String>,
    pub role_id: Option<i64>,
    pub status: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserWithRole,
}

impl Default for LoginResponse {
    fn default() -> Self {
        Self {
            token: String::new(),
            user: UserWithRole::default(),
        }
    }
} 