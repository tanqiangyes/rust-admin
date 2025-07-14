use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, Default)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub price: f64,
    pub description: Option<String>,
    pub category_id: Option<i64>,
    pub stock: i32,
    pub images: Option<String>, // 存储为 JSON 字符串
    pub status: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProductWithCategory {
    pub id: i64,
    pub name: String,
    pub price: f64,
    pub description: Option<String>,
    pub category_id: Option<i64>,
    pub category_name: Option<String>,
    pub stock: i32,
    pub images: Option<String>,
    pub status: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub price: f64,
    pub description: Option<String>,
    pub category_id: Option<i64>,
    pub stock: i32,
    pub images: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProductRequest {
    pub name: String,
    pub price: f64,
    pub description: Option<String>,
    pub category_id: Option<i64>,
    pub stock: i32,
    pub images: Vec<String>,
    pub status: i32,
} 