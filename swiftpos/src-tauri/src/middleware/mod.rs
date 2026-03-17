use serde::{Deserialize, Serialize};
use crate::db;
use crate::commands::auth::{AppState, NeonAuthUser, NeonAuthSession, VerifyNeonTokenResponse};

/// Claims extracted from Neon Auth token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeonAuthClaims {
    pub sub: String,           // Stack Auth user ID
    pub email: String,
    pub name: Option<String>,
    pub email_verified: bool,
}

/// Authenticated user context with tenant and branch info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthContext {
    pub neon_user_id: String,
    pub email: String,
    pub name: String,
    pub role: String,              // SwiftPOS role: super_admin, owner, manager, cashier
    pub tenant_id: Option<String>,
    pub branch_id: Option<String>,
    pub permissions: Vec<String>,
}

impl AuthContext {
    /// Check if user has a specific permission
    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.contains(&permission.to_string())
    }

    /// Check if user has a specific role
    pub fn has_role(&self, role: &str) -> bool {
        self.role == role
    }

    /// Check if user is tenant admin or higher
    pub fn is_admin(&self) -> bool {
        matches!(self.role.as_str(), "super_admin" | "owner" | "manager")
    }
}

/// Role-based permissions mapping
pub fn get_role_permissions(role: &str) -> Vec<String> {
    match role {
        "super_admin" => vec![
            "tenants:read".to_string(), "tenants:write".to_string(), "tenants:delete".to_string(),
            "branches:read".to_string(), "branches:write".to_string(), "branches:delete".to_string(),
            "users:read".to_string(), "users:write".to_string(), "users:delete".to_string(),
            "categories:read".to_string(), "categories:write".to_string(), "categories:delete".to_string(),
            "products:read".to_string(), "products:write".to_string(), "products:delete".to_string(),
            "transactions:read".to_string(), "transactions:write".to_string(), "transactions:void".to_string(),
            "reports:read".to_string(), "reports:export".to_string(),
            "settings:read".to_string(), "settings:write".to_string(),
            "printers:read".to_string(), "printers:write".to_string(), "printers:test".to_string(),
        ],
        "owner" => vec![
            "branches:read".to_string(), "branches:write".to_string(), "branches:delete".to_string(),
            "users:read".to_string(), "users:write".to_string(), "users:delete".to_string(),
            "categories:read".to_string(), "categories:write".to_string(), "categories:delete".to_string(),
            "products:read".to_string(), "products:write".to_string(), "products:delete".to_string(),
            "transactions:read".to_string(), "transactions:write".to_string(), "transactions:void".to_string(),
            "reports:read".to_string(), "reports:export".to_string(),
            "settings:read".to_string(), "settings:write".to_string(),
            "printers:read".to_string(), "printers:write".to_string(), "printers:test".to_string(),
        ],
        "manager" => vec![
            "branches:read".to_string(), "branches:write".to_string(),
            "users:read".to_string(), "users:write".to_string(),
            "categories:read".to_string(), "categories:write".to_string(), "categories:delete".to_string(),
            "products:read".to_string(), "products:write".to_string(), "products:delete".to_string(),
            "transactions:read".to_string(), "transactions:write".to_string(), "transactions:void".to_string(),
            "reports:read".to_string(), "reports:export".to_string(),
            "settings:read".to_string(),
            "printers:read".to_string(), "printers:test".to_string(),
        ],
        "cashier" => vec![
            "products:read".to_string(),
            "transactions:read".to_string(), "transactions:write".to_string(),
            "printers:read".to_string(), "printers:test".to_string(),
        ],
        _ => vec![],
    }
}

/// Validate Neon Auth token and build auth context
pub async fn validate_token(
    token: &str,
    state: &AppState,
) -> Result<AuthContext, String> {
    // First verify the token with Stack Auth API
    let neon_auth_response = verify_neon_token_api(token, state).await?;

    if !neon_auth_response.valid {
        return Err(neon_auth_response.error.unwrap_or_else(|| "Invalid token".to_string()));
    }

    let neon_user = neon_auth_response.user.ok_or("No user in token response")?;

    // Try to find local user by neon_user_id or email
    // If not found, create a new user based on Neon Auth data
    let local_user = find_or_create_local_user(&neon_user).await?;

    // Get role permissions
    let permissions = get_role_permissions(&local_user.role);

    Ok(AuthContext {
        neon_user_id: neon_user.id,
        email: local_user.email.clone(),
        name: local_user.full_name.clone(),
        role: local_user.role,
        tenant_id: local_user.tenant_id.map(|id| id.to_string()),
        branch_id: local_user.branch_id.map(|id| id.to_string()),
        permissions,
    })
}

/// Call Stack Auth API to verify token
async fn verify_neon_token_api(
    token: &str,
    state: &AppState,
) -> Result<VerifyNeonTokenResponse, String> {
    let auth_url = format!(
        "https://api.stack-auth.com/api/v1/projects/{}/auth/verify-token",
        state.neon_project_id
    );

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| format!("Client error: {}", e))?;

    let response = client
        .post(&auth_url)
        .header("Authorization", format!("Bearer {}", state.stack_secret_key))
        .json(&serde_json::json!({ "token": token }))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if response.status().is_success() {
        let session: NeonAuthSession = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        Ok(VerifyNeonTokenResponse {
            valid: true,
            user: Some(NeonAuthUser {
                id: session.user.id,
                email: session.user.email,
                name: session.user.name,
                email_verified: session.user.email_verified,
            }),
            error: None,
        })
    } else {
        Ok(VerifyNeonTokenResponse {
            valid: false,
            user: None,
            error: Some(format!("Token verification failed: {}", response.status())),
        })
    }
}

/// Find local user by neon_user_id or email, or create new one
async fn find_or_create_local_user(
    neon_user: &NeonAuthUser,
) -> Result<db::User, String> {
    // Try to find by email first
    match db::User::find_by_email(&neon_user.email).await {
        Ok(user) => {
            // Check if user is active
            if !user.is_active {
                return Err(format!(
                    "User {} is deactivated. Please contact administrator.",
                    neon_user.email
                ));
            }
            Ok(user)
        },
        Err(_) => {
            // User doesn't exist - return error with guidance
            Err(format!(
                "User {} not found. Please contact administrator to provision access.",
                neon_user.email
            ))
        }
    }
}

/// Require specific permission - helper for command handlers
pub fn require_permission(auth: &AuthContext, permission: &str) -> Result<(), String> {
    if auth.has_permission(permission) {
        Ok(())
    } else {
        Err(format!("Permission denied: {}", permission))
    }
}

/// Require specific role - helper for command handlers
pub fn require_role(auth: &AuthContext, role: &str) -> Result<(), String> {
    if auth.has_role(role) {
        Ok(())
    } else {
        Err(format!("Role '{}' required", role))
    }
}
