use tauri::State;
use crate::{AppState, models::category::*};
use crate::api::ApiResponse;
use chrono::Utc;

#[tauri::command]
pub async fn get_categories(
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<CategoryWithParent>>, String> {
    let db = state.db.lock().await;
    
    let categories = sqlx::query_as::<_, CategoryWithParent>(
        r#"
        SELECT c.id, c.name, c.parent_id, p.name as parent_name, c.sort_order, c.created_at, c.updated_at
        FROM categories c
        LEFT JOIN categories p ON c.parent_id = p.id
        ORDER BY c.sort_order
        "#
    )
    .fetch_all(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(categories))
}

#[tauri::command]
pub async fn create_category(
    state: State<'_, AppState>,
    request: CreateCategoryRequest,
) -> Result<ApiResponse<Category>, String> {
    let db = state.db.lock().await;
    
    // 检查分类名是否已存在
    let existing = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM categories WHERE name = ?"
    )
    .bind(&request.name)
    .fetch_one(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    if existing > 0 {
        return Ok(ApiResponse::error("分类名已存在".to_string()));
    }

    let result = sqlx::query(
        "INSERT INTO categories (name, parent_id, sort_order, created_at, updated_at) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&request.name)
    .bind(request.parent_id)
    .bind(request.sort_order)
    .bind(Utc::now())
    .bind(Utc::now())
    .execute(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    let category = sqlx::query_as::<_, Category>(
        "SELECT * FROM categories WHERE id = ?"
    )
    .bind(result.last_insert_rowid())
    .fetch_one(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(category))
}

#[tauri::command]
pub async fn update_category(
    state: State<'_, AppState>,
    id: i64,
    request: UpdateCategoryRequest,
) -> Result<ApiResponse<Category>, String> {
    let db = state.db.lock().await;
    
    sqlx::query(
        "UPDATE categories SET name = ?, parent_id = ?, sort_order = ?, updated_at = ? WHERE id = ?"
    )
    .bind(&request.name)
    .bind(request.parent_id)
    .bind(request.sort_order)
    .bind(Utc::now())
    .bind(id)
    .execute(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    let category = sqlx::query_as::<_, Category>(
        "SELECT * FROM categories WHERE id = ?"
    )
    .bind(id)
    .fetch_one(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(category))
}

#[tauri::command]
pub async fn delete_category(
    state: State<'_, AppState>,
    id: i64,
) -> Result<ApiResponse<()>, String> {
    let db = state.db.lock().await;
    
    sqlx::query("DELETE FROM categories WHERE id = ?")
        .bind(id)
        .execute(&db.pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(()))
} 