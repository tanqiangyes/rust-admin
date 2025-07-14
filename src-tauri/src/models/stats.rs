use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_users: i64,
    pub total_products: i64,
    pub total_orders: i64,
    pub total_categories: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub system_name: String,
    pub system_version: String,
    pub system_description: String,
    pub rust_version: String,
    pub tauri_version: String,
} 