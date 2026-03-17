use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error, Level};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tauri::Manager;

pub mod db;
pub mod commands;
pub mod models;
pub mod utils;

// Application state - shared with auth commands
pub struct AppState {
    pub jwt_secret: String,
}

// Initialize logging
fn init_logging() {
    let log_dir = dirs::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("SwiftPOS")
        .join("logs");
    
    std::fs::create_dir_all(&log_dir).ok();
    
    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        &log_dir,
        "swiftpos.log"
    );
    
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    
    // Keep the guard alive for the lifetime of the application
    std::mem::forget(_guard);
    
    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_writer(non_blocking)
                .with_ansi(false)
        )
        .with(
            fmt::layer()
                .with_writer(std::io::stderr)
        )
        .with(EnvFilter::from_default_env().add_directive(Level::INFO.into()))
        .init();
    
    info!("SwiftPOS logging initialized");
    info!("Log directory: {:?}", log_dir);
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    info!("Greet command called with name: {}", name);
    format!("Hello, {}! Welcome to SwiftPOS!", name)
}

#[tauri::command]
fn get_app_info() -> serde_json::Value {
    serde_json::json!({
        "name": "SwiftPOS",
        "version": "1.0.0",
        "description": "Desktop Point of Sale Application"
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logging first
    init_logging();
    
    info!("Starting SwiftPOS application...");
    
    // Set up panic hook for logging
    std::panic::set_hook(Box::new(|panic_info| {
        error!("Application panicked: {}", panic_info);
    }));
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            info!("Tauri setup complete");
            
            // Get JWT secret from environment - REQUIRE it to be set
            let jwt_secret = std::env::var("JWT_SECRET")
                .map_err(|_| {
                    error!("JWT_SECRET environment variable is not set");
                    "JWT_SECRET must be set in environment"
                })?;
            
            if jwt_secret.len() < 32 {
                error!("JWT_SECRET is too short (minimum 32 characters)");
                return Err("JWT_SECRET must be at least 32 characters".into());
            }
            
            info!("JWT secret loaded from environment");
            
            // Get database URL from environment - REQUIRE it to be set
            let database_url = std::env::var("DATABASE_URL")
                .map_err(|_| {
                    error!("DATABASE_URL environment variable is not set");
                    "DATABASE_URL must be set in environment"
                })?;
            
            info!("Database URL loaded from environment");
            
            // Store state - use the same AppState type as auth commands
            app.manage(Arc::new(RwLock::new(AppState {
                jwt_secret: jwt_secret.clone(),
            })));
            
            info!("Application state initialized");
            
            // Initialize database pool in a blocking task
            let db_url = database_url.clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = db::init_db_pool(&db_url).await {
                    error!("Failed to initialize database pool: {}", e);
                }
            });
            
            info!("JWT secret configured");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet, 
            get_app_info,
            commands::auth::login,
            commands::auth::logout,
            commands::auth::register_tenant,
            commands::tenants::get_tenants,
            commands::tenants::get_tenant,
            commands::branches::get_branches,
            commands::branches::create_branch,
            commands::categories::get_categories,
            commands::categories::create_category,
            commands::products::get_products,
            commands::products::create_product,
            commands::transactions::create_transaction,
            commands::transactions::get_transactions,
            commands::transactions::void_transaction,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
