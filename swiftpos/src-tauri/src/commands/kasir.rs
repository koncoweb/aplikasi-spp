use serde::{Deserialize, Serialize};
use crate::db;
use crate::middleware;
use tauri::State;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::AppState;

// ─── Structs ─────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct KasirShiftRow {
    pub id: uuid::Uuid,
    pub tenant_id: uuid::Uuid,
    pub branch_id: uuid::Uuid,
    pub user_id: Option<uuid::Uuid>,
    pub modal_awal: f64,
    pub modal_aktual: Option<f64>,
    pub total_penjualan: Option<f64>,
    pub total_kas_masuk: Option<f64>,
    pub total_pengeluaran: Option<f64>,
    pub status: String,
    pub catatan: Option<String>,
    pub opened_at: chrono::DateTime<chrono::Utc>,
    pub closed_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KasirShiftResponse {
    pub id: String,
    pub tenant_id: String,
    pub branch_id: String,
    pub user_id: Option<String>,
    pub modal_awal: f64,
    pub modal_aktual: Option<f64>,
    pub total_penjualan: f64,
    pub total_kas_masuk: f64,
    pub total_pengeluaran: f64,
    pub status: String,
    pub catatan: Option<String>,
    pub opened_at: String,
    pub closed_at: Option<String>,
}

impl From<KasirShiftRow> for KasirShiftResponse {
    fn from(r: KasirShiftRow) -> Self {
        KasirShiftResponse {
            id: r.id.to_string(),
            tenant_id: r.tenant_id.to_string(),
            branch_id: r.branch_id.to_string(),
            user_id: r.user_id.map(|u| u.to_string()),
            modal_awal: r.modal_awal,
            modal_aktual: r.modal_aktual,
            total_penjualan: r.total_penjualan.unwrap_or(0.0),
            total_kas_masuk: r.total_kas_masuk.unwrap_or(0.0),
            total_pengeluaran: r.total_pengeluaran.unwrap_or(0.0),
            status: r.status,
            catatan: r.catatan,
            opened_at: r.opened_at.to_rfc3339(),
            closed_at: r.closed_at.map(|d| d.to_rfc3339()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct KasMasukRow {
    pub id: uuid::Uuid,
    pub tenant_id: uuid::Uuid,
    pub branch_id: uuid::Uuid,
    pub shift_id: Option<uuid::Uuid>,
    pub deskripsi: String,
    pub jumlah: f64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KasMasukResponse {
    pub id: String,
    pub shift_id: Option<String>,
    pub deskripsi: String,
    pub jumlah: f64,
    pub created_at: String,
}

impl From<KasMasukRow> for KasMasukResponse {
    fn from(r: KasMasukRow) -> Self {
        KasMasukResponse {
            id: r.id.to_string(),
            shift_id: r.shift_id.map(|s| s.to_string()),
            deskripsi: r.deskripsi,
            jumlah: r.jumlah,
            created_at: r.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PengeluaranRow {
    pub id: uuid::Uuid,
    pub tenant_id: uuid::Uuid,
    pub branch_id: uuid::Uuid,
    pub shift_id: Option<uuid::Uuid>,
    pub deskripsi: String,
    pub jumlah: f64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PengeluaranResponse {
    pub id: String,
    pub shift_id: Option<String>,
    pub deskripsi: String,
    pub jumlah: f64,
    pub created_at: String,
}

impl From<PengeluaranRow> for PengeluaranResponse {
    fn from(r: PengeluaranRow) -> Self {
        PengeluaranResponse {
            id: r.id.to_string(),
            shift_id: r.shift_id.map(|s| s.to_string()),
            deskripsi: r.deskripsi,
            jumlah: r.jumlah,
            created_at: r.created_at.to_rfc3339(),
        }
    }
}

// ─── Requests ─────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct BukaKasirRequest {
    pub token: String,
    pub tenant_id: String,
    pub branch_id: String,
    pub user_id: String,
    pub modal_awal: f64,
}

#[derive(Debug, Deserialize)]
pub struct TutupKasirRequest {
    pub token: String,
    pub shift_id: String,
    pub modal_aktual: f64,
    pub catatan: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetKasirShiftRequest {
    pub token: String,
    pub branch_id: String,
}

#[derive(Debug, Deserialize)]
pub struct TambahKasMasukRequest {
    pub token: String,
    pub tenant_id: String,
    pub branch_id: String,
    pub shift_id: Option<String>,
    pub deskripsi: String,
    pub jumlah: f64,
    pub user_id: String,
}

#[derive(Debug, Deserialize)]
pub struct GetKasMasukRequest {
    pub token: String,
    pub shift_id: String,
}

#[derive(Debug, Deserialize)]
pub struct TambahPengeluaranRequest {
    pub token: String,
    pub tenant_id: String,
    pub branch_id: String,
    pub shift_id: Option<String>,
    pub deskripsi: String,
    pub jumlah: f64,
    pub user_id: String,
}

#[derive(Debug, Deserialize)]
pub struct GetPengeluaranRequest {
    pub token: String,
    pub shift_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LaporanTutupKasirResponse {
    pub shift: KasirShiftResponse,
    pub kas_masuk_list: Vec<KasMasukResponse>,
    pub pengeluaran_list: Vec<PengeluaranResponse>,
    pub total_kas_masuk: f64,
    pub total_pengeluaran: f64,
    pub kas_seharusnya: f64,
    pub selisih: f64,
}

// ─── Commands ─────────────────────────────────────────────────────────────────

/// Buka Kasir — creates a new shift
#[tauri::command]
pub async fn buka_kasir(
    request: BukaKasirRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<KasirShiftResponse, String> {
    tracing::info!("Buka kasir for branch: {}", request.branch_id);

    let app_state = state.read().await;
    let _auth_context = middleware::validate_token(&request.token, &app_state).await?;
    drop(app_state);

    let tenant_uuid = uuid::Uuid::parse_str(&request.tenant_id)
        .map_err(|e| format!("Invalid tenant ID: {}", e))?;
    let branch_uuid = uuid::Uuid::parse_str(&request.branch_id)
        .map_err(|e| format!("Invalid branch ID: {}", e))?;
    let user_uuid = uuid::Uuid::parse_str(&request.user_id)
        .map_err(|e| format!("Invalid user ID: {}", e))?;

    let pool = db::get_db_pool().await
        .map_err(|e| format!("Database error: {}", e))?;

    // Check if there's already an open shift for this branch
    let existing = sqlx::query_scalar::<sqlx::Postgres, i64>(
        "SELECT COUNT(*) FROM kasir_shifts WHERE branch_id = $1 AND status = 'open'"
    )
    .bind(branch_uuid)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("DB error: {}", e))?;

    if existing > 0 {
        return Err("Kasir sudah terbuka. Tutup shift sebelumnya terlebih dahulu.".to_string());
    }

    let row = sqlx::query_as::<sqlx::Postgres, KasirShiftRow>(
        "INSERT INTO kasir_shifts (tenant_id, branch_id, user_id, modal_awal, status)
         VALUES ($1, $2, $3, $4, 'open')
         RETURNING id, tenant_id, branch_id, user_id, modal_awal, modal_aktual,
         total_penjualan, total_kas_masuk, total_pengeluaran, status, catatan,
         opened_at, closed_at"
    )
    .bind(tenant_uuid)
    .bind(branch_uuid)
    .bind(user_uuid)
    .bind(request.modal_awal)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Failed to buka kasir: {}", e))?;

    tracing::info!("Kasir opened, shift ID: {}", row.id);
    Ok(row.into())
}

/// Tutup Kasir — closes an active shift
#[tauri::command]
pub async fn tutup_kasir(
    request: TutupKasirRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<KasirShiftResponse, String> {
    tracing::info!("Tutup kasir for shift: {}", request.shift_id);

    let app_state = state.read().await;
    let _auth_context = middleware::validate_token(&request.token, &app_state).await?;
    drop(app_state);

    let shift_uuid = uuid::Uuid::parse_str(&request.shift_id)
        .map_err(|e| format!("Invalid shift ID: {}", e))?;

    let pool = db::get_db_pool().await
        .map_err(|e| format!("Database error: {}", e))?;

    let row = sqlx::query_as::<sqlx::Postgres, KasirShiftRow>(
        "UPDATE kasir_shifts
         SET status = 'closed', modal_aktual = $2, catatan = $3, closed_at = NOW()
         WHERE id = $1 AND status = 'open'
         RETURNING id, tenant_id, branch_id, user_id, modal_awal, modal_aktual,
         total_penjualan, total_kas_masuk, total_pengeluaran, status, catatan,
         opened_at, closed_at"
    )
    .bind(shift_uuid)
    .bind(request.modal_aktual)
    .bind(&request.catatan)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Failed to tutup kasir: {}", e))?;

    tracing::info!("Kasir closed, shift ID: {}", row.id);
    Ok(row.into())
}

/// Get active kasir shift for a branch
#[tauri::command]
pub async fn get_kasir_shift(
    request: GetKasirShiftRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<Option<KasirShiftResponse>, String> {
    let app_state = state.read().await;
    let _auth_context = middleware::validate_token(&request.token, &app_state).await?;
    drop(app_state);

    let branch_uuid = uuid::Uuid::parse_str(&request.branch_id)
        .map_err(|e| format!("Invalid branch ID: {}", e))?;

    let pool = db::get_db_pool().await
        .map_err(|e| format!("Database error: {}", e))?;

    let row = sqlx::query_as::<sqlx::Postgres, KasirShiftRow>(
        "SELECT id, tenant_id, branch_id, user_id, modal_awal, modal_aktual,
         total_penjualan, total_kas_masuk, total_pengeluaran, status, catatan,
         opened_at, closed_at
         FROM kasir_shifts WHERE branch_id = $1 AND status = 'open'
         ORDER BY opened_at DESC LIMIT 1"
    )
    .bind(branch_uuid)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Failed to get shift: {}", e))?;

    Ok(row.map(|r| r.into()))
}

/// Tambah Kas Masuk
#[tauri::command]
pub async fn tambah_kas_masuk(
    request: TambahKasMasukRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<KasMasukResponse, String> {
    tracing::info!("Tambah kas masuk: {}", request.deskripsi);

    let app_state = state.read().await;
    let _auth_context = middleware::validate_token(&request.token, &app_state).await?;
    drop(app_state);

    let tenant_uuid = uuid::Uuid::parse_str(&request.tenant_id)
        .map_err(|e| format!("Invalid tenant ID: {}", e))?;
    let branch_uuid = uuid::Uuid::parse_str(&request.branch_id)
        .map_err(|e| format!("Invalid branch ID: {}", e))?;
    let shift_uuid = request.shift_id.as_deref()
        .map(|s| uuid::Uuid::parse_str(s).map_err(|e| format!("Invalid shift ID: {}", e)))
        .transpose()?;
    let user_uuid = uuid::Uuid::parse_str(&request.user_id)
        .map_err(|e| format!("Invalid user ID: {}", e))?;

    let pool = db::get_db_pool().await
        .map_err(|e| format!("Database error: {}", e))?;

    let row = sqlx::query_as::<sqlx::Postgres, KasMasukRow>(
        "INSERT INTO kas_masuk (tenant_id, branch_id, shift_id, deskripsi, jumlah, created_by)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING id, tenant_id, branch_id, shift_id, deskripsi, jumlah, created_at"
    )
    .bind(tenant_uuid)
    .bind(branch_uuid)
    .bind(shift_uuid)
    .bind(&request.deskripsi)
    .bind(request.jumlah)
    .bind(user_uuid)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Failed to tambah kas masuk: {}", e))?;

    // Update shift total_kas_masuk
    if let Some(sid) = shift_uuid {
        let _ = sqlx::query(
            "UPDATE kasir_shifts SET total_kas_masuk = COALESCE(total_kas_masuk, 0) + $1 WHERE id = $2"
        )
        .bind(request.jumlah)
        .bind(sid)
        .execute(&*pool)
        .await;
    }

    Ok(row.into())
}

/// Get list of kas masuk for a shift
#[tauri::command]
pub async fn get_kas_masuk(
    request: GetKasMasukRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<Vec<KasMasukResponse>, String> {
    let app_state = state.read().await;
    let _auth_context = middleware::validate_token(&request.token, &app_state).await?;
    drop(app_state);

    let shift_uuid = uuid::Uuid::parse_str(&request.shift_id)
        .map_err(|e| format!("Invalid shift ID: {}", e))?;

    let pool = db::get_db_pool().await
        .map_err(|e| format!("Database error: {}", e))?;

    let rows = sqlx::query_as::<sqlx::Postgres, KasMasukRow>(
        "SELECT id, tenant_id, branch_id, shift_id, deskripsi, jumlah, created_at
         FROM kas_masuk WHERE shift_id = $1 ORDER BY created_at DESC"
    )
    .bind(shift_uuid)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Failed to get kas masuk: {}", e))?;

    Ok(rows.into_iter().map(|r| r.into()).collect())
}

/// Tambah Pengeluaran
#[tauri::command]
pub async fn tambah_pengeluaran(
    request: TambahPengeluaranRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<PengeluaranResponse, String> {
    tracing::info!("Tambah pengeluaran: {}", request.deskripsi);

    let app_state = state.read().await;
    let _auth_context = middleware::validate_token(&request.token, &app_state).await?;
    drop(app_state);

    let tenant_uuid = uuid::Uuid::parse_str(&request.tenant_id)
        .map_err(|e| format!("Invalid tenant ID: {}", e))?;
    let branch_uuid = uuid::Uuid::parse_str(&request.branch_id)
        .map_err(|e| format!("Invalid branch ID: {}", e))?;
    let shift_uuid = request.shift_id.as_deref()
        .map(|s| uuid::Uuid::parse_str(s).map_err(|e| format!("Invalid shift ID: {}", e)))
        .transpose()?;
    let user_uuid = uuid::Uuid::parse_str(&request.user_id)
        .map_err(|e| format!("Invalid user ID: {}", e))?;

    let pool = db::get_db_pool().await
        .map_err(|e| format!("Database error: {}", e))?;

    let row = sqlx::query_as::<sqlx::Postgres, PengeluaranRow>(
        "INSERT INTO pengeluaran (tenant_id, branch_id, shift_id, deskripsi, jumlah, created_by)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING id, tenant_id, branch_id, shift_id, deskripsi, jumlah, created_at"
    )
    .bind(tenant_uuid)
    .bind(branch_uuid)
    .bind(shift_uuid)
    .bind(&request.deskripsi)
    .bind(request.jumlah)
    .bind(user_uuid)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Failed to tambah pengeluaran: {}", e))?;

    // Update shift total_pengeluaran
    if let Some(sid) = shift_uuid {
        let _ = sqlx::query(
            "UPDATE kasir_shifts SET total_pengeluaran = COALESCE(total_pengeluaran, 0) + $1 WHERE id = $2"
        )
        .bind(request.jumlah)
        .bind(sid)
        .execute(&*pool)
        .await;
    }

    Ok(row.into())
}

/// Get pengeluaran list for a shift
#[tauri::command]
pub async fn get_pengeluaran(
    request: GetPengeluaranRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<Vec<PengeluaranResponse>, String> {
    let app_state = state.read().await;
    let _auth_context = middleware::validate_token(&request.token, &app_state).await?;
    drop(app_state);

    let shift_uuid = uuid::Uuid::parse_str(&request.shift_id)
        .map_err(|e| format!("Invalid shift ID: {}", e))?;

    let pool = db::get_db_pool().await
        .map_err(|e| format!("Database error: {}", e))?;

    let rows = sqlx::query_as::<sqlx::Postgres, PengeluaranRow>(
        "SELECT id, tenant_id, branch_id, shift_id, deskripsi, jumlah, created_at
         FROM pengeluaran WHERE shift_id = $1 ORDER BY created_at DESC"
    )
    .bind(shift_uuid)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Failed to get pengeluaran: {}", e))?;

    Ok(rows.into_iter().map(|r| r.into()).collect())
}

/// Get laporan tutup kasir — full closing summary for a shift
#[tauri::command]
pub async fn get_laporan_tutup_kasir(
    token: String,
    shift_id: String,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<LaporanTutupKasirResponse, String> {
    let app_state = state.read().await;
    let _auth_context = middleware::validate_token(&token, &app_state).await?;
    drop(app_state);

    let shift_uuid = uuid::Uuid::parse_str(&shift_id)
        .map_err(|e| format!("Invalid shift ID: {}", e))?;

    let pool = db::get_db_pool().await
        .map_err(|e| format!("Database error: {}", e))?;

    // Get shift
    let shift_row = sqlx::query_as::<sqlx::Postgres, KasirShiftRow>(
        "SELECT id, tenant_id, branch_id, user_id, modal_awal, modal_aktual,
         total_penjualan, total_kas_masuk, total_pengeluaran, status, catatan,
         opened_at, closed_at FROM kasir_shifts WHERE id = $1"
    )
    .bind(shift_uuid)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Shift not found: {}", e))?;

    // Get kas masuk
    let kas_masuk_rows = sqlx::query_as::<sqlx::Postgres, KasMasukRow>(
        "SELECT id, tenant_id, branch_id, shift_id, deskripsi, jumlah, created_at
         FROM kas_masuk WHERE shift_id = $1 ORDER BY created_at"
    )
    .bind(shift_uuid)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Failed to get kas masuk: {}", e))?;

    // Get pengeluaran
    let pengeluaran_rows = sqlx::query_as::<sqlx::Postgres, PengeluaranRow>(
        "SELECT id, tenant_id, branch_id, shift_id, deskripsi, jumlah, created_at
         FROM pengeluaran WHERE shift_id = $1 ORDER BY created_at"
    )
    .bind(shift_uuid)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Failed to get pengeluaran: {}", e))?;

    let total_kas_masuk: f64 = kas_masuk_rows.iter().map(|r| r.jumlah).sum();
    let total_pengeluaran: f64 = pengeluaran_rows.iter().map(|r| r.jumlah).sum();
    let total_penjualan = shift_row.total_penjualan.unwrap_or(0.0);

    // Kas seharusnya = modal awal + penjualan tunai + kas masuk - pengeluaran
    let kas_seharusnya = shift_row.modal_awal + total_penjualan + total_kas_masuk - total_pengeluaran;
    let modal_aktual = shift_row.modal_aktual.unwrap_or(0.0);
    let selisih = modal_aktual - kas_seharusnya;

    let shift_resp: KasirShiftResponse = shift_row.into();

    Ok(LaporanTutupKasirResponse {
        shift: shift_resp,
        kas_masuk_list: kas_masuk_rows.into_iter().map(|r| r.into()).collect(),
        pengeluaran_list: pengeluaran_rows.into_iter().map(|r| r.into()).collect(),
        total_kas_masuk,
        total_pengeluaran,
        kas_seharusnya,
        selisih,
    })
}
