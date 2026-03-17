use serde::{Deserialize, Serialize};
use crate::db;
use crate::middleware;
use tauri::State;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::commands::auth::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct BranchResponse {
    pub id: String,
    pub tenant_id: String,
    pub code: String,
    pub name: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub is_main_branch: bool,
    pub is_active: bool,
    pub created_at: String,
}

impl From<db::Branch> for BranchResponse {
    fn from(b: db::Branch) -> Self {
        BranchResponse {
            id: b.id.to_string(),
            tenant_id: b.tenant_id.to_string(),
            code: b.code,
            name: b.name,
            address: b.address,
            phone: b.phone,
            is_main_branch: b.is_main_branch,
            is_active: b.is_active,
            created_at: b.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct GetBranchesRequest {
    pub token: String,
    pub tenant_id: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateBranchRequest {
    pub token: String,
    pub tenant_id: String,
    pub code: String,
    pub name: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub is_main_branch: bool,
}

#[tauri::command]
pub async fn get_branches(
    request: GetBranchesRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<Vec<BranchResponse>, String> {
    tracing::info!("Getting branches for tenant: {}", request.tenant_id);
    
    // Validate token and get auth context
    let app_state = state.read().await;
    let auth_context = middleware::validate_token(&request.token, &app_state).await?;
    drop(app_state);
    
    // Check permission
    middleware::require_permission(&auth_context, "branches:read")?;
    
    // Verify tenant access
    if let Some(ref auth_tenant_id) = auth_context.tenant_id {
        if auth_tenant_id != &request.tenant_id {
            return Err("Access denied to this tenant".to_string());
        }
    }
    
    let uuid = uuid::Uuid::parse_str(&request.tenant_id)
        .map_err(|e| format!("Invalid tenant ID: {}", e))?;
    
    let branches = db::Branch::find_by_tenant(uuid)
        .await
        .map_err(|e| format!("Failed to get branches: {}", e))?;
    
    Ok(branches.into_iter().map(|b| b.into()).collect())
}

#[tauri::command]
pub async fn create_branch(
    request: CreateBranchRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<BranchResponse, String> {
    tracing::info!("Creating branch: {} for tenant: {}", request.name, request.tenant_id);
    
    // Validate token and get auth context
    let app_state = state.read().await;
    let auth_context = middleware::validate_token(&request.token, &app_state).await?;
    drop(app_state);
    
    // Check permission
    middleware::require_permission(&auth_context, "branches:write")?;
    
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
    
    let row = sqlx::query_as::<_, BranchRow>(
        "INSERT INTO branches (tenant_id, code, name, address, phone, is_main_branch) 
         VALUES ($1, $2, $3, $4, $5, $6) 
         RETURNING id, tenant_id, code, name, address, phone, is_main_branch, is_active, created_at"
    )
    .bind(tenant_uuid)
    .bind(&request.code)
    .bind(&request.name)
    .bind(&request.address)
    .bind(&request.phone)
    .bind(request.is_main_branch)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Failed to create branch: {}", e))?;
    
    tracing::info!("Branch created successfully: {}", row.id);
    
    Ok(BranchResponse {
        id: row.id.to_string(),
        tenant_id: row.tenant_id.to_string(),
        code: row.code,
        name: row.name,
        address: row.address,
        phone: row.phone,
        is_main_branch: row.is_main_branch,
        is_active: row.is_active,
        created_at: row.created_at.to_rfc3339(),
    })
}

#[derive(Debug, sqlx::FromRow)]
struct BranchRow {
    id: uuid::Uuid,
    tenant_id: uuid::Uuid,
    code: String,
    name: String,
    address: Option<String>,
    phone: Option<String>,
    is_main_branch: bool,
    is_active: bool,
    created_at: chrono::DateTime<chrono::Utc>,
}
