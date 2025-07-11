use tauri::State;
use sqlx::Row;
use crate::{AppState, models::product::*};
use crate::api::{ApiResponse, PaginatedResponse};

#[tauri::command]
pub async fn get_products(
    state: State<'_, AppState>,
    page: Option<i32>,
    per_page: Option<i32>,
    search: Option<String>,
    category_id: Option<i64>,
) -> Result<ApiResponse<PaginatedResponse<ProductWithCategory>>, String> {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let mut base_query = r#"
        SELECT p.id, p.name, p.price, p.description, p.category_id, c.name as category_name,
               p.stock, p.images, p.status, p.created_at, p.updated_at
        FROM products p
        LEFT JOIN categories c ON p.category_id = c.id
        WHERE 1=1
    "#.to_string();

    let mut params = Vec::new();
    let mut param_count = 0;

    if let Some(search_term) = &search {
        if !search_term.is_empty() {
            param_count += 1;
            base_query.push_str(&format!(" AND p.name LIKE ?{}", param_count));
            params.push(format!("%{}%", search_term));
        }
    }

    if let Some(cat_id) = category_id {
        param_count += 1;
        base_query.push_str(&format!(" AND p.category_id = ?{}", param_count));
        params.push(cat_id.to_string());
    }

    let count_query = format!("SELECT COUNT(*) FROM ({}) as counted", base_query);
    let data_query = format!("{} ORDER BY p.created_at DESC LIMIT {} OFFSET {}", base_query, per_page, offset);

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

    let mut products = Vec::new();
    for row in rows {
        // 处理图片字段，从 JSON 字符串转换为 Vec<String>
        let images_json: Option<String> = row.get("images");
        let images = if let Some(json_str) = images_json {
            serde_json::from_str::<Vec<String>>(&json_str).ok()
        } else {
            None
        };

        products.push(ProductWithCategory {
            id: row.get("id"),
            name: row.get("name"),
            price: row.get("price"),
            description: row.get("description"),
            category_id: row.get("category_id"),
            category_name: row.get("category_name"),
            stock: row.get("stock"),
            images,
            status: row.get("status"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        });
    }

    Ok(ApiResponse::success(PaginatedResponse {
        items: products,
        total,
        page,
        per_page,
    }))
}

#[tauri::command]
pub async fn create_product(
    state: State<'_, AppState>,
    request: CreateProductRequest,
) -> Result<ApiResponse<Product>, String> {
    // 将 Vec<String> 转换为 JSON 字符串
    let images_json = if let Some(images) = &request.images {
        Some(serde_json::to_string(images).map_err(|e| e.to_string())?)
    } else {
        None
    };

    let product = sqlx::query_as::<_, Product>(
        r#"
        INSERT INTO products (name, price, description, category_id, stock, images, status)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        RETURNING *
        "#
    )
    .bind(&request.name)
    .bind(request.price)
    .bind(&request.description)
    .bind(request.category_id)
    .bind(request.stock)
    .bind(images_json)
    .bind(request.status)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(product))
}

#[tauri::command]
pub async fn update_product(
    state: State<'_, AppState>,
    product_id: i64,
    request: UpdateProductRequest,
) -> Result<ApiResponse<Product>, String> {
    // 将 Vec<String> 转换为 JSON 字符串
    let images_json = if let Some(images) = &request.images {
        Some(serde_json::to_string(images).map_err(|e| e.to_string())?)
    } else {
        None
    };

    let product = sqlx::query_as::<_, Product>(
        r#"
        UPDATE products 
        SET name = COALESCE(?, name),
            price = COALESCE(?, price),
            description = COALESCE(?, description),
            category_id = COALESCE(?, category_id),
            stock = COALESCE(?, stock),
            images = COALESCE(?, images),
            status = COALESCE(?, status),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ?
        RETURNING *
        "#
    )
    .bind(&request.name)
    .bind(request.price)
    .bind(&request.description)
    .bind(request.category_id)
    .bind(request.stock)
    .bind(images_json)
    .bind(request.status)
    .bind(product_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(product))
}

#[tauri::command]
pub async fn delete_product(
    state: State<'_, AppState>,
    product_id: i64,
) -> Result<ApiResponse<()>, String> {
    sqlx::query("DELETE FROM products WHERE id = ?")
        .bind(product_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(()))
} 