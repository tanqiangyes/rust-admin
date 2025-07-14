use tauri::State;
use crate::{AppState, models::settings::SystemSetting};
use crate::api::ApiResponse;
use serde::{Deserialize, Serialize};

// 定义响应类型
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemSettingsResponse {
    pub system: SystemSettings,
    pub ui: UISettings,
    pub security: SecuritySettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemSettings {
    pub system_name: String,
    pub system_description: String,
    pub system_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UISettings {
    pub theme_color: String,
    pub language: String,
    pub page_size: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecuritySettings {
    pub max_login_attempts: i32,
    pub lockout_duration: i32,
    pub reset_attempts_after: i32,
}

#[tauri::command]
pub async fn get_all_settings(
    state: State<'_, AppState>,
) -> Result<ApiResponse<SystemSettingsResponse>, String> {
    let db = state.db.lock().await;
    
    let settings = sqlx::query_as::<_, SystemSetting>(
        "SELECT * FROM system_settings ORDER BY setting_key"
    )
    .fetch_all(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    // 将设置转换为结构化格式
    let mut system = SystemSettings {
        system_name: "Rust Admin".to_string(),
        system_description: "基于 Tauri + Vue 3 的后台管理系统".to_string(),
        system_version: "1.0.0".to_string(),
    };

    let mut ui = UISettings {
        theme_color: "#1890ff".to_string(),
        language: "zh-CN".to_string(),
        page_size: 10,
    };

    let mut security = SecuritySettings {
        max_login_attempts: 5,
        lockout_duration: 300,
        reset_attempts_after: 3600,
    };

    // 从数据库设置中填充值
    for setting in settings {
        match setting.setting_key.as_str() {
            "system_name" => system.system_name = setting.setting_value,
            "system_description" => system.system_description = setting.setting_value,
            "system_version" => system.system_version = setting.setting_value,
            "theme_color" => ui.theme_color = setting.setting_value,
            "language" => ui.language = setting.setting_value,
            "page_size" => ui.page_size = setting.setting_value.parse().unwrap_or(10),
            "max_login_attempts" => security.max_login_attempts = setting.setting_value.parse().unwrap_or(5),
            "lockout_duration" => security.lockout_duration = setting.setting_value.parse().unwrap_or(300),
            "reset_attempts_after" => security.reset_attempts_after = setting.setting_value.parse().unwrap_or(3600),
            _ => {}
        }
    }

    Ok(ApiResponse::success(SystemSettingsResponse {
        system,
        ui,
        security,
    }))
}

#[tauri::command]
pub async fn save_system_settings(
    state: State<'_, AppState>,
    settings: SystemSettings,
) -> Result<ApiResponse<()>, String> {
    let db = state.db.lock().await;

    // 更新系统设置
    sqlx::query(
        "UPDATE system_settings SET setting_value = ? WHERE setting_key = ?"
    )
    .bind(&settings.system_name)
    .bind("system_name")
    .execute(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    sqlx::query(
        "UPDATE system_settings SET setting_value = ? WHERE setting_key = ?"
    )
    .bind(&settings.system_description)
    .bind("system_description")
    .execute(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    sqlx::query(
        "UPDATE system_settings SET setting_value = ? WHERE setting_key = ?"
    )
    .bind(&settings.system_version)
    .bind("system_version")
    .execute(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(()))
}

#[tauri::command]
pub async fn save_ui_settings(
    state: State<'_, AppState>,
    settings: UISettings,
) -> Result<ApiResponse<()>, String> {
    let db = state.db.lock().await;

    // 更新UI设置
    sqlx::query(
        "UPDATE system_settings SET setting_value = ? WHERE setting_key = ?"
    )
    .bind(&settings.theme_color)
    .bind("theme_color")
    .execute(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    sqlx::query(
        "UPDATE system_settings SET setting_value = ? WHERE setting_key = ?"
    )
    .bind(&settings.language)
    .bind("language")
    .execute(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    sqlx::query(
        "UPDATE system_settings SET setting_value = ? WHERE setting_key = ?"
    )
    .bind(&settings.page_size.to_string())
    .bind("page_size")
    .execute(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(()))
}

#[tauri::command]
pub async fn save_security_settings(
    state: State<'_, AppState>,
    settings: SecuritySettings,
) -> Result<ApiResponse<()>, String> {
    let db = state.db.lock().await;

    // 更新安全设置
    sqlx::query(
        "UPDATE system_settings SET setting_value = ? WHERE setting_key = ?"
    )
    .bind(&settings.max_login_attempts.to_string())
    .bind("max_login_attempts")
    .execute(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    sqlx::query(
        "UPDATE system_settings SET setting_value = ? WHERE setting_key = ?"
    )
    .bind(&settings.lockout_duration.to_string())
    .bind("lockout_duration")
    .execute(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    sqlx::query(
        "UPDATE system_settings SET setting_value = ? WHERE setting_key = ?"
    )
    .bind(&settings.reset_attempts_after.to_string())
    .bind("reset_attempts_after")
    .execute(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(()))
} 