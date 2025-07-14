use tauri::State;
use sqlx::Row;
use crate::{AppState, models::order::*};
use crate::api::{ApiResponse, PaginatedResponse};

#[tauri::command]
pub async fn get_orders(
    state: State<'_, AppState>,
    page: Option<i32>,
    per_page: Option<i32>,
    _search: Option<String>,
    _status: Option<String>,
) -> Result<ApiResponse<PaginatedResponse<OrderWithUser>>, String> {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;
    
    let db = state.db.lock().await;

    let base_query = r#"
        SELECT o.id, o.order_no, o.user_id, u.username,
               o.total_amount, o.status, o.created_at, o.updated_at
        FROM orders o
        LEFT JOIN users u ON o.user_id = u.id
        ORDER BY o.created_at DESC
        LIMIT ? OFFSET ?
    "#;

    let orders = sqlx::query(base_query)
        .bind(per_page)
        .bind(offset)
        .fetch_all(&db.pool)
        .await
        .map_err(|e| e.to_string())?;

    let order_list: Vec<OrderWithUser> = orders
        .into_iter()
        .map(|row| OrderWithUser {
            id: row.get("id"),
            order_no: row.get("order_no"),
            user_id: row.get("user_id"),
            username: row.get("username"),
            total_amount: row.get("total_amount"),
            status: row.get("status"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .collect();

    let total_query = "SELECT COUNT(*) FROM orders";
    let total = sqlx::query_scalar::<_, i64>(total_query)
        .fetch_one(&db.pool)
        .await
        .map_err(|e| e.to_string())?;

    let response = PaginatedResponse {
        items: order_list,
        total,
        page,
        per_page,
        total_pages: (total + per_page as i64 - 1) / per_page as i64,
    };

    Ok(ApiResponse::success(response))
}

#[tauri::command]
pub async fn create_order(
    state: State<'_, AppState>,
    request: CreateOrderRequest,
) -> Result<ApiResponse<Order>, String> {
    let db = state.db.lock().await;
    
    let order_no = format!("ORD{}", chrono::Utc::now().timestamp());
    
    // 计算总金额（这里简化处理，实际应该根据商品计算）
    let total_amount = 0.0; // 需要根据实际业务逻辑计算
    
    let result = sqlx::query(
        "INSERT INTO orders (order_no, user_id, total_amount, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&order_no)
    .bind(request.user_id)
    .bind(total_amount)
    .bind(1) // 默认状态为待处理
    .bind(chrono::Utc::now())
    .bind(chrono::Utc::now())
    .execute(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    let order = sqlx::query_as::<_, Order>(
        "SELECT * FROM orders WHERE id = ?"
    )
    .bind(result.last_insert_rowid())
    .fetch_one(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(order))
}

#[tauri::command]
pub async fn update_order_status(
    state: State<'_, AppState>,
    id: i64,
    status: i32,
) -> Result<ApiResponse<Order>, String> {
    let db = state.db.lock().await;
    
    sqlx::query(
        "UPDATE orders SET status = ?, updated_at = ? WHERE id = ?"
    )
    .bind(status)
    .bind(chrono::Utc::now())
    .bind(id)
    .execute(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    let order = sqlx::query_as::<_, Order>(
        "SELECT * FROM orders WHERE id = ?"
    )
    .bind(id)
    .fetch_one(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(order))
} 