use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePool, Sqlite};
use anyhow::Result;

pub mod migrations;

#[derive(Clone)]
pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub async fn new() -> Result<Self> {
        let database_url = "sqlite:./data.db";
        
        // 创建数据库文件（如果不存在）
        if !Sqlite::database_exists(database_url).await.unwrap_or(false) {
            Sqlite::create_database(database_url).await?;
        }

        let pool = SqlitePool::connect(database_url).await?;
        
        Ok(Database { pool })
    }

    pub async fn migrate(&self) -> Result<()> {
        migrations::run_migrations(&self.pool).await
    }
} 