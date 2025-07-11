use tauri::State;
use sqlx::Row; // 添加这个导入
use crate::{AppState, models::order::*};
use crate::api::{ApiResponse, PaginatedResponse};

#[tauri::command]
pub async fn get_orders(
    state: State<'_, AppState>,
    page: Option<i32>,
    per_page: Option<i32>,
    search: Option<String>,
    status: Option<String>,
) -> Result<ApiResponse<PaginatedResponse<OrderWithUser>>, String> {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let mut base_query = r#"
        SELECT o.id, o.order_no, o.user_id, u.username,
               o.total_amount, o.status, o.created_at, o.updated_at
        FROM orders o
        LEFT JOIN users u ON o.user_id = u.id
        WHERE 1=1
    "#.to_string();

    let mut params = Vec::new();
    let mut param_count = 0;

    if let Some(search_term) = &search {
        if !search_term.is_empty() {
            param_count += 1;
            base_query.push_str(&format!(" AND o.order_no LIKE ?{}", param_count));
            params.push(format!("%{}%", search_term));
        }
    }

    if let Some(status_filter) = &status {
        if !status_filter.is_empty() {
            param_count += 1;
            base_query.push_str(&format!(" AND o.status = ?{}", param_count));
            params.push(status_filter.clone());
        }
    }

    let count_query = format!("SELECT COUNT(*) FROM ({}) as counted", base_query);
    let data_query = format!("{} ORDER BY o.created_at DESC LIMIT {} OFFSET {}", base_query, per_page, offset);

    let mut count_query_builder = sqlx::query(&count_query);
    let mut data_query_builder = sqlx::query(&data_query);

    for param in &params {
        count_query_builder = count_query_builder.bind(param);
        data_query_builder = data_query_builder.bind(param);
    }

    let total = count_query_builder
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| e.to_string())?
        .get::<i64, _>(0);

    let rows = data_query_builder
        .fetch_all(&state.db.pool)
        .await
        .map_err(|e| e.to_string())?;

    let mut orders = Vec::new();
    for row in rows {
        orders.push(OrderWithUser {
            id: row.get("id"),
            order_no: row.get("order_no"),
            user_id: row.get("user_id"),
            username: row.get("username"),
            total_amount: row.get("total_amount"),
            status: row.get("status"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        });
    }

    Ok(ApiResponse::success(PaginatedResponse {
        items: orders,
        total,
        page,
        per_page,
    }))
}

#[tauri::command]
pub async fn get_order_by_id(
    state: State<'_, AppState>,
    order_id: i64,
) -> Result<ApiResponse<Order>, String> {
    let order = sqlx::query_as::<_, Order>(
        "SELECT * FROM orders WHERE id = ?"
    )
    .bind(order_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(order))
}

#[tauri::command]
pub async fn update_order_status(
    state: State<'_, AppState>,
    order_id: i64,
    status: String,
) -> Result<ApiResponse<()>, String> {
    sqlx::query(
        "UPDATE orders SET status = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
    )
    .bind(status)
    .bind(order_id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(()))
} 