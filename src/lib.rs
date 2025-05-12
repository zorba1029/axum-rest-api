pub mod handlers;
pub mod middleware;
pub mod models;

use sqlx::mysql::MySqlPool;
use std::sync::Arc;
use std::env;
use dotenvy::dotenv;

pub use handlers::*;
pub use middleware::*;
pub use models::*;

pub struct AppState {
    pub db_name: String,
    pub db_user: String,
    pub db_host: String,
    pub db_port: String,
    pub server_host: String,
    pub server_port: String,
    pub admin_api_key: String,
}

pub struct AppConfig {
    pub db_pool: MySqlPool,
    pub app_state: Arc<AppState>,
    pub host: String,
    pub port: String,
}

pub async fn init_app() -> Result<AppConfig, Box<dyn std::error::Error>> {
    dotenv().ok();
    let db_name = env::var("DB_NAME").expect("DB_NAME must be set");
    let db_user = env::var("DB_USER").expect("DB_USER must be set");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
    let db_host = env::var("DB_HOST").expect("DB_HOST must be set");
    let db_port = env::var("DB_PORT").expect("DB_PORT must be set");
    let db_url = format!("mysql://{}:{}@{}:{}/{}", db_user, db_password, db_host, db_port, db_name);

    let admin_api_key = env::var("ADMIN_API_KEY").expect("ADMIN_API_KEY must be set");
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    
    let app_state = Arc::new(AppState {
        db_name: db_name.clone(),
        db_user: db_user.clone(),
        db_host: db_host.clone(),
        db_port: db_port.clone(),
        server_host: host.clone(),
        server_port: port.clone(),
        admin_api_key: admin_api_key.clone(),
    });

    let db_pool = MySqlPool::connect(&db_url)
        .await
        .expect("Failed to connect to DB");

    Ok(AppConfig {
        db_pool,
        app_state,
        host,
        port,
    })
}
