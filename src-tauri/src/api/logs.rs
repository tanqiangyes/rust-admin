use tauri::State;
use crate::{AppState, models::log::*};
use crate::api::{ApiResponse, PaginatedResponse};

#[tauri::command]
pub async fn get_logs(
    state: State<'_, AppState>,
    page: Option<i32>,
    per_page: Option<i32>,
) -> Result<ApiResponse<PaginatedResponse<LogWithUser>>, String> {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let logs = sqlx::query_as::<_, LogWithUser>(
        r#"
        SELECT l.id, l.user_id, u.username, l.action, l.resource, l.details, l.created_at
        FROM logs l
        LEFT JOIN users u ON l.user_id = u.id
        ORDER BY l.created_at DESC
        LIMIT ? OFFSET ?
        "#
    )
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;

    let total = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM logs")
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| e.to_string())?;

    let total_pages = (total as f64 / per_page as f64).ceil() as i32;

    let response = PaginatedResponse {
        items: logs,
        total,
        page,
        per_page,
        total_pages,
    };

    Ok(ApiResponse::success(response))
}

#[tauri::command]
pub async fn create_log(
    state: State<'_, AppState>,
    user_id: Option<i64>,
    action: String,
    resource: String,
    details: Option<String>,
) -> Result<ApiResponse<()>, String> {
    sqlx::query(
        "INSERT INTO logs (user_id, action, resource, details) VALUES (?, ?, ?, ?)"
    )
    .bind(user_id)
    .bind(action)
    .bind(resource)
    .bind(details)
    .execute(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(()))
} 