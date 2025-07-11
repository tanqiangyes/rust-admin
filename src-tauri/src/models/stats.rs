use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_users: i64,
    pub total_products: i64,
    pub total_orders: i64,
    pub total_categories: i64,
    pub today_orders: i64,
    pub today_sales: f64,
    pub monthly_sales: Vec<MonthlySales>,
    pub order_status_stats: Vec<OrderStatusStats>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MonthlySales {
    pub month: String,
    pub sales: f64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct OrderStatusStats {
    pub status: i32,
    pub count: i64,
    pub name: String,
} 