use tauri::State;
use crate::{AppState, models::user::*};
use crate::api::{ApiResponse, PaginatedResponse};
use bcrypt::{hash, DEFAULT_COST};

#[tauri::command]
pub async fn get_users(
    state: State<'_, AppState>,
    page: Option<i32>,
    per_page: Option<i32>,
    search: Option<String>,
) -> Result<ApiResponse<PaginatedResponse<User>>, String> {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let mut query = "SELECT * FROM users WHERE 1=1".to_string();
    let mut count_query = "SELECT COUNT(*) FROM users WHERE 1=1".to_string();

    if let Some(search) = &search {
        query.push_str(" AND (username LIKE ? OR email LIKE ?)");
        count_query.push_str(" AND (username LIKE ? OR email LIKE ?)");
    }

    query.push_str(" ORDER BY created_at DESC LIMIT ? OFFSET ?");

    let mut users_query = sqlx::query_as::<_, User>(&query);
    let mut total_query = sqlx::query_scalar::<_, i64>(&count_query);

    if let Some(search) = &search {
        let search_pattern = format!("%{}%", search);
        users_query = users_query.bind(&search_pattern).bind(&search_pattern);
        total_query = total_query.bind(&search_pattern).bind(&search_pattern);
    }

    let users = users_query
        .bind(per_page)
        .bind(offset)
        .fetch_all(&state.db.pool)
        .await
        .map_err(|e| e.to_string())?;

    let total = total_query
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| e.to_string())?;

    let total_pages = (total as f64 / per_page as f64).ceil() as i32;

    let response = PaginatedResponse {
        items: users,
        total,
        page,
        per_page,
        total_pages,
    };

    Ok(ApiResponse::success(response))
}

#[tauri::command]
pub async fn create_user(
    state: State<'_, AppState>,
    request: CreateUserRequest,
) -> Result<ApiResponse<User>, String> {
    let password_hash = hash(&request.password, DEFAULT_COST)
        .map_err(|e| e.to_string())?;

    let user_id = sqlx::query_scalar::<_, i64>(
        "INSERT INTO users (username, email, password_hash, phone, address, role_id) 
         VALUES (?, ?, ?, ?, ?, ?) RETURNING id"
    )
    .bind(&request.username)
    .bind(&request.email)
    .bind(&password_hash)
    .bind(&request.phone)
    .bind(&request.address)
    .bind(request.role_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(user))
}

#[tauri::command]
pub async fn update_user(
    state: State<'_, AppState>,
    id: i64,
    request: UpdateUserRequest,
) -> Result<ApiResponse<User>, String> {
    let mut query = "UPDATE users SET updated_at = CURRENT_TIMESTAMP".to_string();
    let mut params: Vec<String> = Vec::new();

    if let Some(username) = &request.username {
        query.push_str(", username = ?");
        params.push(username.clone());
    }
    if let Some(email) = &request.email {
        query.push_str(", email = ?");
        params.push(email.clone());
    }
    if let Some(phone) = &request.phone {
        query.push_str(", phone = ?");
        params.push(phone.clone());
    }
    if let Some(address) = &request.address {
        query.push_str(", address = ?");
        params.push(address.clone());
    }
    if let Some(role_id) = request.role_id {
        query.push_str(", role_id = ?");
        params.push(role_id.to_string());
    }
    if let Some(status) = request.status {
        query.push_str(", status = ?");
        params.push(status.to_string());
    }

    query.push_str(" WHERE id = ?");
    params.push(id.to_string());

    let mut query_builder = sqlx::query(&query);
    for param in params {
        query_builder = query_builder.bind(param);
    }

    query_builder
        .execute(&state.db.pool)
        .await
        .map_err(|e| e.to_string())?;

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(user))
}

#[tauri::command]
pub async fn delete_user(
    state: State<'_, AppState>,
    id: i64,
) -> Result<ApiResponse<()>, String> {
    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(()))
} 