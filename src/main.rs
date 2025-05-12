use axum::{ Router, Server, routing::{delete, get, post} };

use axum_rest_api::{handlers, middleware, init_app};

//--- tokio main ----------------
// https://www.twilio.com/en-us/blog/build-high-performance-rest-apis-rust-axum
// DB: my local docker: practical-go DB 사용
// -e MYSQL_ROOT_PASSWORD=rootpassword \
// -e MYSQL_DATABASE=package_server \
// -e MYSQL_USER=packages_rw \
// -e MYSQL_PASSWORD=password \
//---------------------------------

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = init_app().await?;

    let admin_routes = Router::new()
        .route("/get_app_state", get(handlers::get_app_state)
        .route_layer(axum::middleware::from_fn_with_state(
            config.app_state.clone(),
            middleware::auth_middleware,
        ))
    );
    let app = Router::new()
        .route("/", get(|| async { "hello, Rust!" }))
        .route("/create-user", post(handlers::create_user))
        .route("/create-user-db", post(handlers::create_user_db))
        .route("/users", get(handlers::list_users))
        .route("/axum-users", get(handlers::list_users_db))
        .route("/item/:id", get(handlers::show_item))
        .route("/add-item", post(handlers::add_item))
        .route("/delete-user/:id", delete(handlers::delete_user))
        .nest("/admin", admin_routes)  //-- /admin 경로에 인증 적용
        .with_state(config.app_state.clone())
        .layer(axum::middleware::from_fn(middleware::logging_middleware))
        .layer(axum::extract::Extension(config.db_pool))
        .into_make_service_with_connect_info::<std::net::SocketAddr>();

    let addr = format!("{}:{}", config.host, config.port);
    println!("서버가 http://{} 포트에서 실행 중입니다", addr);

    Server::bind(&addr.parse()?)
        .serve(app)
        .await?;
    
    Ok(())
}

// --------------------
// > tree ./axum-rest-api -L 3 -a -I "target" -I ".git"
// ./axum-rest-api
// ├── .env
// ├── .env.example
// ├── .git_access_token
// ├── .gitignore
// ├── Cargo.lock
// ├── Cargo.toml
// ├── README.md
// ├── src
// │   ├── handlers
// │   │   ├── item.rs
// │   │   ├── mod.rs
// │   │   └── user.rs
// │   ├── lib.rs
// │   ├── main.rs
// │   ├── middleware
// │   │   └── mod.rs
// │   └── models
// │       └── mod.rs
// ├── target
// │   ├── .rustc_info.json
// │   ├── CACHEDIR.TAG
// │   └── debug
// │       ├── .cargo-lock
// │       ├── .fingerprint
// │       ├── axum-rest-api
// │       ├── axum-rest-api.d
// │       ├── build
// │       ├── deps
// │       ├── examples
// │       ├── incremental
// │       ├── libaxum_rest_api.d
// │       └── libaxum_rest_api.rlib
// ├── test-create-user-db.sh
// └── test-script.sh

//-- client test ----------------
// curl -X POST http://localhost:3000/create-user
// curl http://localhost:3000/users | jq
// curl "http://localhost:3000/item/42?number=2"
// curl -X POST http://localhost:3000/add-item \
//     -H "Content-Type: application/json" \
//     -d '{"title": "Some random item"}'
// curl -X DELETE http://localhost:3000/delete-user/2
// curl http://localhost:3000/axum-users | jq