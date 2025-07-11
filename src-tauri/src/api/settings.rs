use tauri::State;
use crate::{AppState, models::settings::*};
use crate::api::ApiResponse;
use crate::api::auth::{check_permission, get_user_id_from_token};
use std::collections::HashMap;

#[tauri::command]
pub async fn get_all_settings(
    state: State<'_, AppState>,
    token: String,
) -> Result<ApiResponse<AllSettings>, String> {
    // 权限检查 - 只有超级管理员可以查看系统设置
    if let Some(user_id) = get_user_id_from_token(&token) {
        if !check_permission(&state, user_id, "settings:read").await? {
            return Err("没有权限访问系统设置".to_string());
        }
    } else {
        return Err("无效的token".to_string());
    }

    // 原有逻辑...
    let settings = sqlx::query_as::<_, SystemSetting>(
        "SELECT * FROM system_settings"
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;

    let mut settings_map: HashMap<String, String> = HashMap::new();
    for setting in settings {
        settings_map.insert(setting.setting_key, setting.setting_value);
    }

    let system_settings = SystemSettings {
        system_name: settings_map.get("system_name").unwrap_or(&"Rust Admin".to_string()).clone(),
        system_description: settings_map.get("system_description").unwrap_or(&"基于 Tauri + Vue 3 的后台管理系统".to_string()).clone(),
        system_version: settings_map.get("system_version").unwrap_or(&"1.0.0".to_string()).clone(),
    };

    let ui_settings = UISettings {
        theme_color: settings_map.get("theme_color").unwrap_or(&"#1890ff".to_string()).clone(),
        language: settings_map.get("language").unwrap_or(&"zh-CN".to_string()).clone(),
        page_size: settings_map.get("page_size").unwrap_or(&"10".to_string()).parse().unwrap_or(10),
    };

    let security_settings = SecuritySettings {
        enable_registration: settings_map.get("enable_registration").unwrap_or(&"false".to_string()) == "true",
        session_timeout: settings_map.get("session_timeout").unwrap_or(&"3600".to_string()).parse().unwrap_or(3600),
        max_login_attempts: settings_map.get("max_login_attempts").unwrap_or(&"5".to_string()).parse().unwrap_or(5),
        maintenance_mode: settings_map.get("maintenance_mode").unwrap_or(&"false".to_string()) == "true",
    };

    Ok(ApiResponse::success(AllSettings {
        system: system_settings,
        ui: ui_settings,
        security: security_settings,
    }))
}

#[tauri::command]
pub async fn save_system_settings(
    state: State<'_, AppState>,
    token: String,
    settings: SystemSettings,
) -> Result<ApiResponse<()>, String> {
    // 权限检查 - 只有超级管理员可以修改系统设置
    if let Some(user_id) = get_user_id_from_token(&token) {
        if !check_permission(&state, user_id, "settings:write").await? {
            return Err("没有权限修改系统设置".to_string());
        }
    } else {
        return Err("无效的token".to_string());
    }

    // 原有的保存逻辑...
    sqlx::query(
        "UPDATE system_settings SET setting_value = ?, updated_at = CURRENT_TIMESTAMP WHERE setting_key = ?"
    )
    .bind(&settings.system_name)
    .bind("system_name")
    .execute(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;

    // ... 其他设置的更新 ...

    Ok(ApiResponse::success(()))
}

#[tauri::command]
pub async fn save_ui_settings(
    state: State<'_, AppState>,
    settings: UISettings,
) -> Result<ApiResponse<()>, String> {
    // 更新主题颜色
    sqlx::query(
        "UPDATE system_settings SET setting_value = ?, updated_at = CURRENT_TIMESTAMP WHERE setting_key = ?"
    )
    .bind(&settings.theme_color)
    .bind("theme_color")
    .execute(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;

    // 更新语言
    sqlx::query(
        "UPDATE system_settings SET setting_value = ?, updated_at = CURRENT_TIMESTAMP WHERE setting_key = ?"
    )
    .bind(&settings.language)
    .bind("language")
    .execute(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;

    // 更新页面大小
    sqlx::query(
        "UPDATE system_settings SET setting_value = ?, updated_at = CURRENT_TIMESTAMP WHERE setting_key = ?"
    )
    .bind(settings.page_size.to_string())
    .bind("page_size")
    .execute(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(()))
}

#[tauri::command]
pub async fn save_security_settings(
    state: State<'_, AppState>,
    settings: SecuritySettings,
) -> Result<ApiResponse<()>, String> {
    // 更新注册开关
    sqlx::query(
        "UPDATE system_settings SET setting_value = ?, updated_at = CURRENT_TIMESTAMP WHERE setting_key = ?"
    )
    .bind(settings.enable_registration.to_string())
    .bind("enable_registration")
    .execute(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;

    // 更新会话超时
    sqlx::query(
        "UPDATE system_settings SET setting_value = ?, updated_at = CURRENT_TIMESTAMP WHERE setting_key = ?"
    )
    .bind(settings.session_timeout.to_string())
    .bind("session_timeout")
    .execute(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;

    // 更新最大登录尝试次数
    sqlx::query(
        "UPDATE system_settings SET setting_value = ?, updated_at = CURRENT_TIMESTAMP WHERE setting_key = ?"
    )
    .bind(settings.max_login_attempts.to_string())
    .bind("max_login_attempts")
    .execute(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;

    // 更新维护模式
    sqlx::query(
        "UPDATE system_settings SET setting_value = ?, updated_at = CURRENT_TIMESTAMP WHERE setting_key = ?"
    )
    .bind(settings.maintenance_mode.to_string())
    .bind("maintenance_mode")
    .execute(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(()))
}

#[tauri::command]
pub async fn update_setting(
    state: State<'_, AppState>,
    request: UpdateSettingRequest,
) -> Result<ApiResponse<()>, String> {
    sqlx::query(
        "UPDATE system_settings SET setting_value = ?, updated_at = CURRENT_TIMESTAMP WHERE setting_key = ?"
    )
    .bind(&request.setting_value)
    .bind(&request.setting_key)
    .execute(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(()))
}

#[tauri::command]
pub async fn get_system_settings(
    state: State<'_, AppState>,
) -> Result<ApiResponse<SystemSettings>, String> {
    let settings = sqlx::query_as::<_, SystemSetting>(
        "SELECT * FROM system_settings WHERE setting_key IN ('system_name', 'system_description', 'system_version')"
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;

    let mut settings_map: HashMap<String, String> = HashMap::new();
    for setting in settings {
        settings_map.insert(setting.setting_key, setting.setting_value);
    }

    Ok(ApiResponse::success(SystemSettings {
        system_name: settings_map.get("system_name").unwrap_or(&"Rust Admin".to_string()).clone(),
        system_description: settings_map.get("system_description").unwrap_or(&"基于 Tauri + Vue 3 的后台管理系统".to_string()).clone(),
        system_version: settings_map.get("system_version").unwrap_or(&"1.0.0".to_string()).clone(),
    }))
} 