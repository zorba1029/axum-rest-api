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

// 결론부터 말씀드리면, sqlx::Pool은 일반적으로 명시적으로 닫아줄 필요가 없습니다.
// 그 이유는 다음과 같습니다:
// 1] Drop Trait 구현: sqlx::Pool 타입은 Rust의 Drop 트레잇을 구현하고 있습니다. 
// Rust에서는 어떤 값(value)이 스코프(scope)를 벗어날 때 자동으로 해당 값의 Drop 트레잇에 정의된 코드가 실행됩니다.
// 2] 자동 정리: 애플리케이션이 정상적으로 종료될 때 (main 함수가 끝나거나 Ok(())를 반환할 때), 
// main 함수 내에서 생성된 db_pool 변수도 스코프를 벗어나게 됩니다. 
// 이때 sqlx::Pool의 Drop 구현이 호출되어 내부적으로 다음 작업들을 수행합니다:
//  - 새로운 커넥션 요청을 받지 않습니다.
//  - 현재 사용 중이지 않은 유휴(idle) 커넥션들을 닫습니다.
//  - 만약 현재 사용 중인 커넥션이 있다면, 해당 작업이 완료될 때까지 잠시 기다린 후 닫습니다. 
//    (내부적으로 타임아웃이 있을 수 있습니다.)
