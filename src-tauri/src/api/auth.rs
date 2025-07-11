use tauri::State;
use crate::AppState;
use crate::models::user::{User, LoginRequest, LoginResponse};
use crate::api::ApiResponse;
use bcrypt::verify;

#[tauri::command]
pub async fn login(
    state: State<'_, AppState>,
    request: LoginRequest,
) -> Result<ApiResponse<LoginResponse>, String> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE username = ? AND status = 1"
    )
    .bind(&request.username)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;

    if let Some(user) = user {
        if verify(&request.password, &user.password_hash).unwrap_or(false) {
            let token = format!("token_{}", user.id);
            
            let response = LoginResponse {
                token,
                user,
            };
            
            Ok(ApiResponse::success(response))
        } else {
            Ok(ApiResponse::error("Invalid credentials".to_string()))
        }
    } else {
        Ok(ApiResponse::error("User not found".to_string()))
    }
}

#[tauri::command]
pub async fn logout() -> Result<ApiResponse<()>, String> {
    Ok(ApiResponse::success(()))
}

#[tauri::command]
pub async fn get_current_user(
    state: State<'_, AppState>,
    user_id: i64,
) -> Result<ApiResponse<User>, String> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = ?"
    )
    .bind(user_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;

    if let Some(user) = user {
        Ok(ApiResponse::success(user))
    } else {
        Ok(ApiResponse::error("User not found".to_string()))
    }
} 