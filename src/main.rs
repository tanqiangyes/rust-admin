use tauri::Manager;
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;

mod database;
mod api;
mod models;
mod utils;

use database::Database;
use api::*;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
}

#[tokio::main]
async fn main() {
    // 初始化数据库
    let database = Database::new().await.expect("Failed to initialize database");
    
    // 运行数据库迁移
    database.migrate().await.expect("Failed to run migrations");
    
    let app_state = AppState {
        db: Arc::new(database),
    };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            // 用户管理
            auth::login,
            auth::logout,
            auth::get_current_user,
            users::get_users,
            users::create_user,
            users::update_user,
            users::delete_user,
            // 商品管理
            products::get_products,
            products::create_product,
            products::update_product,
            products::delete_product,
            categories::get_categories,
            categories::create_category,
            // 订单管理
            orders::get_orders,
            orders::create_order,
            orders::update_order,
            orders::get_order_details,
            // 系统管理
            roles::get_roles,
            logs::get_logs,
            stats::get_dashboard_stats
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
