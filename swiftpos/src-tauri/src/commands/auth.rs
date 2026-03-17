use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Header, EncodingKey, decode, DecodingKey, Validation};
use chrono::{Utc, Duration};
use crate::db::{self, DbError};
use tauri::State;
use std::sync::Arc;
use tokio::sync::RwLock;

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
