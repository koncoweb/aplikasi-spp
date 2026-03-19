use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;
use once_cell::sync::Lazy;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Validation error: {0}")]
    Validation(String),
}

pub type DbResult<T> = Result<T, DbError>;

// Global database pool
pub static DB_POOL: Lazy<Arc<RwLock<Option<PgPool>>>> = Lazy::new(|| Arc::new(RwLock::new(None)));

/// Initialize the database connection pool
pub async fn init_db_pool(database_url: &str) -> DbResult<()> {
    let pool = PgPool::connect(database_url).await?;
    let mut guard = DB_POOL.write().await;
    *guard = Some(pool);
    tracing::info!("Database connection pool initialized");
    Ok(())
}

/// Get the database pool
pub async fn get_db_pool() -> DbResult<Arc<PgPool>> {
    let guard = DB_POOL.read().await;
    match guard.as_ref() {
        Some(pool) => Ok(Arc::new((*pool).clone())),
        None => Err(DbError::NotFound("Database pool not initialized".to_string())),
    }
}

/// Tenant model
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Tenant {
    pub id: uuid::Uuid,
    pub name: String,
    pub slug: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub logo_url: Option<String>,
    pub application_name: String,
    pub subscription_tier: String,
    pub timezone: String,
    pub currency_code: String,
    pub currency_symbol: String,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}


#[derive(Debug, sqlx::FromRow)]
struct TenantRow {
    id: uuid::Uuid,
    name: String,
    slug: String,
    address: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    logo_url: Option<String>,
    application_name: String,
    subscription_tier: String,
    timezone: String,
    currency_code: String,
    currency_symbol: String,
    is_active: bool,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<TenantRow> for Tenant {
    fn from(row: TenantRow) -> Self {
        Tenant {
            id: row.id,
            name: row.name,
            slug: row.slug,
            address: row.address,
            phone: row.phone,
            email: row.email,
            logo_url: row.logo_url,
            application_name: row.application_name,
            subscription_tier: row.subscription_tier,
            timezone: row.timezone,
            currency_code: row.currency_code,
            currency_symbol: row.currency_symbol,
            is_active: row.is_active,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

impl Tenant {
    pub async fn find_all() -> DbResult<Vec<Self>> {
        let pool = get_db_pool().await?;
        let rows = sqlx::query_as::<sqlx::Postgres, TenantRow>(
            "SELECT id, name, slug, address, phone, email, logo_url, application_name, 
             subscription_tier, timezone, currency_code, currency_symbol, is_active, 
             created_at, updated_at FROM tenants"
        )
        .fetch_all(&*pool)
        .await?;
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn find_by_id(id: uuid::Uuid) -> DbResult<Self> {
        let pool = get_db_pool().await?;
        let row = sqlx::query_as::<sqlx::Postgres, TenantRow>(
            "SELECT id, name, slug, address, phone, email, logo_url, application_name, 
             subscription_tier, timezone, currency_code, currency_symbol, is_active, 
             created_at, updated_at FROM tenants WHERE id = $1"
        )
        .bind(id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => DbError::NotFound(format!("Tenant {} not found", id)),
            _ => DbError::Sqlx(e),
        })?;
        Ok(row.into())
    }

    pub async fn find_by_slug(slug: &str) -> DbResult<Self> {
        let pool = get_db_pool().await?;
        let row = sqlx::query_as::<sqlx::Postgres, TenantRow>(
            "SELECT id, name, slug, address, phone, email, logo_url, application_name, 
             subscription_tier, timezone, currency_code, currency_symbol, is_active, 
             created_at, updated_at FROM tenants WHERE slug = $1"
        )
        .bind(slug)
        .fetch_one(&*pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => DbError::NotFound(format!("Tenant with slug {} not found", slug)),
            _ => DbError::Sqlx(e),
        })?;
        Ok(row.into())
    }

    pub async fn create(name: &str, slug: &str, email: &str, phone: Option<&str>, address: Option<&str>) -> DbResult<Self> {
        let pool = get_db_pool().await?;
        let row = sqlx::query_as::<sqlx::Postgres, TenantRow>(
            "INSERT INTO tenants (name, slug, email, phone, address) 
             VALUES ($1, $2, $3, $4, $5) 
             RETURNING id, name, slug, address, phone, email, logo_url, application_name, 
             subscription_tier, timezone, currency_code, currency_symbol, is_active, 
             created_at, updated_at"
        )
        .bind(name)
        .bind(slug)
        .bind(email)
        .bind(phone)
        .bind(address)
        .fetch_one(&*pool)
        .await?;
        Ok(row.into())
    }

    pub async fn update(&self) -> DbResult<()> {
        let pool = get_db_pool().await?;
        sqlx::query(
            "UPDATE tenants SET name = $1, address = $2, phone = $3, updated_at = NOW() WHERE id = $4"
        )
        .bind(&self.name)
        .bind(&self.address)
        .bind(&self.phone)
        .bind(self.id)
        .execute(&*pool)
        .await?;
        Ok(())
    }
}

/// User model
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub tenant_id: Option<uuid::Uuid>,
    pub branch_id: Option<uuid::Uuid>,
    pub email: String,
    pub password_hash: String,
    pub full_name: String,
    pub role: String,
    pub is_active: bool,
    pub last_login: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl User {
    pub async fn find_by_id(id: uuid::Uuid) -> DbResult<Self> {
        let pool = get_db_pool().await?;
        let row = sqlx::query_as::<sqlx::Postgres, UserRow>(
            "SELECT id, tenant_id, branch_id, email, password_hash, full_name, role, is_active, 
             last_login, created_at, updated_at FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => DbError::NotFound(format!("User {} not found", id)),
            _ => DbError::Sqlx(e),
        })?;
        Ok(row.into())
    }
    
    pub async fn find_by_id_string(id: &str) -> DbResult<Self> {
        let uuid = uuid::Uuid::parse_str(id)
            .map_err(|_| DbError::Validation(format!("Invalid UUID: {}", id)))?;
        Self::find_by_id(uuid).await
    }
    
    pub async fn find_by_email(email: &str) -> DbResult<Self> {
        let pool = get_db_pool().await?;
        let row = sqlx::query_as::<sqlx::Postgres, UserRow>(
            "SELECT id, tenant_id, branch_id, email, password_hash, full_name, role, is_active, 
             last_login, created_at, updated_at FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_one(&*pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => DbError::NotFound(format!("User {} not found", email)),
            _ => DbError::Sqlx(e),
        })?;
        Ok(row.into())
    }
    
    pub async fn verify_password(&self, password: &str) -> bool {
        // Use argon2 to verify password
        use argon2::{Argon2, password_hash::PasswordHash, password_hash::PasswordVerifier};
        
        if self.password_hash.is_empty() {
            return false;
        }
        
        // Parse the stored hash
        let parsed_hash = match PasswordHash::new(&self.password_hash) {
            Ok(hash) => hash,
            Err(_) => return false,
        };
        
        // Verify the password
        Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
    }
    
    pub async fn update_last_login(&self) -> DbResult<()> {
        let pool = get_db_pool().await?;
        sqlx::query("UPDATE users SET last_login = NOW() WHERE id = $1")
            .bind(self.id)
            .execute(&*pool)
            .await?;
        Ok(())
    }

    pub async fn find_by_tenant(tenant_id: uuid::Uuid) -> DbResult<Vec<Self>> {
        let pool = get_db_pool().await?;
        let rows = sqlx::query_as::<sqlx::Postgres, UserRow>(
            "SELECT id, tenant_id, branch_id, email, password_hash, full_name, role, is_active, 
             last_login, created_at, updated_at FROM users WHERE tenant_id = $1"
        )
        .bind(tenant_id)
        .fetch_all(&*pool)
        .await?;
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
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

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        User {
            id: row.id,
            tenant_id: row.tenant_id,
            branch_id: row.branch_id,
            email: row.email,
            password_hash: row.password_hash,
            full_name: row.full_name,
            role: row.role,
            is_active: row.is_active,
            last_login: row.last_login,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

/// Branch model
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Branch {
    pub id: uuid::Uuid,
    pub tenant_id: uuid::Uuid,
    pub code: String,
    pub name: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub is_main_branch: bool,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Branch {
    pub async fn find_by_id(id: uuid::Uuid) -> DbResult<Self> {
        let pool = get_db_pool().await?;
        let row = sqlx::query_as::<sqlx::Postgres, BranchRow>(
            "SELECT id, tenant_id, code, name, address, phone, is_main_branch, 
             is_active, created_at FROM branches WHERE id = $1"
        )
        .bind(id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => DbError::NotFound(format!("Branch {} not found", id)),
            _ => DbError::Sqlx(e),
        })?;
        Ok(row.into())
    }
    
    pub async fn find_by_tenant(tenant_id: uuid::Uuid) -> DbResult<Vec<Self>> {
        let pool = get_db_pool().await?;
        let rows = sqlx::query_as::<sqlx::Postgres, BranchRow>(
            "SELECT id, tenant_id, code, name, address, phone, is_main_branch, 
             is_active, created_at FROM branches WHERE tenant_id = $1 AND is_active = true"
        )
        .bind(tenant_id)
        .fetch_all(&*pool)
        .await?;
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}

#[derive(Debug, sqlx::FromRow)]
struct BranchRow {
    id: uuid::Uuid,
    tenant_id: uuid::Uuid,
    code: String,
    name: String,
    address: Option<String>,
    phone: Option<String>,
    is_main_branch: bool,
    is_active: bool,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl From<BranchRow> for Branch {
    fn from(row: BranchRow) -> Self {
        Branch {
            id: row.id,
            tenant_id: row.tenant_id,
            code: row.code,
            name: row.name,
            address: row.address,
            phone: row.phone,
            is_main_branch: row.is_main_branch,
            is_active: row.is_active,
            created_at: row.created_at,
        }
    }
}

/// Category model
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Category {
    pub id: uuid::Uuid,
    pub tenant_id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub color: String,
    pub sort_order: i32,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Category {
    pub async fn find_by_tenant(tenant_id: uuid::Uuid) -> DbResult<Vec<Self>> {
        let pool = get_db_pool().await?;
        let rows = sqlx::query_as::<sqlx::Postgres, CategoryRow>(
            "SELECT id, tenant_id, name, description, color, sort_order, is_active, 
             created_at FROM categories WHERE tenant_id = $1 AND is_active = true 
             ORDER BY sort_order, name"
        )
        .bind(tenant_id)
        .fetch_all(&*pool)
        .await?;
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}

#[derive(Debug, sqlx::FromRow)]
struct CategoryRow {
    id: uuid::Uuid,
    tenant_id: uuid::Uuid,
    name: String,
    description: Option<String>,
    color: String,
    sort_order: i32,
    is_active: bool,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl From<CategoryRow> for Category {
    fn from(row: CategoryRow) -> Self {
        Category {
            id: row.id,
            tenant_id: row.tenant_id,
            name: row.name,
            description: row.description,
            color: row.color,
            sort_order: row.sort_order,
            is_active: row.is_active,
            created_at: row.created_at,
        }
    }
}

/// Product model
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Product {
    pub id: uuid::Uuid,
    pub tenant_id: uuid::Uuid,
    pub category_id: Option<uuid::Uuid>,
    pub sku: String,
    pub barcode: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub is_variant: bool,
    pub parent_product_id: Option<uuid::Uuid>,
    pub variant_name: Option<String>,
    pub unit: String,
    pub hpp: f64,
    pub selling_price: f64,
    pub stock: i32,
    pub stock_min: i32,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Product {
    pub async fn find_by_id(id: uuid::Uuid) -> DbResult<Self> {
        let pool = get_db_pool().await?;
        let row = sqlx::query_as::<sqlx::Postgres, ProductRow>(
            "SELECT id, tenant_id, category_id, sku, barcode, name, description, 
             image_url, is_variant, parent_product_id, variant_name, unit, 
             hpp, selling_price, stock, stock_min, is_active, created_at, updated_at 
             FROM products WHERE id = $1"
        )
        .bind(id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => DbError::NotFound(format!("Product {} not found", id)),
            _ => DbError::Sqlx(e),
        })?;
        Ok(row.into())
    }
    
    pub async fn find_by_tenant(tenant_id: uuid::Uuid) -> DbResult<Vec<Self>> {
        let pool = get_db_pool().await?;
        let rows = sqlx::query_as::<sqlx::Postgres, ProductRow>(
            "SELECT id, tenant_id, category_id, sku, barcode, name, description, 
             image_url, is_variant, parent_product_id, variant_name, unit, 
             hpp, selling_price, stock, stock_min, is_active, created_at, updated_at 
             FROM products WHERE tenant_id = $1 AND is_active = true ORDER BY name"
        )
        .bind(tenant_id)
        .fetch_all(&*pool)
        .await?;
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}

#[derive(Debug, sqlx::FromRow)]
struct ProductRow {
    id: uuid::Uuid,
    tenant_id: uuid::Uuid,
    category_id: Option<uuid::Uuid>,
    sku: String,
    barcode: Option<String>,
    name: String,
    description: Option<String>,
    image_url: Option<String>,
    is_variant: bool,
    parent_product_id: Option<uuid::Uuid>,
    variant_name: Option<String>,
    unit: String,
    hpp: f64,
    selling_price: f64,
    stock: i32,
    stock_min: i32,
    is_active: bool,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<ProductRow> for Product {
    fn from(row: ProductRow) -> Self {
        Product {
            id: row.id,
            tenant_id: row.tenant_id,
            category_id: row.category_id,
            sku: row.sku,
            barcode: row.barcode,
            name: row.name,
            description: row.description,
            image_url: row.image_url,
            is_variant: row.is_variant,
            parent_product_id: row.parent_product_id,
            variant_name: row.variant_name,
            unit: row.unit,
            hpp: row.hpp,
            selling_price: row.selling_price,
            stock: row.stock,
            stock_min: row.stock_min,
            is_active: row.is_active,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
