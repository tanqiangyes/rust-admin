use serde::{Deserialize, Serialize};

pub mod auth;
pub mod users;
pub mod products;
pub mod orders;
pub mod categories;
pub mod roles;
pub mod settings;
pub mod stats;
pub mod logs;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            message: "操作成功".to_string(),
            data: Some(data),
        }
    }

    pub fn error(message: String) -> Self
    where
        T: Default,
    {
        Self {
            success: false,
            message,
            data: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,  // 注意这里是 items，不是 data
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i64,  // 添加这个字段
} 