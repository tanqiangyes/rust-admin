use tauri::State;
use sqlx::Row;
use crate::{AppState, models::product::*};
use crate::api::{ApiResponse, PaginatedResponse};

#[tauri::command]
pub async fn get_products(
    state: State<'_, AppState>,
    page: Option<i32>,
    per_page: Option<i32>,
    _search: Option<String>,
) -> Result<ApiResponse<PaginatedResponse<ProductWithCategory>>, String> {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;
    
    let db = state.db.lock().await;

    let base_query = r#"
        SELECT p.id, p.name, p.price, p.description, p.category_id, c.name as category_name,
               p.stock, p.images, p.status, p.created_at, p.updated_at
        FROM products p
        LEFT JOIN categories c ON p.category_id = c.id
        ORDER BY p.created_at DESC
        LIMIT ? OFFSET ?
    "#;

    let products = sqlx::query(base_query)
        .bind(per_page)
        .bind(offset)
        .fetch_all(&db.pool)
        .await
        .map_err(|e| e.to_string())?;

    let product_list: Vec<ProductWithCategory> = products
        .into_iter()
        .map(|row| ProductWithCategory {
            id: row.get("id"),
            name: row.get("name"),
            price: row.get("price"),
            description: row.get("description"),
            category_id: row.get("category_id"),
            category_name: row.get("category_name"),
            stock: row.get("stock"),
            images: row.get("images"),
            status: row.get("status"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .collect();

    let total = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM products")
        .fetch_one(&db.pool)
        .await
        .map_err(|e| e.to_string())?;

    let response = PaginatedResponse {
        items: product_list,
        total,
        page,
        per_page,
        total_pages: (total + per_page as i64 - 1) / per_page as i64,
    };

    Ok(ApiResponse::success(response))
}

#[tauri::command]
pub async fn create_product(
    state: State<'_, AppState>,
    request: CreateProductRequest,
) -> Result<ApiResponse<Product>, String> {
    let db = state.db.lock().await;
    
    // 将图片数组序列化为JSON字符串
    let images_json = serde_json::to_string(&request.images).unwrap_or_else(|_| "[]".to_string());
    
    let result = sqlx::query(
        "INSERT INTO products (name, price, description, category_id, stock, images, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&request.name)
    .bind(request.price)
    .bind(&request.description)
    .bind(request.category_id)
    .bind(request.stock)
    .bind(&images_json)
    .bind(1) // 默认状态为启用
    .bind(chrono::Utc::now())
    .bind(chrono::Utc::now())
    .execute(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    let product = sqlx::query_as::<_, Product>(
        "SELECT * FROM products WHERE id = ?"
    )
    .bind(result.last_insert_rowid())
    .fetch_one(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(product))
}

#[tauri::command]
pub async fn update_product(
    state: State<'_, AppState>,
    id: i64,
    request: UpdateProductRequest,
) -> Result<ApiResponse<Product>, String> {
    let db = state.db.lock().await;
    
    // 将图片数组序列化为JSON字符串
    let images_json = serde_json::to_string(&request.images).unwrap_or_else(|_| "[]".to_string());
    
    sqlx::query(
        "UPDATE products SET name = ?, price = ?, description = ?, category_id = ?, stock = ?, images = ?, status = ?, updated_at = ? WHERE id = ?"
    )
    .bind(&request.name)
    .bind(request.price)
    .bind(&request.description)
    .bind(request.category_id)
    .bind(request.stock)
    .bind(&images_json)
    .bind(request.status)
    .bind(chrono::Utc::now())
    .bind(id)
    .execute(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    let product = sqlx::query_as::<_, Product>(
        "SELECT * FROM products WHERE id = ?"
    )
    .bind(id)
    .fetch_one(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(product))
}

#[tauri::command]
pub async fn delete_product(
    state: State<'_, AppState>,
    id: i64,
) -> Result<ApiResponse<()>, String> {
    let db = state.db.lock().await;
    
    sqlx::query("DELETE FROM products WHERE id = ?")
        .bind(id)
        .execute(&db.pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(()))
} 