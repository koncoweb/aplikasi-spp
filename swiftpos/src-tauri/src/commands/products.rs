use serde::{Deserialize, Serialize};
use crate::db;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductResponse {
    pub id: String,
    pub tenant_id: String,
    pub category_id: Option<String>,
    pub sku: String,
    pub barcode: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub is_variant: bool,
    pub parent_product_id: Option<String>,
    pub variant_name: Option<String>,
    pub unit: String,
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
            image_url: p.image_url,
            is_variant: p.is_variant,
            parent_product_id: p.parent_product_id.map(|id| id.to_string()),
            variant_name: p.variant_name,
            unit: p.unit,
            is_active: p.is_active,
            created_at: p.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateProductRequest {
    pub tenant_id: String,
    pub category_id: Option<String>,
    pub sku: String,
    pub barcode: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub unit: Option<String>,
}

#[tauri::command]
pub async fn get_products(tenant_id: String) -> Result<Vec<ProductResponse>, String> {
    tracing::info!("Getting products for tenant: {}", tenant_id);
    
    let uuid = uuid::Uuid::parse_str(&tenant_id)
        .map_err(|e| format!("Invalid tenant ID: {}", e))?;
    
    let products = db::Product::find_by_tenant(uuid)
        .await
        .map_err(|e| format!("Failed to get products: {}", e))?;
    
    Ok(products.into_iter().map(|p| p.into()).collect())
}

#[tauri::command]
pub async fn create_product(request: CreateProductRequest) -> Result<ProductResponse, String> {
    tracing::info!("Creating product: {} for tenant: {}", request.name, request.tenant_id);
    
    let tenant_uuid = uuid::Uuid::parse_str(&request.tenant_id)
        .map_err(|e| format!("Invalid tenant ID: {}", e))?;
    
    let category_uuid = if let Some(ref cat_id) = request.category_id {
        Some(uuid::Uuid::parse_str(cat_id).map_err(|e| format!("Invalid category ID: {}", e))?)
    } else {
        None
    };
    
    let pool = db::get_db_pool().await
        .map_err(|e| format!("Database error: {}", e))?;
    
    let row = sqlx::query_as::<_, ProductRow>(
        "INSERT INTO products (tenant_id, category_id, sku, barcode, name, description, image_url, unit) 
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8) 
         RETURNING id, tenant_id, category_id, sku, barcode, name, description, image_url, 
         is_variant, parent_product_id, variant_name, unit, is_active, created_at, updated_at"
    )
    .bind(tenant_uuid)
    .bind(category_uuid)
    .bind(&request.sku)
    .bind(&request.barcode)
    .bind(&request.name)
    .bind(&request.description)
    .bind(&request.image_url)
    .bind(request.unit.as_deref().unwrap_or("pcs"))
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
        image_url: row.image_url,
        is_variant: row.is_variant,
        parent_product_id: row.parent_product_id.map(|id| id.to_string()),
        variant_name: row.variant_name,
        unit: row.unit,
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
    image_url: Option<String>,
    is_variant: bool,
    parent_product_id: Option<uuid::Uuid>,
    variant_name: Option<String>,
    unit: String,
    is_active: bool,
    created_at: chrono::DateTime<chrono::Utc>,
    _updated_at: chrono::DateTime<chrono::Utc>,
}
