use serde::{Deserialize, Serialize};
use crate::db;
use chrono::{Utc, Datelike};

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub id: String,
    pub tenant_id: String,
    pub branch_id: String,
    pub transaction_number: String,
    pub user_id: Option<String>,
    pub subtotal: f64,
    pub tax_amount: f64,
    pub discount_amount: f64,
    pub total_amount: f64,
    pub payment_method: String,
    pub payment_reference: Option<String>,
    pub amount_paid: f64,
    pub change_amount: f64,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionItemResponse {
    pub id: String,
    pub transaction_id: String,
    pub product_id: Option<String>,
    pub product_name: String,
    pub product_sku: Option<String>,
    pub quantity: i32,
    pub unit_price: f64,
    pub discount_percent: f64,
    pub discount_amount: f64,
    pub subtotal: f64,
}

#[derive(Debug, Deserialize)]
pub struct CreateTransactionRequest {
    pub tenant_id: String,
    pub branch_id: String,
    pub user_id: String,
    pub items: Vec<CreateTransactionItemRequest>,
    pub subtotal: f64,
    pub tax_amount: f64,
    pub discount_amount: f64,
    pub total_amount: f64,
    pub payment_method: String,
    pub payment_reference: Option<String>,
    pub amount_paid: f64,
    pub change_amount: f64,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTransactionItemRequest {
    pub product_id: String,
    pub product_name: String,
    pub product_sku: Option<String>,
    pub quantity: i32,
    pub unit_price: f64,
    pub discount_percent: f64,
    pub discount_amount: f64,
    pub subtotal: f64,
}

fn generate_transaction_number() -> String {
    let now = Utc::now();
    format!("TRX{}{:02}{:02}{:06}", now.year(), now.month(), now.day(), rand::random::<u32>() % 1000000)
}

#[tauri::command]
pub async fn create_transaction(request: CreateTransactionRequest) -> Result<TransactionResponse, String> {
    tracing::info!("Creating transaction for tenant: {}", request.tenant_id);
    
    let tenant_uuid = uuid::Uuid::parse_str(&request.tenant_id)
        .map_err(|e| format!("Invalid tenant ID: {}", e))?;
    
    let branch_uuid = uuid::Uuid::parse_str(&request.branch_id)
        .map_err(|e| format!("Invalid branch ID: {}", e))?;
    
    let user_uuid = uuid::Uuid::parse_str(&request.user_id)
        .map_err(|e| format!("Invalid user ID: {}", e))?;
    
    let transaction_number = generate_transaction_number();
    
    let pool = db::get_db_pool().await
        .map_err(|e| format!("Database error: {}", e))?;
    
    // Start transaction
    let mut tx = pool.begin().await
        .map_err(|e| format!("Failed to start transaction: {}", e))?;
    
    // Create transaction
    let row = sqlx::query_as::<_, TransactionRow>(
        "INSERT INTO transactions (tenant_id, branch_id, user_id, transaction_number, 
         subtotal, tax_amount, discount_amount, total_amount, payment_method, 
         payment_reference, amount_paid, change_amount, status, notes) 
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, 'completed', $13) 
         RETURNING id, tenant_id, branch_id, transaction_number, user_id, subtotal, 
         tax_amount, discount_amount, total_amount, payment_method, payment_reference, 
         amount_paid, change_amount, status, notes, created_at"
    )
    .bind(tenant_uuid)
    .bind(branch_uuid)
    .bind(user_uuid)
    .bind(&transaction_number)
    .bind(request.subtotal)
    .bind(request.tax_amount)
    .bind(request.discount_amount)
    .bind(request.total_amount)
    .bind(&request.payment_method)
    .bind(&request.payment_reference)
    .bind(request.amount_paid)
    .bind(request.change_amount)
    .bind(&request.notes)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| format!("Failed to create transaction: {}", e))?;
    
    let transaction_id = row.id;
    
    // Create transaction items
    for item in &request.items {
        let product_uuid = uuid::Uuid::parse_str(&item.product_id)
            .map_err(|e| format!("Invalid product ID: {}", e))?;
        
        sqlx::query(
            "INSERT INTO transaction_items (transaction_id, product_id, product_name, product_sku, 
             quantity, unit_price, discount_percent, discount_amount, subtotal) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
        )
        .bind(transaction_id)
        .bind(product_uuid)
        .bind(&item.product_name)
        .bind(&item.product_sku)
        .bind(item.quantity)
        .bind(item.unit_price)
        .bind(item.discount_percent)
        .bind(item.discount_amount)
        .bind(item.subtotal)
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("Failed to create transaction item: {}", e))?;
        
        // Update stock - fail transaction if stock update fails
        let stock_update = sqlx::query(
            "UPDATE branch_products SET stock_quantity = stock_quantity - $1 
             WHERE branch_id = $2 AND product_id = $3"
        )
        .bind(item.quantity)
        .bind(branch_uuid)
        .bind(product_uuid)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update stock for product {}: {}", item.product_id, e);
            format!("Failed to update stock for product: {}", e)
        })?;
        
        // Check if stock was actually updated (product exists in branch)
        if stock_update.rows_affected() == 0 {
            tracing::warn!("Product {} not found in branch {}, skipping stock update", product_uuid, branch_uuid);
            // Note: This is acceptable - some products may not be in all branches
        }
    }
    
    // Commit transaction
    tx.commit().await
        .map_err(|e| format!("Failed to commit transaction: {}", e))?;
    
    tracing::info!("Transaction created successfully: {}", transaction_number);
    
    Ok(TransactionResponse {
        id: row.id.to_string(),
        tenant_id: row.tenant_id.to_string(),
        branch_id: row.branch_id.to_string(),
        transaction_number: row.transaction_number,
        user_id: row.user_id.map(|id| id.to_string()),
        subtotal: row.subtotal,
        tax_amount: row.tax_amount,
        discount_amount: row.discount_amount,
        total_amount: row.total_amount,
        payment_method: row.payment_method,
        payment_reference: row.payment_reference,
        amount_paid: row.amount_paid,
        change_amount: row.change_amount,
        status: row.status,
        notes: row.notes,
        created_at: row.created_at.to_rfc3339(),
    })
}

#[tauri::command]
pub async fn get_transactions(
    tenant_id: String,
    _branch_id: Option<String>,
    _start_date: Option<String>,
    _end_date: Option<String>,
) -> Result<Vec<TransactionResponse>, String> {
    tracing::info!("Getting transactions for tenant: {}", tenant_id);
    
    let tenant_uuid = uuid::Uuid::parse_str(&tenant_id)
        .map_err(|e| format!("Invalid tenant ID: {}", e))?;
    
    let pool = db::get_db_pool().await
        .map_err(|e| format!("Database error: {}", e))?;
    
    // For simplicity, just get last 100 transactions
    let rows = sqlx::query_as::<_, TransactionRow>(
        "SELECT id, tenant_id, branch_id, transaction_number, user_id, subtotal, 
        tax_amount, discount_amount, total_amount, payment_method, payment_reference, 
        amount_paid, change_amount, status, notes, created_at 
        FROM transactions WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT 100"
    )
    .bind(tenant_uuid)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Failed to get transactions: {}", e))?;
    
    Ok(rows.into_iter().map(|r| TransactionResponse {
        id: r.id.to_string(),
        tenant_id: r.tenant_id.to_string(),
        branch_id: r.branch_id.to_string(),
        transaction_number: r.transaction_number,
        user_id: r.user_id.map(|id| id.to_string()),
        subtotal: r.subtotal,
        tax_amount: r.tax_amount,
        discount_amount: r.discount_amount,
        total_amount: r.total_amount,
        payment_method: r.payment_method,
        payment_reference: r.payment_reference,
        amount_paid: r.amount_paid,
        change_amount: r.change_amount,
        status: r.status,
        notes: r.notes,
        created_at: r.created_at.to_rfc3339(),
    }).collect())
}

#[tauri::command]
pub async fn void_transaction(transaction_id: String, reason: String) -> Result<TransactionResponse, String> {
    tracing::info!("Voiding transaction: {}", transaction_id);
    
    let uuid = uuid::Uuid::parse_str(&transaction_id)
        .map_err(|e| format!("Invalid transaction ID: {}", e))?;
    
    let pool = db::get_db_pool().await
        .map_err(|e| format!("Database error: {}", e))?;
    
    // Update transaction status
    let row = sqlx::query_as::<_, TransactionRow>(
        "UPDATE transactions SET status = 'voided', notes = CONCAT(COALESCE(notes, ''), ' | Voided: ', $2) 
         WHERE id = $1 
         RETURNING id, tenant_id, branch_id, transaction_number, user_id, subtotal, 
         tax_amount, discount_amount, total_amount, payment_method, payment_reference, 
         amount_paid, change_amount, status, notes, created_at"
    )
    .bind(uuid)
    .bind(&reason)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Failed to void transaction: {}", e))?;
    
    tracing::info!("Transaction voided successfully: {}", row.transaction_number);
    
    Ok(TransactionResponse {
        id: row.id.to_string(),
        tenant_id: row.tenant_id.to_string(),
        branch_id: row.branch_id.to_string(),
        transaction_number: row.transaction_number,
        user_id: row.user_id.map(|id| id.to_string()),
        subtotal: row.subtotal,
        tax_amount: row.tax_amount,
        discount_amount: row.discount_amount,
        total_amount: row.total_amount,
        payment_method: row.payment_method,
        payment_reference: row.payment_reference,
        amount_paid: row.amount_paid,
        change_amount: row.change_amount,
        status: row.status,
        notes: row.notes,
        created_at: row.created_at.to_rfc3339(),
    })
}

#[derive(Debug, sqlx::FromRow)]
struct TransactionRow {
    id: uuid::Uuid,
    tenant_id: uuid::Uuid,
    branch_id: uuid::Uuid,
    transaction_number: String,
    user_id: Option<uuid::Uuid>,
    subtotal: f64,
    tax_amount: f64,
    discount_amount: f64,
    total_amount: f64,
    payment_method: String,
    payment_reference: Option<String>,
    amount_paid: f64,
    change_amount: f64,
    status: String,
    notes: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
}
