use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub price: f64,
    pub description: Option<String>,
    pub category_id: Option<i64>,
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
    pub images: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub price: Option<f64>,
    pub description: Option<String>,
    pub category_id: Option<i64>,
    pub stock: Option<i32>,
    pub images: Option<Vec<String>>,
    pub status: Option<i32>,
} 