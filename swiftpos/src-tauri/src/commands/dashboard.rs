use serde::{Deserialize, Serialize};
use sqlx::Row;
use crate::db;
use crate::middleware;
use tauri::State;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct SalesByPeriod {
    pub daily: f64,
    pub weekly: f64,
    pub monthly: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitTransactionCount {
    pub unit_id: String,
    pub unit_name: String,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardStatsResponse {
    pub sales_by_period: SalesByPeriod,
    pub transactions_by_unit: Vec<UnitTransactionCount>,
    pub total_products: i64,
    pub low_stock_products: i64,
}

#[derive(Debug, Deserialize)]
pub struct DashboardStatsRequest {
    pub token: String,
}

#[tauri::command]
pub async fn get_dashboard_stats(
    request: DashboardStatsRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<DashboardStatsResponse, String> {
    let app_state = state.read().await;
    let auth = middleware::validate_token(&request.token, &app_state).await?;
    drop(app_state);

    let tenant_id = auth.tenant_id.ok_or_else(|| "Tenant ID tidak ditemukan".to_string())?;
    let tenant_uuid = uuid::Uuid::parse_str(&tenant_id).map_err(|e| e.to_string())?;

    let pool = db::get_db_pool().await.map_err(|e| e.to_string())?;

    // 1. Sales by period
    // Daily - using simple COALESCE(SUM, 0) and filtering by current date
    let daily_sales: f64 = sqlx::query_scalar::<sqlx::Postgres, f64>(
        r#"SELECT COALESCE(SUM(total_amount), 0)::float8 FROM transactions 
         WHERE tenant_id = $1 AND created_at >= CURRENT_DATE AND (status = 'completed' OR status IS NULL)"#
    )
    .bind(tenant_uuid)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Error daily sales: {}", e))?;

    // Weekly
    let weekly_sales: f64 = sqlx::query_scalar::<sqlx::Postgres, f64>(
        r#"SELECT COALESCE(SUM(total_amount), 0)::float8 FROM transactions 
         WHERE tenant_id = $1 AND created_at >= date_trunc('week', CURRENT_DATE) AND (status = 'completed' OR status IS NULL)"#
    )
    .bind(tenant_uuid)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Error weekly sales: {}", e))?;

    // Monthly
    let monthly_sales: f64 = sqlx::query_scalar::<sqlx::Postgres, f64>(
        r#"SELECT COALESCE(SUM(total_amount), 0)::float8 FROM transactions 
         WHERE tenant_id = $1 AND created_at >= date_trunc('month', CURRENT_DATE) AND (status = 'completed' OR status IS NULL)"#
    )
    .bind(tenant_uuid)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Error monthly sales: {}", e))?;

    // 2. Transactions by unit
    let transactions_by_unit_rows = sqlx::query(
        r#"SELECT b.id as unit_id, b.name as unit_name, COUNT(t.id) as count 
         FROM branches b
         LEFT JOIN transactions t ON b.id = t.branch_id
         WHERE b.tenant_id = $1
         GROUP BY b.id, b.name"#
    )
    .bind(tenant_uuid)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error trx by unit: {}", e))?;

    let transactions_by_unit = transactions_by_unit_rows
        .into_iter()
        .map(|r| UnitTransactionCount {
            unit_id: r.get::<uuid::Uuid, _>("unit_id").to_string(),
            unit_name: r.get("unit_name"),
            count: r.get::<Option<i64>, _>("count").unwrap_or(0),
        })
        .collect();

    // 3. Other stats
    let total_products: i64 = sqlx::query_scalar::<sqlx::Postgres, i64>(
        "SELECT COUNT(*) FROM products WHERE tenant_id = $1 AND is_active = true"
    )
    .bind(tenant_uuid)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Error total products: {}", e))?;

    let low_stock_products: i64 = sqlx::query_scalar::<sqlx::Postgres, i64>(
        "SELECT COUNT(*) FROM products WHERE tenant_id = $1 AND is_active = true AND stock <= stock_min"
    )
    .bind(tenant_uuid)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Error low stock: {}", e))?;

    Ok(DashboardStatsResponse {
        sales_by_period: SalesByPeriod {
            daily: daily_sales,
            weekly: weekly_sales,
            monthly: monthly_sales,
        },
        transactions_by_unit,
        total_products,
        low_stock_products,
    })
}
