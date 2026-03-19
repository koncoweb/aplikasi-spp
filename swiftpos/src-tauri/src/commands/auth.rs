// ============================================================
// SwiftPOS Auth — Option A: Email + Password (argon2 + JWT)
// No external auth provider. Works offline, all platforms.
// ============================================================
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Header, EncodingKey, decode, DecodingKey, Validation};
use chrono::{Utc, Duration};
use crate::db::{self, DbError};
use crate::middleware;
use crate::AppState;
use tauri::State;
use std::sync::Arc;
use tokio::sync::RwLock;

// ── Helpers ───────────────────────────────────────────────────────

/// Konversi nama bisnis menjadi slug DB-safe (huruf kecil, angka, dash).
/// Contoh: "Toko Budi & Rekan!" → "toko-budi-rekan"
fn slugify(s: &str) -> String {
    let mut slug = String::new();
    let mut prev_dash = true; // hindari leading dash
    for c in s.chars() {
        if c.is_alphanumeric() {
            slug.push(c.to_ascii_lowercase());
            prev_dash = false;
        } else if !prev_dash {
            slug.push('-');
            prev_dash = true;
        }
    }
    // Hapus trailing dash
    slug.trim_end_matches('-').to_string()
}


// ── Request / Response types ──────────────────────────────────────

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
    pub name: String,           // Nama bisnis, e.g. "Toko Budi"
    pub slug: Option<String>,   // Auto-generated jika tidak diisi
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub phone: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePasswordRequest {
    pub token: String,
    pub current_password: String,
    pub new_password: String,
}

// ── JWT Claims ────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,         // user_id
    pub email: String,
    pub role: String,
    pub tenant_id: Option<String>,
    pub branch_id: Option<String>,
    pub exp: i64,
}

// AppState is defined in crate root (lib.rs) and imported above

// ── Helpers ───────────────────────────────────────────────────────

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

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    ).map_err(|e| DbError::Validation(e.to_string()))
}

pub fn decode_token(token: &str, jwt_secret: &str) -> Result<Claims, String> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    ).map_err(|e| format!("Invalid token: {}", e))?;

    Ok(token_data.claims)
}

fn user_to_response(user: &db::User) -> UserResponse {
    UserResponse {
        id: user.id.to_string(),
        email: user.email.clone(),
        full_name: user.full_name.clone(),
        role: user.role.clone(),
        tenant_id: user.tenant_id.map(|id| id.to_string()),
        branch_id: user.branch_id.map(|id| id.to_string()),
    }
}

async fn load_tenant(tenant_id: uuid::Uuid) -> Option<TenantResponse> {
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
}

// ── Commands ──────────────────────────────────────────────────────

/// Login dengan email + password. Berlaku untuk semua platform (desktop, offline).
#[tauri::command]
pub async fn login(
    request: LoginRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<LoginResponse, String> {
    tracing::info!("Login attempt: {}", request.email);

    let jwt_secret = state.read().await.jwt_secret.clone();

    // 1. Cari user berdasarkan email
    let user = db::User::find_by_email(&request.email)
        .await
        .map_err(|_| "Email atau password salah".to_string())?;

    // 2. Cek aktif
    if !user.is_active {
        return Err("Akun tidak aktif. Hubungi administrator.".to_string());
    }

    // 3. Verifikasi password (argon2)
    if !user.verify_password(&request.password).await {
        tracing::warn!("Password salah untuk: {}", request.email);
        return Err("Email atau password salah".to_string());
    }

    // 4. Update last_login (fire-and-forget)
    user.update_last_login().await.ok();

    // 5. Load tenant jika ada
    let tenant = if let Some(tid) = user.tenant_id {
        load_tenant(tid).await
    } else {
        None
    };

    // 6. Buat JWT
    let token = create_token(&user, &jwt_secret)
        .map_err(|e| e.to_string())?;

    tracing::info!("Login sukses: {} ({})", user.email, user.role);

    Ok(LoginResponse {
        token,
        user: user_to_response(&user),
        tenant,
    })
}

/// Logout — hapus sesi di frontend (JWT stateless, cukup hapus token dari memory).
#[tauri::command]
pub async fn logout() -> Result<(), String> {
    tracing::info!("User logged out");
    Ok(())
}

/// Verify JWT token and return user info if valid.
#[tauri::command]
pub async fn verify_session(
    token: Option<String>,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<VerifySessionResponse, String> {
    // If no token provided, return invalid
    let token = match token {
        Some(t) => t,
        None => return Ok(VerifySessionResponse {
            valid: false,
            user: UserResponse {
                id: String::new(),
                email: String::new(),
                full_name: String::new(),
                role: String::new(),
                tenant_id: None,
                branch_id: None,
            },
            tenant: None,
        }),
    };
    
    let jwt_secret = state.read().await.jwt_secret.clone();
    
    let claims = decode_token(&token, &jwt_secret)?;
    
    // Load user from database to ensure they still exist and are active
    let user = db::User::find_by_id_string(&claims.sub)
        .await
        .map_err(|_| "User tidak ditemukan".to_string())?;
    
    if !user.is_active {
        return Err("Akun tidak aktif. Hubungi administrator.".to_string());
    }
    
    // Load tenant if available
    let tenant = if let Some(tid) = user.tenant_id {
        load_tenant(tid).await
    } else {
        None
    };
    
    Ok(VerifySessionResponse {
        valid: true,
        user: user_to_response(&user),
        tenant,
    })
}

#[derive(Debug, Serialize)]
pub struct VerifySessionResponse {
    pub valid: bool,
    pub user: UserResponse,
    pub tenant: Option<TenantResponse>,
}

/// Daftar tenant baru + buat akun owner pertama.
#[tauri::command]
pub async fn register_tenant(
    request: RegisterTenantRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<LoginResponse, String> {
    tracing::info!("Mendaftar tenant baru: {}", request.name);

    let jwt_secret = state.read().await.jwt_secret.clone();

    if request.password.len() < 8 {
        return Err("Password minimal 8 karakter".to_string());
    }

    // Cek email sudah terdaftar
    if db::User::find_by_email(&request.email).await.is_ok() {
        return Err(format!("Email '{}' sudah terdaftar. Silakan gunakan email lain atau login.", request.email));
    }

    // Buat slug dari nama bisnis (auto-generate), bersihkan karakter khusus
    let base_slug = request.slug.as_deref().unwrap_or(&request.name);
    let slug = slugify(base_slug);
    
    // Cek apakah slug sudah dipakai; jika ya, tambahkan suffix acak
    let final_slug = if db::Tenant::find_by_slug(&slug).await.is_ok() {
        format!("{}-{}", slug, &uuid::Uuid::new_v4().to_string()[..8])
    } else {
        slug
    };

    // Buat tenant
    let tenant = db::Tenant::create(
        &request.name,
        &final_slug,
        &request.email,
        request.phone.as_deref(),
        request.address.as_deref(),
    )
        .await
        .map_err(|e| {
            tracing::error!("Gagal buat tenant: {}", e);
            "Gagal membuat tenant. Slug mungkin sudah dipakai.".to_string()
        })?;

    // Hash password
    use argon2::{Argon2, password_hash::PasswordHasher};
    let salt = argon2::password_hash::SaltString::generate(&mut rand::thread_rng());
    let password_hash = Argon2::default()
        .hash_password(request.password.as_bytes(), &salt)
        .map_err(|e| format!("Gagal hash password: {}", e))?
        .to_string();

    // Buat user owner
    let pool = db::get_db_pool().await
        .map_err(|e| format!("Database error: {}", e))?;

    let user_row = sqlx::query_as::<_, UserRow>(
        "INSERT INTO users (tenant_id, email, password_hash, full_name, role, is_active)
         VALUES ($1, $2, $3, $4, 'owner', true)
         RETURNING id, tenant_id, branch_id, email, password_hash, full_name, role, is_active, last_login, created_at, updated_at"
    )
    .bind(tenant.id)
    .bind(&request.email)
    .bind(&password_hash)
    .bind(&request.full_name)
    .fetch_one(&*pool)
    .await
    .map_err(|e| {
        tracing::error!("Gagal buat user: {}", e);
        "Gagal membuat akun pengguna".to_string()
    })?;

    let user = row_to_user(user_row);
    let token = create_token(&user, &jwt_secret).map_err(|e| e.to_string())?;

    tracing::info!("Tenant terdaftar: {}", request.name);

    Ok(LoginResponse {
        token,
        user: user_to_response(&user),
        tenant: Some(TenantResponse {
            id: tenant.id.to_string(),
            name: tenant.name,
            slug: tenant.slug,
            application_name: tenant.application_name,
            currency_symbol: tenant.currency_symbol,
        }),
    })
}

/// Ganti password (memerlukan password lama).
#[tauri::command]
pub async fn change_password(
    request: ChangePasswordRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<(), String> {
    let jwt_secret = state.read().await.jwt_secret.clone();

    // Validasi token
    let claims = decode_token(&request.token, &jwt_secret)
        .map_err(|e| e.to_string())?;

    // Ambil user
    let user = db::User::find_by_email(&claims.email)
        .await
        .map_err(|_| "User tidak ditemukan".to_string())?;

    // Verifikasi password lama
    if !user.verify_password(&request.current_password).await {
        return Err("Password lama salah".to_string());
    }

    if request.new_password.len() < 8 {
        return Err("Password baru minimal 8 karakter".to_string());
    }

    // Hash password baru
    use argon2::{Argon2, password_hash::PasswordHasher};
    let salt = argon2::password_hash::SaltString::generate(&mut rand::thread_rng());
    let new_hash = Argon2::default()
        .hash_password(request.new_password.as_bytes(), &salt)
        .map_err(|e| format!("Gagal hash password: {}", e))?
        .to_string();

    // Update di DB
    let pool = db::get_db_pool().await
        .map_err(|e| format!("Database error: {}", e))?;

    sqlx::query("UPDATE users SET password_hash = $1, updated_at = NOW() WHERE id = $2")
        .bind(&new_hash)
        .bind(user.id)
        .execute(&*pool)
        .await
        .map_err(|e| format!("Gagal update password: {}", e))?;

    tracing::info!("Password diubah untuk: {}", user.email);
    Ok(())
}


// ── Internal helpers ──────────────────────────────────────────────

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

fn row_to_user(r: UserRow) -> db::User {
    db::User {
        id: r.id,
        tenant_id: r.tenant_id,
        branch_id: r.branch_id,
        email: r.email,
        password_hash: r.password_hash,
        full_name: r.full_name,
        role: r.role,
        is_active: r.is_active,
        last_login: r.last_login,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }
}
