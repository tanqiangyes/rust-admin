use tauri::State;
use crate::{AppState, models::category::{Category, CategoryWithParent, CreateCategoryRequest}};
use crate::api::ApiResponse;

#[tauri::command]
pub async fn get_categories(
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<CategoryWithParent>>, String> {
    let categories = sqlx::query_as::<_, CategoryWithParent>(
        r#"
        SELECT c.id, c.name, c.parent_id, p.name as parent_name, c.sort_order, c.created_at
        FROM categories c
        LEFT JOIN categories p ON c.parent_id = p.id
        ORDER BY c.sort_order, c.created_at
        "#
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(categories))
}

#[tauri::command]
pub async fn create_category(
    state: State<'_, AppState>,
    request: CreateCategoryRequest,
) -> Result<ApiResponse<Category>, String> {
    let sort_order = request.sort_order.unwrap_or(0);

    let category_id = sqlx::query_scalar::<_, i64>(
        "INSERT INTO categories (name, parent_id, sort_order) 
         VALUES (?, ?, ?) RETURNING id"
    )
    .bind(&request.name)
    .bind(request.parent_id)
    .bind(sort_order)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;

    let category = sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE id = ?")
        .bind(category_id)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(category))
}

#[tauri::command]
pub async fn update_category(
    state: State<'_, AppState>,
    id: i64,
    request: CreateCategoryRequest,
) -> Result<ApiResponse<Category>, String> {
    let sort_order = request.sort_order.unwrap_or(0);

    sqlx::query("UPDATE categories SET name = ?, parent_id = ?, sort_order = ? WHERE id = ?")
        .bind(&request.name)
        .bind(request.parent_id)
        .bind(sort_order)
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| e.to_string())?;

    let category = sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE id = ?")
        .bind(id)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(category))
}

#[tauri::command]
pub async fn delete_category(
    state: State<'_, AppState>,
    id: i64,
) -> Result<ApiResponse<()>, String> {
    sqlx::query("DELETE FROM categories WHERE id = ?")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(()))
} 