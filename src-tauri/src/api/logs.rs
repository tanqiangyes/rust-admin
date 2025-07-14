use tauri::State;
use crate::{AppState, models::log::*, models::user::LoginAttempt};
use crate::api::{ApiResponse, PaginatedResponse};

#[tauri::command]
pub async fn get_logs(
    state: State<'_, AppState>,
    page: Option<i32>,
    per_page: Option<i32>,
) -> Result<ApiResponse<PaginatedResponse<Log>>, String> {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;
    
    let db = state.db.lock().await;

    let logs = sqlx::query_as::<_, Log>(
        "SELECT * FROM logs ORDER BY created_at DESC LIMIT ? OFFSET ?"
    )
    .bind(per_page)
    .bind(offset)
    .fetch_all(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    let total = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM logs")
        .fetch_one(&db.pool)
        .await
        .map_err(|e| e.to_string())?;

    let response = PaginatedResponse {
        items: logs,
        total,
        page,
        per_page,
        total_pages: (total + per_page as i64 - 1) / per_page as i64,
    };

    Ok(ApiResponse::success(response))
}

#[tauri::command]
pub async fn create_log(
    state: State<'_, AppState>,
    request: CreateLogRequest,
) -> Result<ApiResponse<Log>, String> {
    let db = state.db.lock().await;
    
    let result = sqlx::query(
        "INSERT INTO logs (user_id, action, description, ip_address, created_at) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(request.user_id)
    .bind(&request.action)
    .bind(&request.description)
    .bind(&request.ip_address)
    .bind(chrono::Utc::now())
    .execute(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    let log = sqlx::query_as::<_, Log>(
        "SELECT * FROM logs WHERE id = ?"
    )
    .bind(result.last_insert_rowid())
    .fetch_one(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(log))
} 

#[tauri::command]
pub async fn get_login_logs(
    state: State<'_, AppState>,
    page: Option<i32>,
    per_page: Option<i32>,
) -> Result<ApiResponse<PaginatedResponse<LoginAttempt>>, String> {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let db = state.db.lock().await;

    // 获取总数
    let total = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM login_logs")
        .fetch_one(&db.pool)
        .await
        .map_err(|e| e.to_string())?;

    // 获取日志数据
    let logs = sqlx::query_as::<_, LoginAttempt>(
        "SELECT * FROM login_logs ORDER BY created_at DESC LIMIT ? OFFSET ?"
    )
    .bind(per_page)
    .bind(offset)
    .fetch_all(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    let total_pages = (total as f64 / per_page as f64).ceil() as i32;

    Ok(ApiResponse::success(PaginatedResponse {
        items: logs,
        total,
        page,
        per_page,
        total_pages: total_pages.into(),
    }))
} 