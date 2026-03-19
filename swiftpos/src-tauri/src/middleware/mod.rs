// ============================================================
// SwiftPOS Middleware — JWT validation (email+password auth)
// No external auth provider. All validation done locally.
// ============================================================
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::commands::auth::decode_token;

/// Konteks autentikasi hasil validasi JWT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthContext {
    pub user_id: String,
    pub email: String,
    pub name: String,
    pub role: String,
    pub tenant_id: Option<String>,
    pub branch_id: Option<String>,
    pub permissions: Vec<String>,
}

impl AuthContext {
    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.contains(&permission.to_string())
    }
    pub fn has_role(&self, role: &str) -> bool {
        self.role == role
    }
    pub fn is_admin(&self) -> bool {
        matches!(self.role.as_str(), "super_admin" | "owner" | "manager")
    }
}

/// Validasi JWT dan kembalikan AuthContext. Digunakan di semua command yang butuh auth.
pub async fn validate_token(
    token: &str,
    state: &AppState,
) -> Result<AuthContext, String> {
    let claims = decode_token(token, &state.jwt_secret)?;

    let permissions = get_role_permissions(&claims.role);

    Ok(AuthContext {
        user_id: claims.sub.clone(),
        email: claims.email.clone(),
        name: claims.email.clone(), // full_name tidak ada di claims; gunakan email sebagai fallback
        role: claims.role,
        tenant_id: claims.tenant_id,
        branch_id: claims.branch_id,
        permissions,
    })
}

/// Mapping role → daftar permission
pub fn get_role_permissions(role: &str) -> Vec<String> {
    match role {
        "super_admin" => vec![
            "tenants:read", "tenants:write", "tenants:delete",
            "branches:read", "branches:write", "branches:delete",
            "users:read", "users:write", "users:delete",
            "categories:read", "categories:write", "categories:delete",
            "products:read", "products:write", "products:delete",
            "transactions:read", "transactions:write", "transactions:void",
            "reports:read", "reports:export",
            "settings:read", "settings:write",
            "printers:read", "printers:write", "printers:test",
        ].iter().map(|s| s.to_string()).collect(),
        "owner" => vec![
            "branches:read", "branches:write", "branches:delete",
            "users:read", "users:write", "users:delete",
            "categories:read", "categories:write", "categories:delete",
            "products:read", "products:write", "products:delete",
            "transactions:read", "transactions:write", "transactions:void",
            "reports:read", "reports:export",
            "settings:read", "settings:write",
            "printers:read", "printers:write", "printers:test",
        ].iter().map(|s| s.to_string()).collect(),
        "manager" => vec![
            "branches:read", "branches:write",
            "users:read", "users:write",
            "categories:read", "categories:write", "categories:delete",
            "products:read", "products:write", "products:delete",
            "transactions:read", "transactions:write", "transactions:void",
            "reports:read", "reports:export",
            "settings:read",
            "printers:read", "printers:test",
        ].iter().map(|s| s.to_string()).collect(),
        "kasir" | "cashier" => vec![
            "products:read",
            "transactions:read", "transactions:write",
            "printers:read", "printers:test",
        ].iter().map(|s| s.to_string()).collect(),
        _ => vec![],
    }
}

/// Helper: require specific permission
pub fn require_permission(auth: &AuthContext, permission: &str) -> Result<(), String> {
    if auth.has_permission(permission) {
        Ok(())
    } else {
        Err(format!("Akses ditolak: butuh izin '{}'", permission))
    }
}

/// Helper: require specific role
pub fn require_role(auth: &AuthContext, role: &str) -> Result<(), String> {
    if auth.has_role(role) {
        Ok(())
    } else {
        Err(format!("Akses ditolak: butuh role '{}'", role))
    }
}
