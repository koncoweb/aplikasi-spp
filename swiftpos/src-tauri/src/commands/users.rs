use serde::{Deserialize, Serialize};
use sqlx::Row;
use crate::db;
use crate::middleware;
use tauri::State;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::AppState;
use crate::commands::auth::UserResponse;

#[derive(Debug, Deserialize)]
pub struct GetUsersRequest {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub token: String,
    pub email: String,
    pub full_name: String,
    pub password: String,
    pub role: String,
    pub branch_id: Option<String>,
}

#[tauri::command]
pub async fn get_users(
    request: GetUsersRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<Vec<UserResponse>, String> {
    let app_state = state.read().await;
    let auth = middleware::validate_token(&request.token, &app_state).await?;
    drop(app_state);

    // Only owner/manager/super_admin can list users
    if !["owner", "manager", "super_admin"].contains(&auth.role.as_str()) {
        return Err("Tidak punya izin untuk melihat daftar pengguna".to_string());
    }

    let tenant_id = auth.tenant_id
        .ok_or_else(|| "Tenant ID tidak ditemukan dalam token".to_string())?;
    
    let tenant_uuid = uuid::Uuid::parse_str(&tenant_id)
        .map_err(|e| format!("Invalid tenant ID: {}", e))?;

    let users = db::User::find_by_tenant(tenant_uuid)
        .await
        .map_err(|e| format!("Gagal mengambil daftar pengguna: {}", e))?;

    Ok(users.into_iter().map(|u| UserResponse {
        id: u.id.to_string(),
        email: u.email,
        full_name: u.full_name,
        role: u.role,
        tenant_id: u.tenant_id.map(|id| id.to_string()),
        branch_id: u.branch_id.map(|id| id.to_string()),
    }).collect())
}

#[tauri::command]
pub async fn create_user(
    request: CreateUserRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<UserResponse, String> {
    let app_state_guard = state.read().await;

    // Validasi token
    let auth = middleware::validate_token(&request.token, &*app_state_guard).await?;
    drop(app_state_guard);

    // Hanya owner/manager bisa buat user
    if !["owner", "manager", "super_admin"].contains(&auth.role.as_str()) {
        return Err("Tidak punya izin untuk membuat pengguna".to_string());
    }

    if request.password.len() < 8 {
        return Err("Password minimal 8 karakter".to_string());
    }

    // Cek role yang boleh dibuat
    let allowed_roles = match auth.role.as_str() {
        "super_admin" => vec!["super_admin", "owner", "manager", "kasir"],
        "owner"       => vec!["manager", "kasir"],
        "manager"     => vec!["kasir"],
        _             => vec![],
    };
    if !allowed_roles.contains(&request.role.as_str()) {
        return Err(format!("Tidak bisa membuat role '{}'", request.role));
    }

    // Cek user sudah ada
    if db::User::find_by_email(&request.email).await.is_ok() {
        return Err(format!("Email '{}' sudah terdaftar", request.email));
    }

    // Hash password
    use argon2::{Argon2, password_hash::PasswordHasher};
    let salt = argon2::password_hash::SaltString::generate(&mut rand::thread_rng());
    let password_hash = Argon2::default()
        .hash_password(request.password.as_bytes(), &salt)
        .map_err(|e| format!("Gagal hash password: {}", e))?
        .to_string();

    let pool = db::get_db_pool().await
        .map_err(|e| format!("Database error: {}", e))?;

    let tenant_id = auth.tenant_id
        .as_deref()
        .and_then(|s| uuid::Uuid::parse_str(s).ok());

    let branch_uuid = request.branch_id
        .as_deref()
        .and_then(|s| uuid::Uuid::parse_str(s).ok());

    let row = sqlx::query(
        "INSERT INTO users (tenant_id, branch_id, email, password_hash, full_name, role, is_active)
         VALUES ($1, $2, $3, $4, $5, $6, true)
         RETURNING id, tenant_id, branch_id, email, full_name, role, is_active"
    )
    .bind(tenant_id)
    .bind(branch_uuid)
    .bind(&request.email)
    .bind(&password_hash)
    .bind(&request.full_name)
    .bind(&request.role)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Gagal buat user: {}", e))?;

    tracing::info!("User '{}' dibuat oleh '{}'", request.email, auth.email);
    
    Ok(UserResponse {
        id: row.get::<uuid::Uuid, _>("id").to_string(),
        email: row.get("email"),
        full_name: row.get("full_name"),
        role: row.get("role"),
        tenant_id: row.get::<Option<uuid::Uuid>, _>("tenant_id").map(|id| id.to_string()),
        branch_id: row.get::<Option<uuid::Uuid>, _>("branch_id").map(|id| id.to_string()),
    })
}
