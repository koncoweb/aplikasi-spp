use serde::{Deserialize, Serialize};
use crate::db;
use crate::middleware;
use tauri::State;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::AppState;

// ─── Structs ─────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PiutangRow {
    pub id: uuid::Uuid,
    pub tenant_id: uuid::Uuid,
    pub branch_id: uuid::Uuid,
    pub transaction_id: Option<uuid::Uuid>,
    pub shift_id: Option<uuid::Uuid>,
    pub customer_name: String,
    pub customer_phone: Option<String>,
    pub total_amount: f64,
    pub dp_amount: f64,
    pub sisa_piutang: f64,
    pub jatuh_tempo: Option<chrono::NaiveDate>,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PiutangResponse {
    pub id: String,
    pub tenant_id: String,
    pub branch_id: String,
    pub transaction_id: Option<String>,
    pub shift_id: Option<String>,
    pub customer_name: String,
    pub customer_phone: Option<String>,
    pub total_amount: f64,
    pub dp_amount: f64,
    pub sisa_piutang: f64,
    pub jatuh_tempo: Option<String>,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: String,
}

impl From<PiutangRow> for PiutangResponse {
    fn from(r: PiutangRow) -> Self {
        PiutangResponse {
            id: r.id.to_string(),
            tenant_id: r.tenant_id.to_string(),
            branch_id: r.branch_id.to_string(),
            transaction_id: r.transaction_id.map(|u| u.to_string()),
            shift_id: r.shift_id.map(|u| u.to_string()),
            customer_name: r.customer_name,
            customer_phone: r.customer_phone,
            total_amount: r.total_amount,
            dp_amount: r.dp_amount,
            sisa_piutang: r.sisa_piutang,
            jatuh_tempo: r.jatuh_tempo.map(|d| d.to_string()),
            status: r.status,
            notes: r.notes,
            created_at: r.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PiutangPaymentRow {
    pub id: uuid::Uuid,
    pub piutang_id: uuid::Uuid,
    pub shift_id: Option<uuid::Uuid>,
    pub jumlah_bayar: f64,
    pub metode_bayar: String,
    pub catatan: Option<String>,
    pub paid_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PiutangPaymentResponse {
    pub id: String,
    pub piutang_id: String,
    pub shift_id: Option<String>,
    pub jumlah_bayar: f64,
    pub metode_bayar: String,
    pub catatan: Option<String>,
    pub paid_at: String,
    pub sisa_piutang_baru: f64,
}

// ─── Requests ─────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreatePiutangRequest {
    pub token: String,
    pub tenant_id: String,
    pub branch_id: String,
    pub transaction_id: Option<String>,
    pub shift_id: Option<String>,
    pub customer_name: String,
    pub customer_phone: Option<String>,
    pub total_amount: f64,
    pub dp_amount: f64,
    pub jatuh_tempo: Option<String>,
    pub notes: Option<String>,
    pub user_id: String,
}

#[derive(Debug, Deserialize)]
pub struct GetPiutangListRequest {
    pub token: String,
    pub tenant_id: String,
    pub branch_id: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BayarPiutangRequest {
    pub token: String,
    pub piutang_id: String,
    pub shift_id: Option<String>,
    pub jumlah_bayar: f64,
    pub metode_bayar: String,
    pub catatan: Option<String>,
    pub user_id: String,
}

// ─── Commands ─────────────────────────────────────────────────────────────────

/// Create piutang record
#[tauri::command]
pub async fn create_piutang(
    request: CreatePiutangRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<PiutangResponse, String> {
    tracing::info!("Creating piutang for customer: {}", request.customer_name);

    let app_state = state.read().await;
    let _auth_context = middleware::validate_token(&request.token, &app_state).await?;
    drop(app_state);

    let tenant_uuid = uuid::Uuid::parse_str(&request.tenant_id)
        .map_err(|e| format!("Invalid tenant ID: {}", e))?;
    let branch_uuid = uuid::Uuid::parse_str(&request.branch_id)
        .map_err(|e| format!("Invalid branch ID: {}", e))?;
    let user_uuid = uuid::Uuid::parse_str(&request.user_id)
        .map_err(|e| format!("Invalid user ID: {}", e))?;
    let transaction_uuid = request.transaction_id.as_deref()
        .map(|s| uuid::Uuid::parse_str(s).map_err(|e| format!("Invalid transaction ID: {}", e)))
        .transpose()?;
    let shift_uuid = request.shift_id.as_deref()
        .map(|s| uuid::Uuid::parse_str(s).map_err(|e| format!("Invalid shift ID: {}", e)))
        .transpose()?;
    let jatuh_tempo = request.jatuh_tempo.as_deref()
        .map(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .map_err(|e| format!("Invalid jatuh_tempo: {}", e)))
        .transpose()?;

    let sisa_piutang = request.total_amount - request.dp_amount;

    if sisa_piutang < 0.0 {
        return Err("DP tidak boleh melebihi total piutang".to_string());
    }

    let pool = db::get_db_pool().await
        .map_err(|e| format!("Database error: {}", e))?;

    let row = sqlx::query_as::<_, PiutangRow>(
        "INSERT INTO piutang (tenant_id, branch_id, transaction_id, shift_id, customer_name,
         customer_phone, total_amount, dp_amount, sisa_piutang, jatuh_tempo, notes, created_by)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
         RETURNING id, tenant_id, branch_id, transaction_id, shift_id, customer_name,
         customer_phone, total_amount, dp_amount, sisa_piutang, jatuh_tempo, status,
         notes, created_at, updated_at"
    )
    .bind(tenant_uuid)
    .bind(branch_uuid)
    .bind(transaction_uuid)
    .bind(shift_uuid)
    .bind(&request.customer_name)
    .bind(&request.customer_phone)
    .bind(request.total_amount)
    .bind(request.dp_amount)
    .bind(sisa_piutang)
    .bind(jatuh_tempo)
    .bind(&request.notes)
    .bind(user_uuid)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Failed to create piutang: {}", e))?;

    tracing::info!("Piutang created: {}", row.id);
    Ok(row.into())
}

/// Get list of piutang
#[tauri::command]
pub async fn get_piutang_list(
    request: GetPiutangListRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<Vec<PiutangResponse>, String> {
    let app_state = state.read().await;
    let _auth_context = middleware::validate_token(&request.token, &app_state).await?;
    drop(app_state);

    let tenant_uuid = uuid::Uuid::parse_str(&request.tenant_id)
        .map_err(|e| format!("Invalid tenant ID: {}", e))?;

    let pool = db::get_db_pool().await
        .map_err(|e| format!("Database error: {}", e))?;

    let status_filter = request.status.as_deref().unwrap_or("aktif");

    let rows = sqlx::query_as::<_, PiutangRow>(
        "SELECT id, tenant_id, branch_id, transaction_id, shift_id, customer_name,
         customer_phone, total_amount, dp_amount, sisa_piutang, jatuh_tempo, status,
         notes, created_at, updated_at
         FROM piutang WHERE tenant_id = $1 AND status = $2
         ORDER BY created_at DESC LIMIT 200"
    )
    .bind(tenant_uuid)
    .bind(status_filter)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Failed to get piutang list: {}", e))?;

    Ok(rows.into_iter().map(|r| r.into()).collect())
}

/// Bayar piutang — record payment and update sisa_piutang
#[tauri::command]
pub async fn bayar_piutang(
    request: BayarPiutangRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<PiutangPaymentResponse, String> {
    tracing::info!("Bayar piutang: {}, jumlah: {}", request.piutang_id, request.jumlah_bayar);

    let app_state = state.read().await;
    let _auth_context = middleware::validate_token(&request.token, &app_state).await?;
    drop(app_state);

    let piutang_uuid = uuid::Uuid::parse_str(&request.piutang_id)
        .map_err(|e| format!("Invalid piutang ID: {}", e))?;
    let shift_uuid = request.shift_id.as_deref()
        .map(|s| uuid::Uuid::parse_str(s).map_err(|e| format!("Invalid shift ID: {}", e)))
        .transpose()?;
    let user_uuid = uuid::Uuid::parse_str(&request.user_id)
        .map_err(|e| format!("Invalid user ID: {}", e))?;

    let pool = db::get_db_pool().await
        .map_err(|e| format!("Database error: {}", e))?;

    // Start DB transaction
    let mut tx = pool.begin().await
        .map_err(|e| format!("Failed to start transaction: {}", e))?;

    // Get current piutang
    let current: (f64, String) = sqlx::query_as::<_, (f64, String)>(
        "SELECT sisa_piutang, status FROM piutang WHERE id = $1 FOR UPDATE"
    )
    .bind(piutang_uuid)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| format!("Piutang not found: {}", e))?;

    let (sisa_piutang, status) = current;

    if status == "lunas" {
        return Err("Piutang ini sudah lunas".to_string());
    }

    if request.jumlah_bayar > sisa_piutang {
        return Err(format!("Jumlah bayar melebihi sisa piutang (Rp {:.0})", sisa_piutang));
    }

    // Insert payment record
    let payment_row = sqlx::query_as::<_, PiutangPaymentRow>(
        "INSERT INTO piutang_payments (piutang_id, shift_id, jumlah_bayar, metode_bayar, catatan, paid_by)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING id, piutang_id, shift_id, jumlah_bayar, metode_bayar, catatan, paid_at"
    )
    .bind(piutang_uuid)
    .bind(shift_uuid)
    .bind(request.jumlah_bayar)
    .bind(&request.metode_bayar)
    .bind(&request.catatan)
    .bind(user_uuid)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| format!("Failed to record payment: {}", e))?;

    // Update sisa_piutang
    let sisa_baru = sisa_piutang - request.jumlah_bayar;
    let new_status = if sisa_baru <= 0.0 { "lunas" } else { "aktif" };

    sqlx::query(
        "UPDATE piutang SET sisa_piutang = $1, status = $2, updated_at = NOW() WHERE id = $3"
    )
    .bind(sisa_baru)
    .bind(new_status)
    .bind(piutang_uuid)
    .execute(&mut *tx)
    .await
    .map_err(|e| format!("Failed to update piutang: {}", e))?;

    tx.commit().await
        .map_err(|e| format!("Failed to commit: {}", e))?;

    tracing::info!("Piutang payment recorded. Sisa baru: {}", sisa_baru);

    Ok(PiutangPaymentResponse {
        id: payment_row.id.to_string(),
        piutang_id: payment_row.piutang_id.to_string(),
        shift_id: payment_row.shift_id.map(|u| u.to_string()),
        jumlah_bayar: payment_row.jumlah_bayar,
        metode_bayar: payment_row.metode_bayar,
        catatan: payment_row.catatan,
        paid_at: payment_row.paid_at.to_rfc3339(),
        sisa_piutang_baru: sisa_baru,
    })
}
