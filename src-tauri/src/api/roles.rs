use tauri::State;
use crate::{AppState, models::role::Role};
use crate::api::ApiResponse;

#[tauri::command]
pub async fn get_roles(
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<Role>>, String> {
    let db = state.db.lock().await;
    
    let roles = sqlx::query_as::<_, Role>(
        "SELECT * FROM roles ORDER BY id"
    )
    .fetch_all(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(roles))
} 