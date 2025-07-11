use tauri::State;
use crate::{AppState, models::user::*, utils::permissions::Permission};
use crate::api::ApiResponse;

// 权限检查函数
pub async fn check_permission(
    state: &State<'_, AppState>,
    user_id: i64,
    required_permission: &str,
) -> Result<bool, String> {
    // 获取用户角色权限
    let permissions = sqlx::query_scalar::<_, String>(
        "SELECT r.permissions FROM users u JOIN roles r ON u.role_id = r.id WHERE u.id = ?"
    )
    .bind(user_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Permission::has_permission(&permissions, required_permission))
}

// 获取当前用户ID（从token中解析，这里简化处理）
pub fn get_user_id_from_token(token: &str) -> Option<i64> {
    // 这里是简化的token解析，实际项目中应该使用JWT
    if token.starts_with("token_") {
        let parts: Vec<&str> = token.split('_').collect();
        if parts.len() >= 2 {
            return parts[1].parse().ok();
        }
    }
    None
}

#[tauri::command]
pub async fn check_user_permission(
    state: State<'_, AppState>,
    token: String,
    permission: String,
) -> Result<ApiResponse<bool>, String> {
    if let Some(user_id) = get_user_id_from_token(&token) {
        let has_permission = check_permission(&state, user_id, &permission).await?;
        Ok(ApiResponse::success(has_permission))
    } else {
        Ok(ApiResponse::success(false))
    }
}

#[tauri::command]
pub async fn get_user_permissions(
    state: State<'_, AppState>,
    token: String,
) -> Result<ApiResponse<Vec<String>>, String> {
    if let Some(user_id) = get_user_id_from_token(&token) {
        let permissions = sqlx::query_scalar::<_, String>(
            "SELECT r.permissions FROM users u JOIN roles r ON u.role_id = r.id WHERE u.id = ?"
        )
        .bind(user_id)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| e.to_string())?;

        let user_permissions = Permission::get_permissions(&permissions);
        Ok(ApiResponse::success(user_permissions))
    } else {
        Ok(ApiResponse::success(vec![]))
    }
}

#[tauri::command]
pub async fn login(
    state: State<'_, AppState>,
    request: LoginRequest,
) -> Result<ApiResponse<LoginResponse>, String> {
    println!("Login attempt for username: {}", request.username);
    
    // 查找用户
    let user_result = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE username = ? OR email = ?"
    )
    .bind(&request.username)
    .bind(&request.username)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        println!("Database error: {}", e);
        format!("数据库错误: {}", e)
    })?;

    let user = match user_result {
        Some(user) => user,
        None => {
            println!("User not found: {}", request.username);
            return Ok(ApiResponse {
                success: false,
                data: LoginResponse::default(),
                message: "用户名或密码错误".to_string(),
            });
        }
    };

    // 验证密码
    let password_valid = bcrypt::verify(&request.password, &user.password_hash)
        .map_err(|e| {
            println!("Password verification error: {}", e);
            format!("密码验证错误: {}", e)
        })?;

    if !password_valid {
        println!("Invalid password for user: {}", request.username);
        return Ok(ApiResponse {
            success: false,
            data: LoginResponse::default(),
            message: "用户名或密码错误".to_string(),
        });
    }

    // 检查用户状态
    if user.status != 1 {
        println!("User is disabled: {}", request.username);
        return Ok(ApiResponse {
            success: false,
            data: LoginResponse::default(),
            message: "用户账户已被禁用".to_string(),
        });
    }

    // 获取用户角色信息
    let role_name = sqlx::query_scalar::<_, String>(
        "SELECT name FROM roles WHERE id = ?"
    )
    .bind(user.role_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or_else(|_| "普通用户".to_string());

    // 生成简单的token
    let token = format!("token_{}_{}", user.id, chrono::Utc::now().timestamp());

    println!("Login successful for user: {}", request.username);

    let response = LoginResponse {
        token,
        user: UserWithRole {
            id: user.id,
            username: user.username,
            email: user.email,
            phone: user.phone,
            address: user.address,
            avatar: user.avatar,
            role_id: user.role_id,
            role_name,
            status: user.status,
            created_at: user.created_at,
            updated_at: user.updated_at,
        },
    };

    Ok(ApiResponse::success(response))
}

#[tauri::command]
pub async fn logout(
    _state: State<'_, AppState>,
) -> Result<ApiResponse<()>, String> {
    Ok(ApiResponse::success(()))
}

#[tauri::command]
pub async fn get_current_user(
    state: State<'_, AppState>,
    user_id: i64,
) -> Result<ApiResponse<UserWithRole>, String> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = ?"
    )
    .bind(user_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;

    let role_name = sqlx::query_scalar::<_, String>(
        "SELECT name FROM roles WHERE id = ?"
    )
    .bind(user.role_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or_else(|_| "普通用户".to_string());

    let user_with_role = UserWithRole {
        id: user.id,
        username: user.username,
        email: user.email,
        phone: user.phone,
        address: user.address,
        avatar: user.avatar,
        role_id: user.role_id,
        role_name,
        status: user.status,
        created_at: user.created_at,
        updated_at: user.updated_at,
    };

    Ok(ApiResponse::success(user_with_role))
} 