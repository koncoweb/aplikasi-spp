use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Header, EncodingKey, decode, DecodingKey, Validation};
use chrono::{Utc, Duration};
use crate::db::{self, DbError};
use crate::middleware;
use tauri::State;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvisionUserRequest {
    pub email: String,           // Stack Auth email
    pub full_name: String,
    pub role: String,            // super_admin, owner, manager, cashier
    pub tenant_id: Option<String>,
    pub branch_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvisionUserResponse {
    pub success: bool,
    pub user: Option<UserResponse>,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeonAuthLoginRequest {
    pub token: String,  // Stack Auth token from frontend
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeonAuthLoginResponse {
    pub success: bool,
    pub user: Option<UserResponse>,
    pub tenant: Option<TenantResponse>,
    pub permissions: Vec<String>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
    pub tenant: Option<TenantResponse>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub full_name: String,
    pub role: String,
    pub tenant_id: Option<String>,
    pub branch_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenantResponse {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub application_name: String,
    pub currency_symbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterTenantRequest {
    pub name: String,
    pub slug: String,
    pub email: String,
    pub password: String,
    pub full_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub email: String,
    pub role: String,
    pub tenant_id: Option<String>,
    pub branch_id: Option<String>,
    pub exp: i64,
}

// AppState now lives in lib.rs, we reference it here
pub struct AppState {
    pub jwt_secret: String,
    pub neon_project_id: String,
    pub stack_secret_key: String,
}

fn create_token(user: &db::User, jwt_secret: &str) -> Result<String, DbError> {
    let exp = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("Invalid timestamp")
        .timestamp();
    
    let claims = Claims {
        sub: user.id.to_string(),
        email: user.email.clone(),
        role: user.role.clone(),
        tenant_id: user.tenant_id.map(|id| id.to_string()),
        branch_id: user.branch_id.map(|id| id.to_string()),
        exp,
    };
    
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    ).map_err(|e| DbError::Validation(e.to_string()))?;
    
    Ok(token)
}

pub fn decode_token(token: &str, jwt_secret: &str) -> Result<Claims, String> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    ).map_err(|e| format!("Invalid token: {}", e))?;
    
    Ok(token_data.claims)
}

#[tauri::command]
pub async fn login(
    request: LoginRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<LoginResponse, String> {
    tracing::info!("Login attempt for email: {}", request.email);
    
    // Get JWT secret from state
    let app_state = state.read().await;
    let jwt_secret = app_state.jwt_secret.clone();
    drop(app_state);
    
    // Find user by email
    let user = db::User::find_by_email(&request.email)
        .await
        .map_err(|e| {
            tracing::error!("User not found: {}", e);
            "Invalid email or password".to_string()
        })?;
    
    // Check if user is active
    if !user.is_active {
        tracing::error!("User account is inactive");
        return Err("Account is inactive".to_string());
    }
    
    // Verify password using argon2
    let password_valid = user.verify_password(&request.password).await;
    if !password_valid {
        tracing::error!("Invalid password for user: {}", request.email);
        return Err("Invalid email or password".to_string());
    }
    
    // Update last login
    user.update_last_login().await.ok();
    
    // Get tenant info if available
    let tenant = if let Some(tenant_id) = user.tenant_id {
        match db::Tenant::find_by_id(tenant_id).await {
            Ok(t) => Some(TenantResponse {
                id: t.id.to_string(),
                name: t.name.clone(),
                slug: t.slug.clone(),
                application_name: t.application_name,
                currency_symbol: t.currency_symbol,
            }),
            Err(_) => None,
        }
    } else {
        None
    };
    
    // Create token
    let token = create_token(&user, &jwt_secret)
        .map_err(|e| e.to_string())?;
    
    tracing::info!("Login successful for user: {}", request.email);
    
    Ok(LoginResponse {
        token,
        user: UserResponse {
            id: user.id.to_string(),
            email: user.email,
            full_name: user.full_name,
            role: user.role,
            tenant_id: user.tenant_id.map(|id| id.to_string()),
            branch_id: user.branch_id.map(|id| id.to_string()),
        },
        tenant,
    })
}

#[tauri::command]
pub async fn logout() -> Result<(), String> {
    tracing::info!("User logged out");
    Ok(())
}

/// Neon Auth login - integrates Stack Auth with local SwiftPOS users
#[tauri::command]
pub async fn neon_auth_login(
    request: NeonAuthLoginRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<NeonAuthLoginResponse, String> {
    tracing::info!("Neon Auth login attempt");
    
    // Get app state
    let app_state = state.read().await;
    let neon_project_id = app_state.neon_project_id.clone();
    let stack_secret_key = app_state.stack_secret_key.clone();
    drop(app_state);
    
    // Verify token with Stack Auth API
    let auth_url = format!(
        "https://api.stack-auth.com/api/v1/projects/{}/auth/verify-token",
        neon_project_id
    );

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| format!("Client error: {}", e))?;
    
    let response = client
        .post(&auth_url)
        .header("Authorization", format!("Bearer {}", stack_secret_key))
        .json(&serde_json::json!({ "token": request.token }))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.status().is_success() {
        tracing::error!("Token verification failed: {}", response.status());
        return Ok(NeonAuthLoginResponse {
            success: false,
            user: None,
            tenant: None,
            permissions: vec![],
            message: Some("Invalid token".to_string()),
        });
    }

    let session: NeonAuthSession = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let neon_user = session.user;
    tracing::info!("Neon Auth user verified: {}", neon_user.email);

    // Try to find local user by email
    let local_user = match db::User::find_by_email(&neon_user.email).await {
        Ok(user) => {
            // Update last login
            user.update_last_login().await.ok();
            user
        }
        Err(_) => {
            // User doesn't exist in SwiftPOS
            tracing::warn!("Neon Auth user {} not found in SwiftPOS", neon_user.email);
            return Ok(NeonAuthLoginResponse {
                success: false,
                user: None,
                tenant: None,
                permissions: vec![],
                message: Some(
                    format!(
                        "User {} is not provisioned in SwiftPOS. Please contact administrator.",
                        neon_user.email
                    )
                ),
            });
        }
    };

    // Check if user is active
    if !local_user.is_active {
        tracing::error!("User account is inactive: {}", neon_user.email);
        return Ok(NeonAuthLoginResponse {
            success: false,
            user: None,
            tenant: None,
            permissions: vec![],
            message: Some("Account is inactive".to_string()),
        });
    }

    // Get tenant info if available
    let tenant = if let Some(tenant_id) = local_user.tenant_id {
        match db::Tenant::find_by_id(tenant_id).await {
            Ok(t) => Some(TenantResponse {
                id: t.id.to_string(),
                name: t.name.clone(),
                slug: t.slug.clone(),
                application_name: t.application_name,
                currency_symbol: t.currency_symbol,
            }),
            Err(_) => None,
        }
    } else {
        None
    };

    // Get role permissions
    let permissions = crate::middleware::get_role_permissions(&local_user.role);

    tracing::info!("Login successful for user: {} with role: {}", neon_user.email, local_user.role);

    Ok(NeonAuthLoginResponse {
        success: true,
        user: Some(UserResponse {
            id: local_user.id.to_string(),
            email: local_user.email.clone(),
            full_name: local_user.full_name.clone(),
            role: local_user.role.clone(),
            tenant_id: local_user.tenant_id.map(|id| id.to_string()),
            branch_id: local_user.branch_id.map(|id| id.to_string()),
        }),
        tenant,
        permissions,
        message: None,
    })
}

#[tauri::command]
pub async fn register_tenant(
    request: RegisterTenantRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<LoginResponse, String> {
    tracing::info!("Registering new tenant: {}", request.name);
    
    // Get JWT secret from state
    let app_state = state.read().await;
    let jwt_secret = app_state.jwt_secret.clone();
    drop(app_state);
    
    // Validate password is not empty
    if request.password.len() < 8 {
        return Err("Password must be at least 8 characters".to_string());
    }
    
    // Create tenant
    let tenant = db::Tenant::create(&request.name, &request.slug, &request.email)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create tenant: {}", e);
            "Failed to create tenant".to_string()
        })?;
    
    // Hash password using argon2
    use argon2::{Argon2, password_hash::PasswordHasher};
    let salt = argon2::password_hash::SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(request.password.as_bytes(), &salt)
        .map_err(|e| {
            tracing::error!("Failed to hash password: {}", e);
            "Failed to hash password".to_string()
        })?
        .to_string();
    
    // Get database pool and create user
    let pool = db::get_db_pool().await
        .map_err(|e| format!("Database error: {}", e))?;
    
    let user_row = sqlx::query_as::<_, UserRow>(
        "INSERT INTO users (tenant_id, email, password_hash, full_name, role, is_active) 
         VALUES ($1, $2, $3, $4, 'owner', true) 
         RETURNING id, tenant_id, branch_id, email, full_name, role, is_active, last_login, created_at, updated_at"
    )
    .bind(tenant.id)
    .bind(&request.email)
    .bind(&password_hash)
    .bind(&request.full_name)
    .fetch_one(&*pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create user: {}", e);
        "Failed to create user".to_string()
    })?;
    
    let user = db::User {
        id: user_row.id,
        tenant_id: user_row.tenant_id,
        branch_id: user_row.branch_id,
        email: user_row.email,
        password_hash: user_row.password_hash,
        full_name: user_row.full_name,
        role: user_row.role,
        is_active: user_row.is_active,
        last_login: user_row.last_login,
        created_at: user_row.created_at,
        updated_at: user_row.updated_at,
    };
    
    // Create token
    let token = create_token(&user, &jwt_secret)
        .map_err(|e| e.to_string())?;
    
    tracing::info!("Tenant registered successfully: {}", request.name);
    
    Ok(LoginResponse {
        token,
        user: UserResponse {
            id: user.id.to_string(),
            email: user.email,
            full_name: user.full_name,
            role: user.role,
            tenant_id: user.tenant_id.map(|id| id.to_string()),
            branch_id: user.branch_id.map(|id| id.to_string()),
        },
        tenant: Some(TenantResponse {
            id: tenant.id.to_string(),
            name: tenant.name.clone(),
            slug: tenant.slug.clone(),
            application_name: tenant.application_name,
            currency_symbol: tenant.currency_symbol,
        }),
    })
}

#[derive(Debug, sqlx::FromRow)]
struct UserRow {
    id: uuid::Uuid,
    tenant_id: Option<uuid::Uuid>,
    branch_id: Option<uuid::Uuid>,
    email: String,
    password_hash: String,
    full_name: String,
    role: String,
    is_active: bool,
    last_login: Option<chrono::DateTime<chrono::Utc>>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

// Neon Auth token verification
#[derive(Debug, Serialize, Deserialize)]
pub struct NeonAuthUser {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub email_verified: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeonAuthSession {
    pub token: String,
    pub user: NeonAuthUser,
    pub expires_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyNeonTokenRequest {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyNeonTokenResponse {
    pub valid: bool,
    pub user: Option<NeonAuthUser>,
    pub error: Option<String>,
}

// Verify Neon Auth token by calling the Neon Auth API
#[tauri::command]
pub async fn verify_neon_token(
    state: tauri::State<'_, crate::AppState>,
    request: VerifyNeonTokenRequest,
) -> Result<VerifyNeonTokenResponse, String> {
    let neon_project_id = &state.neon_project_id;
    let stack_secret_key = &state.stack_secret_key;
    
    let auth_url = format!(
        "https://api.stack-auth.com/api/v1/projects/{}/auth/verify-token",
        neon_project_id
    );

    let client = reqwest::Client::new();
    
    match client
        .post(&auth_url)
        .header("Authorization", format!("Bearer {}", stack_secret_key))
        .json(&serde_json::json!({ "token": request.token }))
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<NeonAuthSession>().await {
                    Ok(session) => Ok(VerifyNeonTokenResponse {
                        valid: true,
                        user: Some(session.user),
                        error: None,
                    }),
                    Err(e) => Ok(VerifyNeonTokenResponse {
                        valid: false,
                        user: None,
                        error: Some(format!("Failed to parse session: {}", e)),
                    }),
                }
            } else {
                Ok(VerifyNeonTokenResponse {
                    valid: false,
                    user: None,
                    error: Some(format!("Token verification failed: {}", response.status())),
                })
            }
        }
        Err(e) => Err(format!("Network error: {}", e)),
    }
}

// Provision a new user in SwiftPOS (links to Stack Auth account)
// Only super_admin users can provision new users
#[tauri::command]
pub async fn provision_user(
    request: ProvisionUserRequest,
    token: String,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<ProvisionUserResponse, String> {
    tracing::info!("Provisioning user: {}", request.email);
    
    // Validate token and get auth context
    let app_state = state.read().await;
    let auth_context = middleware::validate_token(&token, &app_state).await?;
    
    // Check permission - only super_admin can provision users
    middleware::require_role(&auth_context, "super_admin")?;
    
    // Validate role - prevent creating super_admin by non-super_admin
    if request.role == "super_admin" && auth_context.role != "super_admin" {
        return Ok(ProvisionUserResponse {
            success: false,
            user: None,
            message: "Cannot create super_admin users".to_string(),
        });
    }
    
    // Non-super_admin can only create users within their tenant
    if auth_context.role != "super_admin" {
        if let Some(ref auth_tenant_id) = auth_context.tenant_id {
            if let Some(ref request_tenant_id) = request.tenant_id {
                if auth_tenant_id != request_tenant_id {
                    return Ok(ProvisionUserResponse {
                        success: false,
                        user: None,
                        message: "Cannot provision users for different tenant".to_string(),
                    });
                }
            }
        }
    }
    drop(app_state);
    
    // Validate tenant_id if provided
    let tenant_uuid = if let Some(ref tid) = request.tenant_id {
        match uuid::Uuid::parse_str(tid) {
            Ok(uuid) => Some(uuid),
            Err(_) => return Ok(ProvisionUserResponse {
                success: false,
                user: None,
                message: "Invalid tenant_id format".to_string(),
            }),
        }
    } else {
        None
    };
    
    // Validate branch_id if provided
    let branch_uuid = if let Some(ref bid) = request.branch_id {
        match uuid::Uuid::parse_str(bid) {
            Ok(uuid) => Some(uuid),
            Err(_) => return Ok(ProvisionUserResponse {
                success: false,
                user: None,
                message: "Invalid branch_id format".to_string(),
            }),
        }
    } else {
        None
    };
    
    // Check if user already exists
    if db::User::find_by_email(&request.email).await.is_ok() {
        return Ok(ProvisionUserResponse {
            success: false,
            user: None,
            message: format!("User {} already exists", request.email),
        });
    }
    
    // Get database pool
    let pool = db::get_db_pool().await
        .map_err(|e| format!("Database error: {}", e))?;
    
    // Create user with a placeholder hash (auth is via Stack Auth)
    // Use a cryptographically invalid hash that can never authenticate
    let password_placeholder = "$argon2id$v=19$m=65536,t=3,p=4$NO_PASSWORD_HASH_FOR_STACK_AUTH_USER";
    let user_row = sqlx::query_as::<_, UserRow>(
        "INSERT INTO users (tenant_id, branch_id, email, password_hash, full_name, role, is_active) 
         VALUES ($1, $2, $3, $4, $5, $6, true) 
         RETURNING id, tenant_id, branch_id, email, full_name, role, is_active, last_login, created_at, updated_at"
    )
    .bind(tenant_uuid)
    .bind(branch_uuid)
    .bind(&request.email)
    .bind(password_placeholder)  // Empty password - auth is via Stack Auth
    .bind(&request.full_name)
    .bind(&request.role)
    .fetch_one(&*pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create user: {}", e);
        format!("Failed to create user: {}", e)
    })?;
    
    let user = db::User {
        id: user_row.id,
        tenant_id: user_row.tenant_id,
        branch_id: user_row.branch_id,
        email: user_row.email,
        password_hash: user_row.password_hash,
        full_name: user_row.full_name,
        role: user_row.role,
        is_active: user_row.is_active,
        last_login: user_row.last_login,
        created_at: user_row.created_at,
        updated_at: user_row.updated_at,
    };
    
    tracing::info!("User provisioned successfully: {}", request.email);
    
    Ok(ProvisionUserResponse {
        success: true,
        user: Some(UserResponse {
            id: user.id.to_string(),
            email: user.email,
            full_name: user.full_name,
            role: user.role,
            tenant_id: user.tenant_id.map(|id| id.to_string()),
            branch_id: user.branch_id.map(|id| id.to_string()),
        }),
        message: "User provisioned successfully".to_string(),
    })
}
