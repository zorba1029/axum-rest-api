mod handlers;
mod middleware;
mod models;

use axum::{
    routing::{delete, get, post}, Router, Server
};
use sqlx::mysql::MySqlPool;
use dotenvy::dotenv;
use std::env;

//--- tokio main ----------------
// https://www.twilio.com/en-us/blog/build-high-performance-rest-apis-rust-axum
// DB: my local docker: practical-go DB 사용
// -e MYSQL_ROOT_PASSWORD=rootpassword \
// -e MYSQL_DATABASE=package_server \
// -e MYSQL_USER=packages_rw \
// -e MYSQL_PASSWORD=password \
// --------------------
// > tree ./axum-rest-api -L 3 -a -I "target" --dirsfirst
// ./axum-rest-api
// ├── .env
// ├── .env.example
// ├── .git
// │   ├── config
// │   ├── description
// │   ├── HEAD
// │   ├── hooks
// │   │   └── README.sample
// │   ├── info
// │   │   └── exclude
// │   ├── objects
// │   │   ├── info
// │   │   └── pack
// │   └── refs
// │       ├── heads
// │       └── tags
// ├── .gitignore
// ├── Cargo.lock
// ├── Cargo.toml
// ├── src
// │   ├── handlers
// │   │   ├── item.rs
// │   │   ├── mod.rs
// │   │   └── user.rs
// │   ├── main.rs
// │   ├── middleware
// │   │   └── mod.rs
// │   └── models
// │       └── mod.rs
// └── test-script.sh

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    
    let db_pool = MySqlPool::connect(&db_url)
        .await
        .expect("Failed to connect to DB");

    let app = Router::new()
        .route("/", get(|| async { "hello, Rust!" }))
        .route("/create-user", post(handlers::create_user))
        .route("/users", get(handlers::list_users))
        .route("/item/:id", get(handlers::show_item))
        .route("/add-item", post(handlers::add_item))
        .route("/delete-user/:id", delete(handlers::delete_user))
        .route("/axum-users", get(handlers::get_axum_users))
        .layer(axum::middleware::from_fn(middleware::logging_middleware))
        .layer(axum::extract::Extension(db_pool))
        .into_make_service_with_connect_info::<std::net::SocketAddr>();

    let addr = format!("{}:{}", host, port);
    println!("서버가 http://{} 포트에서 실행 중입니다", addr);

    // Server::bind(&"0.0.0.0:3000".parse()?).serve(app.into_make_service()).await?;
    Server::bind(&addr.parse()?)
        .serve(app)
        .await?;
    
    Ok(())
}


//-- client test ----------------
// curl -X POST http://localhost:3000/create-user
// curl http://localhost:3000/users | jq
// curl "http://localhost:3000/item/42?number=2"
// curl -X POST http://localhost:3000/add-item \
//     -H "Content-Type: application/json" \
//     -d '{"title": "Some random item"}'
// curl -X DELETE http://localhost:3000/delete-user/2
// curl http://localhost:3000/axum-users | jq