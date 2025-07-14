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
) -> Result<ApiResponse<PaginatedResponse<UserWithRole>>, String> {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;
    
    let db = state.db.lock().await;

    if let Some(search_term) = &search {
        let search_pattern = format!("%{}%", search_term);
        
        let rows = sqlx::query(
            r#"
            SELECT u.id, u.username, u.email, u.phone, u.address, u.avatar, 
                   u.role_id, r.name as role_name, r.permissions, u.status, u.created_at, u.updated_at
            FROM users u
            LEFT JOIN roles r ON u.role_id = r.id
            WHERE u.username LIKE ? OR u.email LIKE ?
            ORDER BY u.created_at DESC LIMIT ? OFFSET ?
            "#
        )
        .bind(&search_pattern)
        .bind(&search_pattern)
        .bind(per_page)
        .bind(offset)
        .fetch_all(&db.pool)
        .await
        .map_err(|e| e.to_string())?;

        let total = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM users u LEFT JOIN roles r ON u.role_id = r.id WHERE u.username LIKE ? OR u.email LIKE ?"
        )
        .bind(&search_pattern)
        .bind(&search_pattern)
        .fetch_one(&db.pool)
        .await
        .map_err(|e| e.to_string())?;

        let users: Vec<UserWithRole> = rows.into_iter().map(|row| {
            UserWithRole {
                id: row.get("id"),
                username: row.get("username"),
                email: row.get("email"),
                phone: row.get("phone"),
                address: row.get("address"),
                avatar: row.get("avatar"),
                role_id: row.get("role_id"),
                role_name: row.get::<Option<String>, _>("role_name").unwrap_or_else(|| "未知角色".to_string()),
                permissions: row.get::<Option<String>, _>("permissions").unwrap_or_else(|| "[]".to_string()),
                status: row.get("status"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }
        }).collect();

        let total_pages = (total as f64 / per_page as f64).ceil() as i64;

        let response = PaginatedResponse {
            items: users,
            total,
            page,
            per_page,
            total_pages,
        };

        Ok(ApiResponse::success(response))
    } else {
        let rows = sqlx::query(
            r#"
            SELECT u.id, u.username, u.email, u.phone, u.address, u.avatar, 
                   u.role_id, r.name as role_name, r.permissions, u.status, u.created_at, u.updated_at
            FROM users u
            LEFT JOIN roles r ON u.role_id = r.id
            ORDER BY u.created_at DESC LIMIT ? OFFSET ?
            "#
        )
        .bind(per_page)
        .bind(offset)
        .fetch_all(&db.pool)
        .await
        .map_err(|e| e.to_string())?;

        let total = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
            .fetch_one(&db.pool)
            .await
            .map_err(|e| e.to_string())?;

        let users: Vec<UserWithRole> = rows.into_iter().map(|row| {
            UserWithRole {
                id: row.get("id"),
                username: row.get("username"),
                email: row.get("email"),
                phone: row.get("phone"),
                address: row.get("address"),
                avatar: row.get("avatar"),
                role_id: row.get("role_id"),
                role_name: row.get::<Option<String>, _>("role_name").unwrap_or_else(|| "未知角色".to_string()),
                permissions: row.get::<Option<String>, _>("permissions").unwrap_or_else(|| "[]".to_string()),
                status: row.get("status"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }
        }).collect();

        let total_pages = (total as f64 / per_page as f64).ceil() as i64;

        let response = PaginatedResponse {
            items: users,
            total,
            page,
            per_page,
            total_pages,
        };

        Ok(ApiResponse::success(response))
    }
}

#[tauri::command]
pub async fn create_user(
    state: State<'_, AppState>,
    request: CreateUserRequest,
) -> Result<ApiResponse<User>, String> {
    let db = state.db.lock().await;
    
    // 检查用户名和邮箱是否已存在
    let existing = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM users WHERE username = ? OR email = ?"
    )
    .bind(&request.username)
    .bind(&request.email)
    .fetch_one(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    if existing > 0 {
        return Ok(ApiResponse::error("用户名或邮箱已存在".to_string()));
    }

    // 加密密码
    let password_hash = bcrypt::hash(&request.password, bcrypt::DEFAULT_COST)
        .map_err(|e| e.to_string())?;

    let result = sqlx::query(
        r#"INSERT INTO users (username, email, password_hash, phone, address, avatar, role_id, status, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
    )
    .bind(&request.username)
    .bind(&request.email)
    .bind(&password_hash)
    .bind(&request.phone)
    .bind(&request.address)
    .bind(&request.avatar)
    .bind(request.role_id)
    .bind(1) // 默认状态为启用
    .bind(chrono::Utc::now())
    .bind(chrono::Utc::now())
    .execute(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = ?"
    )
    .bind(result.last_insert_rowid())
    .fetch_one(&db.pool)
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
    let db = state.db.lock().await;
    
    // 检查是否为admin用户，如果是则限制某些字段的修改
    let current_user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = ?"
    )
    .bind(user_id)
    .fetch_one(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    if current_user.username == "admin" {
        // 对于admin用户，只更新基本信息，不更新敏感信息
        sqlx::query(
            "UPDATE users SET phone = ?, address = ?, avatar = ?, updated_at = ? WHERE id = ?"
        )
        .bind(&request.phone)
        .bind(&request.address)
        .bind(&request.avatar)
        .bind(chrono::Utc::now())
        .bind(user_id)
        .execute(&db.pool)
        .await
        .map_err(|e| e.to_string())?;
    } else {
        // 对于普通用户，可以更新所有字段
        sqlx::query(
            "UPDATE users SET username = ?, email = ?, phone = ?, address = ?, avatar = ?, role_id = ?, updated_at = ? WHERE id = ?"
        )
        .bind(&request.username)
        .bind(&request.email)
        .bind(&request.phone)
        .bind(&request.address)
        .bind(&request.avatar)
        .bind(request.role_id)
        .bind(chrono::Utc::now())
        .bind(user_id)
        .execute(&db.pool)
        .await
        .map_err(|e| e.to_string())?;
    }

    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = ?"
    )
    .bind(user_id)
    .fetch_one(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(user))
}

#[tauri::command]
pub async fn delete_user(
    state: State<'_, AppState>,
    user_id: i64,
) -> Result<ApiResponse<()>, String> {
    // 检查是否为admin用户（假设admin用户的ID为1，或者用户名为admin）
    let db = state.db.lock().await;
    
    let user = sqlx::query("SELECT id, username FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_one(&db.pool)
        .await
        .map_err(|e| e.to_string())?;
    
    let username: String = user.get("username");
    
    // 禁止删除admin用户
    if username == "admin" || user_id == 1 {
        return Ok(ApiResponse::error("管理员账户不可删除".to_string()));
    }
    
    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(user_id)
        .execute(&db.pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(()))
} 