// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod database;
mod models;
mod utils;

use database::Database;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AppState {
    db: Arc<Mutex<Database>>,
}

#[tokio::main]
async fn main() {
    // 初始化数据库
    let db = Database::new().await.expect("Failed to initialize database");
    let app_state = AppState {
        db: Arc::new(Mutex::new(db)),
    };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            // 认证相关
            api::auth::login,
            api::auth::logout,
            api::auth::get_current_user,
            
            // 用户管理
            api::users::get_users,
            api::users::create_user,
            api::users::update_user,
            api::users::delete_user,
            
            // 商品管理
            api::products::get_products,
            api::products::create_product,
            api::products::update_product,
            api::products::delete_product,
            
            // 订单管理
            api::orders::get_orders,
            api::orders::create_order,
            api::orders::update_order_status,
            
            // 分类管理
            api::categories::get_categories,
            api::categories::create_category,
            api::categories::update_category,
            api::categories::delete_category,
            
            // 角色管理
            api::roles::get_roles,
            
            // 统计信息
            api::stats::get_dashboard_stats,
            api::stats::get_system_info,
            
            // 日志管理
            api::logs::get_logs,
            api::logs::create_log,
            api::logs::get_login_logs,
            api::settings::get_all_settings,
            api::settings::save_system_settings,
            api::settings::save_ui_settings,
            api::settings::save_security_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
