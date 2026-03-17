use serde::{Deserialize, Serialize};
use crate::db;
use crate::middleware;
use tauri::State;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::commands::auth::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct TenantResponse {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub logo_url: Option<String>,
    pub application_name: String,
    pub subscription_tier: String,
    pub timezone: String,
    pub currency_code: String,
    pub currency_symbol: String,
    pub is_active: bool,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct GetTenantRequest {
    pub token: String,
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct GetTenantsRequest {
    pub token: String,
}

impl From<db::Tenant> for TenantResponse {
    fn from(t: db::Tenant) -> Self {
        TenantResponse {
            id: t.id.to_string(),
            name: t.name,
            slug: t.slug,
            address: t.address,
            phone: t.phone,
            email: t.email,
            logo_url: t.logo_url,
            application_name: t.application_name,
            subscription_tier: t.subscription_tier,
            timezone: t.timezone,
            currency_code: t.currency_code,
            currency_symbol: t.currency_symbol,
            is_active: t.is_active,
            created_at: t.created_at.to_rfc3339(),
        }
    }
}

#[tauri::command]
pub async fn get_tenants(
    request: GetTenantsRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<Vec<TenantResponse>, String> {
    tracing::info!("Getting all tenants");
    
    // Validate token and get auth context
    let app_state = state.read().await;
    let auth_context = middleware::validate_token(&request.token, &app_state).await?;
    drop(app_state);
    
    // Check permission - only super_admin can see all tenants
    middleware::require_role(&auth_context, "super_admin")?;
    
    // Return all tenants - filtered by super_admin access
    let tenants = db::Tenant::find_all()
        .await
        .map_err(|e| format!("Failed to get tenants: {}", e))?;
    
    Ok(tenants.into_iter().map(|t| t.into()).collect())
}

#[tauri::command]
pub async fn get_tenant(
    request: GetTenantRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<TenantResponse, String> {
    tracing::info!("Getting tenant: {}", request.id);
    
    // Validate token and get auth context
    let app_state = state.read().await;
    let auth_context = middleware::validate_token(&request.token, &app_state).await?;
    drop(app_state);
    
    // Check permission
    middleware::require_permission(&auth_context, "tenants:read")?;
    
    // Verify tenant access - users can only view their own tenant unless super_admin
    if let Some(ref auth_tenant_id) = auth_context.tenant_id {
        if auth_tenant_id != &request.id && auth_context.role != "super_admin" {
            return Err("Access denied to this tenant".to_string());
        }
    }
    
    let uuid = uuid::Uuid::parse_str(&request.id)
        .map_err(|e| format!("Invalid tenant ID: {}", e))?;
    
    let tenant = db::Tenant::find_by_id(uuid)
        .await
        .map_err(|e| format!("Tenant not found: {}", e))?;
    
    Ok(tenant.into())
}
