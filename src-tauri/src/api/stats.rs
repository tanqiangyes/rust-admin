use tauri::State;
use crate::{AppState, models::stats::*};
use crate::api::ApiResponse;

#[tauri::command]
pub async fn get_dashboard_stats(
    state: State<'_, AppState>,
) -> Result<ApiResponse<DashboardStats>, String> {
    let db = state.db.lock().await;
    
    let total_users = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
        .fetch_one(&db.pool)
        .await
        .map_err(|e| e.to_string())?;

    let total_products = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM products")
        .fetch_one(&db.pool)
        .await
        .map_err(|e| e.to_string())?;

    let total_orders = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM orders")
        .fetch_one(&db.pool)
        .await
        .map_err(|e| e.to_string())?;

    let total_categories = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM categories")
        .fetch_one(&db.pool)
        .await
        .map_err(|e| e.to_string())?;

    let stats = DashboardStats {
        total_users,
        total_products,
        total_orders,
        total_categories,
    };

    Ok(ApiResponse::success(stats))
}

#[tauri::command]
pub async fn get_system_info(
    state: State<'_, AppState>,
) -> Result<ApiResponse<SystemInfo>, String> {
    let db = state.db.lock().await;
    
    let system_name = sqlx::query_scalar::<_, String>(
        "SELECT setting_value FROM system_settings WHERE setting_key = 'system_name'"
    )
        .fetch_one(&db.pool)
        .await
        .unwrap_or_else(|_| "Rust Admin".to_string());

    let system_version = sqlx::query_scalar::<_, String>(
        "SELECT setting_value FROM system_settings WHERE setting_key = 'system_version'"
    )
        .fetch_one(&db.pool)
        .await
        .unwrap_or_else(|_| "1.0.0".to_string());

    let system_description = sqlx::query_scalar::<_, String>(
        "SELECT setting_value FROM system_settings WHERE setting_key = 'system_description'"
    )
        .fetch_one(&db.pool)
        .await
        .unwrap_or_else(|_| "基于 Tauri + Vue 3 的后台管理系统".to_string());

    let info = SystemInfo {
        system_name,
        system_version,
        system_description,
        rust_version: std::env::var("RUSTC_VERSION").unwrap_or_else(|_| "Unknown".to_string()),
        tauri_version: "1.5.0".to_string(),
    };

    Ok(ApiResponse::success(info))
} 