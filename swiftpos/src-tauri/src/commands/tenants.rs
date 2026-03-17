use serde::{Deserialize, Serialize};
use crate::db;

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
pub async fn get_tenants() -> Result<Vec<TenantResponse>, String> {
    tracing::info!("Getting all tenants");
    
    // Only super admin can see all tenants
    // For demo, return empty list
    Ok(vec![])
}

#[tauri::command]
pub async fn get_tenant(id: String) -> Result<TenantResponse, String> {
    tracing::info!("Getting tenant: {}", id);
    
    let uuid = uuid::Uuid::parse_str(&id)
        .map_err(|e| format!("Invalid tenant ID: {}", e))?;
    
    let tenant = db::Tenant::find_by_id(uuid)
        .await
        .map_err(|e| format!("Tenant not found: {}", e))?;
    
    Ok(tenant.into())
}
