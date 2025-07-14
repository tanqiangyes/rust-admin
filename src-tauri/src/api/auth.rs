use tauri::State;
use crate::{AppState, models::user::*, models::role::Role, utils::permissions::Permission};
use crate::api::ApiResponse;
use chrono::{Duration, Utc};
use crate::database::Database;

#[tauri::command]
pub async fn login(
    state: State<'_, AppState>,
    request: LoginRequest,
) -> Result<ApiResponse<LoginResponse>, String> {
    println!("Login attempt for username: {}", request.username);
    
    // 获取数据库连接
    let db = state.db.lock().await;
    
    // 获取系统设置
    let max_attempts = get_system_setting(&db, "max_login_attempts", "5").await.parse::<i32>().unwrap_or(5);
    let lockout_duration = get_system_setting(&db, "lockout_duration", "300").await.parse::<i64>().unwrap_or(300);
    let reset_attempts_after = get_system_setting(&db, "reset_attempts_after", "3600").await.parse::<i64>().unwrap_or(3600);
    
    // 查找用户
    let user_result = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE username = ? OR email = ?"
    )
    .bind(&request.username)
    .bind(&request.username)
    .fetch_optional(&db.pool)
    .await
    .map_err(|e| {
        println!("Database error: {}", e);
        e.to_string()
    })?;

    let mut user = match user_result {
        Some(user) => user,
        None => {
            println!("User not found: {}", request.username);
            return Ok(ApiResponse::error("用户名或密码错误".to_string()));
        }
    };

    // 检查用户是否被锁定
    if let Some(locked_until) = user.locked_until {
        if Utc::now() < locked_until {
            let remaining_seconds = (locked_until - Utc::now()).num_seconds();
            return Ok(ApiResponse::error(format!("账户已被锁定，请在 {} 分钟后重试", remaining_seconds / 60)));
        } else {
            // 解锁用户
            user.locked_until = None;
            user.failed_login_attempts = 0;
            update_user_login_status(&db, &user).await?;
        }
    }

    // 检查是否需要重置登录尝试次数
    if let Some(last_failed) = user.last_failed_login {
        if (Utc::now() - last_failed).num_seconds() > reset_attempts_after {
            user.failed_login_attempts = 0;
            update_user_login_status(&db, &user).await?;
        }
    }

    // 验证密码
    let password_valid = bcrypt::verify(&request.password, &user.password_hash)
        .map_err(|e| e.to_string())?;

    if !password_valid {
        // 密码错误，增加失败次数
        user.failed_login_attempts += 1;
        user.last_failed_login = Some(Utc::now());

        // 检查是否需要锁定账户
        if user.failed_login_attempts >= max_attempts {
            user.locked_until = Some(Utc::now() + Duration::seconds(lockout_duration));
            update_user_login_status(&db, &user).await?;
            return Ok(ApiResponse::error(format!("登录失败次数过多，账户已被锁定 {} 分钟", lockout_duration / 60)));
        } else {
            update_user_login_status(&db, &user).await?;
            let remaining_attempts = max_attempts - user.failed_login_attempts;
            return Ok(ApiResponse::error(format!("用户名或密码错误，还有 {} 次尝试机会", remaining_attempts)));
        }
    }

    // 登录成功，重置失败次数
    user.failed_login_attempts = 0;
    user.last_failed_login = None;
    user.locked_until = None;
    user.last_login = Some(Utc::now());
    update_user_login_status(&db, &user).await?;

    // 获取用户角色
    let role = sqlx::query_as::<_, Role>(
        "SELECT * FROM roles WHERE id = ?"
    )
    .bind(user.role_id)
    .fetch_one(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    // 创建用户响应
    let user_response = UserResponse {
        id: user.id,
        username: user.username,
        email: user.email,
        phone: user.phone,
        address: user.address,
        avatar: user.avatar,
        role_id: user.role_id,
        role_name: role.name,
        permissions: role.permissions,
        status: user.status,
        created_at: user.created_at,
        updated_at: user.updated_at,
    };

    let login_response = LoginResponse {
        token: format!("mock_token_{}", user.id),
        user: user_response,
    };

    Ok(ApiResponse::success(login_response))
}

#[tauri::command]
pub async fn logout() -> Result<ApiResponse<()>, String> {
    Ok(ApiResponse::success(()))
}

#[tauri::command]
pub async fn get_current_user(
    state: State<'_, AppState>,
    user_id: i64,
) -> Result<ApiResponse<UserResponse>, String> {
    let db = state.db.lock().await;
    
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = ?"
    )
    .bind(user_id)
    .fetch_one(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    let role = sqlx::query_as::<_, Role>(
        "SELECT * FROM roles WHERE id = ?"
    )
    .bind(user.role_id)
    .fetch_one(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    let user_response = UserResponse {
        id: user.id,
        username: user.username,
        email: user.email,
        phone: user.phone,
        address: user.address,
        avatar: user.avatar,
        role_id: user.role_id,
        role_name: role.name,
        permissions: role.permissions,
        status: user.status,
        created_at: user.created_at,
        updated_at: user.updated_at,
    };

    Ok(ApiResponse::success(user_response))
}

// 辅助函数：更新用户登录状态
async fn update_user_login_status(
    db: &Database, 
    user: &User
) -> Result<(), String> {
    sqlx::query(
        r#"
        UPDATE users 
        SET failed_login_attempts = ?, last_failed_login = ?, locked_until = ?, last_login = ?
        WHERE id = ?
        "#
    )
    .bind(user.failed_login_attempts)
    .bind(user.last_failed_login)
    .bind(user.locked_until)
    .bind(user.last_login)
    .bind(user.id)
    .execute(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

// 辅助函数：获取系统设置
async fn get_system_setting(db: &Database, key: &str, default: &str) -> String {
    let result = sqlx::query_scalar::<_, String>(
        "SELECT setting_value FROM system_settings WHERE setting_key = ?"
    )
    .bind(key)
    .fetch_optional(&db.pool)
    .await;

    match result {
        Ok(Some(value)) => value,
        _ => default.to_string(),
    }
}

#[tauri::command]
pub async fn check_permission(
    permissions: String,
    required_permission: String,
) -> Result<bool, String> {
    Ok(Permission::has_permission(&permissions, &required_permission))
}

#[tauri::command]
pub async fn get_user_permissions(
    state: State<'_, AppState>,
    user_id: i64,
) -> Result<ApiResponse<crate::api::PaginatedResponse<String>>, String> {
    let db = state.db.lock().await;
    
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = ?"
    )
    .bind(user_id)
    .fetch_one(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    let role = sqlx::query_as::<_, Role>(
        "SELECT * FROM roles WHERE id = ?"
    )
    .bind(user.role_id)
    .fetch_one(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    // 解析权限
    let user_permissions = Permission::get_permissions(&role.permissions);
    let permissions: Vec<String> = user_permissions.into_iter().collect();

    Ok(ApiResponse::success(crate::api::PaginatedResponse {
        items: permissions.clone(),
        total: permissions.len() as i64,
        page: 1,
        per_page: permissions.len() as i32,
        total_pages: 1,
    }))
} 