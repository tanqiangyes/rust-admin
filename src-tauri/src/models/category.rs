use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, Default)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Default)]
pub struct CategoryWithParent {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
    pub parent_name: Option<String>,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub parent_id: Option<i64>,
    pub sort_order: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCategoryRequest {
    pub name: String,
    pub parent_id: Option<i64>,
    pub sort_order: i32,
} 