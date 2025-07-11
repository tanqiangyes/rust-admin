use tauri::State;
use crate::AppState;
use crate::api::ApiResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_users: i64,
    pub total_products: i64,
    pub total_orders: i64,
    pub total_revenue: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub uptime: String,
    pub user_count: i64,
    pub product_count: i64,
    pub order_count: i64,
}

#[tauri::command]
pub async fn get_dashboard_stats(
    state: State<'_, AppState>,
) -> Result<ApiResponse<DashboardStats>, String> {
    let total_users = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| e.to_string())?;

    let total_products = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM products")
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| e.to_string())?;

    let total_orders = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM orders")
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| e.to_string())?;

    let total_revenue = sqlx::query_scalar::<_, Option<f64>>("SELECT SUM(total_amount) FROM orders WHERE status = 'delivered'")
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| e.to_string())?
        .unwrap_or(0.0);

    Ok(ApiResponse::success(DashboardStats {
        total_users,
        total_products,
        total_orders,
        total_revenue,
    }))
}

#[tauri::command]
pub async fn get_system_info(
    state: State<'_, AppState>,
) -> Result<ApiResponse<SystemInfo>, String> {
    let user_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| e.to_string())?;

    let product_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM products")
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| e.to_string())?;

    let order_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM orders")
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(SystemInfo {
        uptime: "1天2小时30分钟".to_string(), // 这里可以计算真实的运行时间
        user_count,
        product_count,
        order_count,
    }))
} 