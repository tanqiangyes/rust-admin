use sqlx::SqlitePool;
use anyhow::Result;

pub async fn run_migrations(pool: &SqlitePool) -> Result<()> {
    // 用户表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            phone TEXT,
            address TEXT,
            avatar TEXT,
            role_id INTEGER NOT NULL DEFAULT 3,
            status INTEGER NOT NULL DEFAULT 1,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    // 角色表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS roles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            permissions TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    // 分类表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS categories (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            parent_id INTEGER,
            sort_order INTEGER DEFAULT 0,
            status INTEGER NOT NULL DEFAULT 1,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (parent_id) REFERENCES categories(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // 商品表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS products (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            price REAL NOT NULL,
            description TEXT,
            category_id INTEGER,
            stock INTEGER NOT NULL DEFAULT 0,
            images TEXT, -- 存储为 JSON 字符串
            status INTEGER NOT NULL DEFAULT 1,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (category_id) REFERENCES categories(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // 订单表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS orders (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            order_no TEXT NOT NULL UNIQUE,
            user_id INTEGER NOT NULL,
            total_amount REAL NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending',
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // 订单商品表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS order_items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            order_id INTEGER NOT NULL,
            product_id INTEGER NOT NULL,
            product_name TEXT NOT NULL,
            price REAL NOT NULL,
            quantity INTEGER NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (order_id) REFERENCES orders(id),
            FOREIGN KEY (product_id) REFERENCES products(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // 操作日志表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER,
            action TEXT NOT NULL,
            module TEXT NOT NULL,
            content TEXT,
            ip TEXT,
            user_agent TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // 系统设置表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS system_settings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            setting_key TEXT NOT NULL UNIQUE,
            setting_value TEXT NOT NULL,
            setting_type TEXT NOT NULL DEFAULT 'string',
            description TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    // 插入默认角色
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO roles (id, name, permissions)
        VALUES 
            (1, '超级管理员', '["*"]'),
            (2, '管理员', '["user:read", "user:write", "product:read", "product:write", "order:read", "order:write"]'),
            (3, '普通用户', '["order:read"]')
        "#,
    )
    .execute(pool)
    .await?;

    // 插入默认分类
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO categories (id, name, parent_id, sort_order, status)
        VALUES 
            (1, '电子产品', NULL, 1, 1),
            (2, '服装', NULL, 2, 1),
            (3, '图书', NULL, 3, 1),
            (4, '手机', 1, 1, 1),
            (5, '电脑', 1, 2, 1)
        "#,
    )
    .execute(pool)
    .await?;

    // 插入默认用户
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO users (id, username, email, password_hash, role_id, status)
        VALUES 
            (1, 'admin', 'admin@example.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewkwzLUW9wjLXfGa', 1, 1)
        "#,
    )
    .execute(pool)
    .await?;

    // 插入默认系统设置
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO system_settings (setting_key, setting_value, setting_type, description)
        VALUES 
            ('system_name', 'Rust Admin', 'string', '系统名称'),
            ('system_description', '基于 Tauri + Vue 3 的后台管理系统', 'string', '系统描述'),
            ('system_version', '1.0.0', 'string', '系统版本'),
            ('theme_color', '#1890ff', 'string', '主题颜色'),
            ('language', 'zh-CN', 'string', '系统语言'),
            ('page_size', '10', 'number', '默认页面大小'),
            ('enable_registration', 'false', 'boolean', '是否允许用户注册'),
            ('session_timeout', '3600', 'number', '会话超时时间（秒）'),
            ('max_login_attempts', '5', 'number', '最大登录尝试次数'),
            ('maintenance_mode', 'false', 'boolean', '维护模式')
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
} 