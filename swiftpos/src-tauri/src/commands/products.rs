use serde::{Deserialize, Serialize};
use crate::db;
use crate::middleware;
use tauri::State;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductResponse {
    pub id: String,
    pub tenant_id: String,
    pub category_id: Option<String>,
    pub sku: String,
    pub barcode: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub unit: String,
    pub hpp: f64,
    pub selling_price: f64,
    pub stock: i32,
    pub stock_min: i32,
    pub is_active: bool,
    pub created_at: String,
}

impl From<db::Product> for ProductResponse {
    fn from(p: db::Product) -> Self {
        ProductResponse {
            id: p.id.to_string(),
            tenant_id: p.tenant_id.to_string(),
            category_id: p.category_id.map(|id| id.to_string()),
            sku: p.sku,
            barcode: p.barcode,
            name: p.name,
            description: p.description,
            unit: p.unit,
            hpp: p.hpp,
            selling_price: p.selling_price,
            stock: p.stock,
            stock_min: p.stock_min,
            is_active: p.is_active,
            created_at: p.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct GetProductsRequest {
    pub token: String,
    pub tenant_id: String,
}

/// Simplified: tenant_id is extracted from JWT, no need to send from frontend.
#[derive(Debug, Deserialize)]
pub struct CreateProductRequest {
    pub token: String,
    pub name: String,
    pub sku: Option<String>,
    pub unit: Option<String>,
    pub hpp: Option<f64>,
    pub selling_price: Option<f64>,
    pub stock: Option<i32>,
    pub stock_min: Option<i32>,
}

#[tauri::command]
pub async fn get_products(
    request: GetProductsRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<Vec<ProductResponse>, String> {
    tracing::info!("Getting products for tenant: {}", request.tenant_id);

    let app_state = state.read().await;
    let auth_context = middleware::validate_token(&request.token, &app_state).await?;
    drop(app_state);

    middleware::require_permission(&auth_context, "products:read")?;

    if let Some(ref auth_tenant_id) = auth_context.tenant_id {
        if auth_tenant_id != &request.tenant_id {
            return Err("Access denied to this tenant".to_string());
        }
    }

    let uuid = uuid::Uuid::parse_str(&request.tenant_id)
        .map_err(|e| format!("Invalid tenant ID: {}", e))?;

    let products = db::Product::find_by_tenant(uuid)
        .await
        .map_err(|e| format!("Failed to get products: {}", e))?;

    Ok(products.into_iter().map(|p| p.into()).collect())
}

#[tauri::command]
pub async fn create_product(
    request: CreateProductRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<ProductResponse, String> {
    let app_state = state.read().await;
    let auth_context = middleware::validate_token(&request.token, &app_state).await?;
    drop(app_state);

    middleware::require_permission(&auth_context, "products:write")?;

    // Extract tenant_id from JWT – no need to send from frontend
    let tenant_id_str = auth_context.tenant_id
        .ok_or_else(|| "No tenant associated with this user".to_string())?;
    let tenant_uuid = uuid::Uuid::parse_str(&tenant_id_str)
        .map_err(|e| format!("Invalid tenant ID in token: {}", e))?;

    // Auto-generate SKU if not provided
    let sku = request.sku
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| {
            let prefix = &request.name.chars().take(3).collect::<String>().to_uppercase();
            format!("{}-{}", prefix, &uuid::Uuid::new_v4().to_string()[..6].to_uppercase())
        });

    tracing::info!("Creating product '{}' for tenant: {}", request.name, tenant_id_str);

    let pool = db::get_db_pool().await
        .map_err(|e| format!("Database error: {}", e))?;

    let row = sqlx::query_as::<_, ProductRow>(
        "INSERT INTO products (tenant_id, sku, name, unit, hpp, selling_price, stock, stock_min)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
         RETURNING id, tenant_id, category_id, sku, barcode, name, description, unit,
         hpp, selling_price, stock, stock_min, is_active, created_at"
    )
    .bind(tenant_uuid)
    .bind(&sku)
    .bind(&request.name)
    .bind(request.unit.as_deref().unwrap_or("pcs"))
    .bind(request.hpp.unwrap_or(0.0))
    .bind(request.selling_price.unwrap_or(0.0))
    .bind(request.stock.unwrap_or(0))
    .bind(request.stock_min.unwrap_or(5))
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Failed to create product: {}", e))?;

    tracing::info!("Product created successfully: {}", row.id);

    Ok(ProductResponse {
        id: row.id.to_string(),
        tenant_id: row.tenant_id.to_string(),
        category_id: row.category_id.map(|id| id.to_string()),
        sku: row.sku,
        barcode: row.barcode,
        name: row.name,
        description: row.description,
        unit: row.unit,
        hpp: row.hpp,
        selling_price: row.selling_price,
        stock: row.stock,
        stock_min: row.stock_min,
        is_active: row.is_active,
        created_at: row.created_at.to_rfc3339(),
    })
}

#[derive(Debug, sqlx::FromRow)]
struct ProductRow {
    id: uuid::Uuid,
    tenant_id: uuid::Uuid,
    category_id: Option<uuid::Uuid>,
    sku: String,
    barcode: Option<String>,
    name: String,
    description: Option<String>,
    unit: String,
    hpp: f64,
    selling_price: f64,
    stock: i32,
    stock_min: i32,
    is_active: bool,
    created_at: chrono::DateTime<chrono::Utc>,
}
