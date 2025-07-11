use tauri::State;
use sqlx::Row;
use crate::{AppState, models::user::*};
use crate::api::{ApiResponse, PaginatedResponse};

#[tauri::command]
pub async fn get_users(
    state: State<'_, AppState>,
    page: Option<i32>,
    per_page: Option<i32>,
    search: Option<String>,
    status: Option<i32>,
) -> Result<ApiResponse<PaginatedResponse<UserWithRole>>, String> {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let mut base_query = r#"
        SELECT u.id, u.username, u.email, u.phone, u.address, u.avatar, u.role_id, r.name as role_name,
               u.status, u.created_at, u.updated_at
        FROM users u
        LEFT JOIN roles r ON u.role_id = r.id
        WHERE 1=1
    "#.to_string();

    let mut params = Vec::new();
    let mut param_count = 0;

    if let Some(search_term) = &search {
        if !search_term.is_empty() {
            param_count += 1;
            base_query.push_str(&format!(" AND (u.username LIKE ?{} OR u.email LIKE ?{})", param_count, param_count + 1));
            params.push(format!("%{}%", search_term));
            params.push(format!("%{}%", search_term));
            param_count += 1;
        }
    }

    if let Some(status_filter) = status {
        param_count += 1;
        base_query.push_str(&format!(" AND u.status = ?{}", param_count));
        params.push(status_filter.to_string());
    }

    let count_query = format!("SELECT COUNT(*) FROM ({}) as counted", base_query);
    let data_query = format!("{} ORDER BY u.created_at DESC LIMIT {} OFFSET {}", base_query, per_page, offset);

    let mut count_query_builder = sqlx::query(&count_query);
    let mut data_query_builder = sqlx::query(&data_query);

    for param in &params {
        count_query_builder = count_query_builder.bind(param);
        data_query_builder = data_query_builder.bind(param);
    }

    let total = count_query_builder
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| e.to_string())?
        .get::<i64, _>(0);

    let rows = data_query_builder
        .fetch_all(&state.db.pool)
        .await
        .map_err(|e| e.to_string())?;

    let mut users = Vec::new();
    for row in rows {
        users.push(UserWithRole {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            phone: row.get("phone"),
            address: row.get("address"),
            avatar: row.get("avatar"),
            role_id: row.get("role_id"),
            role_name: row.get("role_name"),
            status: row.get("status"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        });
    }

    Ok(ApiResponse::success(PaginatedResponse {
        items: users,
        total,
        page,
        per_page,
    }))
}

#[tauri::command]
pub async fn create_user(
    state: State<'_, AppState>,
    request: CreateUserRequest,
) -> Result<ApiResponse<User>, String> {
    let password_hash = bcrypt::hash(&request.password, bcrypt::DEFAULT_COST)
        .map_err(|e| e.to_string())?;

    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (username, email, password_hash, phone, address, avatar, role_id, status)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING *
        "#
    )
    .bind(&request.username)
    .bind(&request.email)
    .bind(&password_hash)
    .bind(&request.phone)
    .bind(&request.address)
    .bind(&request.avatar)
    .bind(request.role_id)
    .bind(request.status)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(user))
}

#[tauri::command]
pub async fn update_user(
    state: State<'_, AppState>,
    user_id: i64,
    request: UpdateUserRequest,
) -> Result<ApiResponse<User>, String> {
    let user = sqlx::query_as::<_, User>(
        r#"
        UPDATE users 
        SET username = COALESCE(?, username),
            email = COALESCE(?, email),
            phone = COALESCE(?, phone),
            address = COALESCE(?, address),
            avatar = COALESCE(?, avatar),
            role_id = COALESCE(?, role_id),
            status = COALESCE(?, status),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ?
        RETURNING *
        "#
    )
    .bind(&request.username)
    .bind(&request.email)
    .bind(&request.phone)
    .bind(&request.address)
    .bind(&request.avatar)
    .bind(request.role_id)
    .bind(request.status)
    .bind(user_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(user))
}

#[tauri::command]
pub async fn delete_user(
    state: State<'_, AppState>,
    user_id: i64,
) -> Result<ApiResponse<()>, String> {
    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(user_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(()))
} 