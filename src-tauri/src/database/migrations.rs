use sqlx::SqlitePool;
use anyhow::Result;

pub async fn run_migrations(pool: &SqlitePool) -> Result<()> {
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
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (role_id) REFERENCES roles(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // 确保管理员有正确的权限
    sqlx::query(
        r#"
        INSERT OR REPLACE INTO roles (id, name, permissions)
        VALUES 
            (1, '超级管理员', '["*"]'),
            (2, '管理员', '["dashboard:read", "user:read", "user:write", "product:read", "product:write", "order:read", "order:write", "category:read", "category:write"]'),
            (3, '普通用户', '["dashboard:read", "order:read"]')
        "#,
    )
    .execute(pool)
    .await?;

    // 创建默认用户
    let admin_password_hash = bcrypt::hash("admin123", bcrypt::DEFAULT_COST)
        .unwrap_or_else(|_| "$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewkwzLUW9wjLXfGa".to_string());

    let manager_password_hash = bcrypt::hash("manager123", bcrypt::DEFAULT_COST)
        .unwrap_or_else(|_| "$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewkwzLUW9wjLXfGa".to_string());

    let user_password_hash = bcrypt::hash("user123", bcrypt::DEFAULT_COST)
        .unwrap_or_else(|_| "$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewkwzLUW9wjLXfGa".to_string());

    // 插入测试用户
    sqlx::query(
        r#"
        INSERT OR REPLACE INTO users (id, username, email, password_hash, role_id, status)
        VALUES 
            (1, 'admin', 'admin@example.com', ?, 1, 1),
            (2, 'manager', 'manager@example.com', ?, 2, 1),
            (3, 'user', 'user@example.com', ?, 3, 1)
        "#,
    )
    .bind(&admin_password_hash)
    .bind(&manager_password_hash)
    .bind(&user_password_hash)
    .execute(pool)
    .await?;

    println!("Created test users:");
    println!("  admin/admin123 (超级管理员)");
    println!("  manager/manager123 (管理员)");
    println!("  user/user123 (普通用户)");

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
            images TEXT,
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

    // 插入默认设置
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO system_settings (setting_key, setting_value, setting_type, description)
        VALUES 
            ('system_name', 'Rust Admin', 'string', '系统名称'),
            ('system_description', '基于 Tauri + Vue 3 的后台管理系统', 'string', '系统描述'),
            ('system_version', '1.0.0', 'string', '系统版本'),
            ('theme_color', '#1890ff', 'string', '主题颜色'),
            ('language', 'zh-CN', 'string', '系统语言'),
            ('page_size', '10', 'number', '默认页面大小')
        "#,
    )
    .execute(pool)
    .await?;

    // 插入一些测试分类
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO categories (id, name, parent_id, sort_order, status)
        VALUES 
            (1, '电子产品', NULL, 1, 1),
            (2, '服装', NULL, 2, 1),
            (3, '图书', NULL, 3, 1)
        "#,
    )
    .execute(pool)
    .await?;

    println!("Database migrations completed successfully!");

    // 在迁移的最后，添加权限验证
    println!("Verifying permissions...");

    // 检查管理员权限
    let manager_permissions = sqlx::query_scalar::<_, String>(
        "SELECT permissions FROM roles WHERE name = '管理员'"
    )
    .fetch_one(pool)
    .await?;

    println!("Manager permissions: {}", manager_permissions);

    // 检查是否包含 user:read 权限
    if manager_permissions.contains("user:read") {
        println!("✓ Manager has user:read permission");
    } else {
        println!("✗ Manager missing user:read permission");
    }

    Ok(())
} 