use sqlx::SqlitePool;
use anyhow::Result;

pub mod migrations;

pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub async fn new() -> Result<Self> {
        // 获取当前工作目录
        let current_dir = std::env::current_dir()?;
        println!("Current working directory: {:?}", current_dir);
        
        // 从当前目录向上找到项目根目录
        let project_root = if current_dir.ends_with("debug") {
            // 如果在 target/debug 目录，向上三级到项目根目录
            current_dir.parent()
                .and_then(|p| p.parent()) // target
                .and_then(|p| p.parent()) // src-tauri
                .unwrap_or(&current_dir)
        } else if current_dir.ends_with("src-tauri") {
            // 如果在 src-tauri 目录，向上一级到项目根目录
            current_dir.parent().unwrap_or(&current_dir)
        } else {
            // 否则假设就在项目根目录
            &current_dir
        };
        
        // 创建 data 目录路径
        let data_dir = project_root.join("data");
        
        // 确保 data 目录存在
        std::fs::create_dir_all(&data_dir)?;
        
        // 创建数据库文件路径
        let db_path = data_dir.join("rust-admin.db");
        let db_url = format!("sqlite:{}", db_path.display());
        
        println!("Project root: {:?}", project_root);
        println!("Data directory: {:?}", data_dir);
        println!("Database path: {}", db_url);
        
        // 检查目录权限
        println!("Data directory exists: {}", data_dir.exists());
        if let Ok(metadata) = data_dir.metadata() {
            println!("Data directory is writable: {}", !metadata.permissions().readonly());
        }
        
        // 连接数据库
        let pool = SqlitePool::connect(&db_url).await?;
        
        let db = Database { pool };
        
        // 运行数据库迁移
        migrations::run_migrations(&db.pool).await?;
        
        Ok(db)
    }
} 