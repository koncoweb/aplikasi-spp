use serde::{Deserialize, Serialize};
use crate::db;
use crate::middleware;
use tauri::State;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::commands::auth::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryResponse {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub description: Option<String>,
    pub color: String,
    pub sort_order: i32,
    pub is_active: bool,
    pub created_at: String,
}

impl From<db::Category> for CategoryResponse {
    fn from(c: db::Category) -> Self {
        CategoryResponse {
            id: c.id.to_string(),
            tenant_id: c.tenant_id.to_string(),
            name: c.name,
            description: c.description,
            color: c.color,
            sort_order: c.sort_order,
            is_active: c.is_active,
            created_at: c.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct GetCategoriesRequest {
    pub token: String,
    pub tenant_id: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateCategoryRequest {
    pub token: String,
    pub tenant_id: String,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub sort_order: Option<i32>,
}

#[tauri::command]
pub async fn get_categories(
    request: GetCategoriesRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<Vec<CategoryResponse>, String> {
    tracing::info!("Getting categories for tenant: {}", request.tenant_id);
    
    // Validate token and get auth context
    let app_state = state.read().await;
    let auth_context = middleware::validate_token(&request.token, &app_state).await?;
    drop(app_state);
    
    // Check permission
    middleware::require_permission(&auth_context, "categories:read")?;
    
    // Verify tenant access
    if let Some(ref auth_tenant_id) = auth_context.tenant_id {
        if auth_tenant_id != &request.tenant_id {
            return Err("Access denied to this tenant".to_string());
        }
    }
    
    let uuid = uuid::Uuid::parse_str(&request.tenant_id)
        .map_err(|e| format!("Invalid tenant ID: {}", e))?;
    
    let categories = db::Category::find_by_tenant(uuid)
        .await
        .map_err(|e| format!("Failed to get categories: {}", e))?;
    
    Ok(categories.into_iter().map(|c| c.into()).collect())
}

#[tauri::command]
pub async fn create_category(
    request: CreateCategoryRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<CategoryResponse, String> {
    tracing::info!("Creating category: {} for tenant: {}", request.name, request.tenant_id);
    
    // Validate token and get auth context
    let app_state = state.read().await;
    let auth_context = middleware::validate_token(&request.token, &app_state).await?;
    drop(app_state);
    
    // Check permission
    middleware::require_permission(&auth_context, "categories:write")?;
    
    // Verify tenant access
    if let Some(ref auth_tenant_id) = auth_context.tenant_id {
        if auth_tenant_id != &request.tenant_id {
            return Err("Access denied to this tenant".to_string());
        }
    }
    
    let tenant_uuid = uuid::Uuid::parse_str(&request.tenant_id)
        .map_err(|e| format!("Invalid tenant ID: {}", e))?;
    
    let pool = db::get_db_pool().await
        .map_err(|e| format!("Database error: {}", e))?;
    
    let row = sqlx::query_as::<_, CategoryRow>(
        "INSERT INTO categories (tenant_id, name, description, color, sort_order) 
         VALUES ($1, $2, $3, $4, $5) 
         RETURNING id, tenant_id, name, description, color, sort_order, is_active, created_at"
    )
    .bind(tenant_uuid)
    .bind(&request.name)
    .bind(&request.description)
    .bind(request.color.as_deref().unwrap_or("#3B82F6"))
    .bind(request.sort_order.unwrap_or(0))
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Failed to create category: {}", e))?;
    
    tracing::info!("Category created successfully: {}", row.id);
    
    Ok(CategoryResponse {
        id: row.id.to_string(),
        tenant_id: row.tenant_id.to_string(),
        name: row.name,
        description: row.description,
        color: row.color,
        sort_order: row.sort_order,
        is_active: row.is_active,
        created_at: row.created_at.to_rfc3339(),
    })
}

#[derive(Debug, sqlx::FromRow)]
struct CategoryRow {
    id: uuid::Uuid,
    tenant_id: uuid::Uuid,
    name: String,
    description: Option<String>,
    color: String,
    sort_order: i32,
    is_active: bool,
    created_at: chrono::DateTime<chrono::Utc>,
}
