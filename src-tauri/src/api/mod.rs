use serde::{Deserialize, Serialize};

pub mod auth;
pub mod users;
pub mod products;
pub mod orders;
pub mod categories;
pub mod roles;
pub mod stats;
pub mod settings;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
    pub message: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data,
            message: "操作成功".to_string(),
        }
    }

    pub fn error(message: String) -> Self
    where
        T: Default,
    {
        Self {
            success: false,
            data: T::default(),
            message,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
} 