use axum::{
    // routing::{get_service, MethodRouter}, // Axum 0.7+ 스타일, 일단 주석
    Router,
    routing::{delete, get, post}
};
use axum_rest_api::{handlers, middleware, init_app, models};
// Arc 추가
use utoipa::OpenApi;
use utoipa::Modify; // Modify 트레잇 임포트
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme}; // ApiKeyValue 임포트 확인
use utoipa_swagger_ui::SwaggerUi; // 다시 활성화
// use tower_http::services::ServeDir; // 일단 주석

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "ApiKeyAuth", // 보안 스키마 이름
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("X-Admin-API-Key"))), // ApiKeyValue 사용
        );
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        // 여기에 핸들러 함수들을 등록합니다.
        handlers::user::create_user,
        handlers::user::create_user_db,
        handlers::user::list_users,
        handlers::user::list_users_db,
        handlers::user::delete_user,
        handlers::user::get_app_state,
        handlers::item::show_item,
        handlers::item::add_item,
    ),
    components(
        schemas(
            // 여기에 API에서 사용하는 모델(스키마)들을 등록합니다.
            models::User,
            models::UserItem,
            models::Page,
            models::BodyItem,
            models::CreateUserRequest,
            // Item 모델이 있다면 추가: models::Item
        )
        // security_schemes 직접 정의 제거
    ),
    modifiers(&SecurityAddon), // modifiers를 사용하여 보안 스키마 추가
    tags(
        (name = "axum-rest-api", description = "Axum REST API endpoints")
    )
    // security(...) // 전역 보안 요구사항은 여기서 정의 가능
)]
struct ApiDoc;

//--- tokio main ----------------
// https://www.twilio.com/en-us/blog/build-high-performance-rest-apis-rust-axum
// DB: my local docker: practical-go DB 사용
// -e MYSQL_ROOT_PASSWORD=rootpassword \
// -e MYSQL_DATABASE=package_server \
// -e MYSQL_USER=packages_rw \
// -e MYSQL_PASSWORD=password \
//---------------------------------

// -- 스웨거 UI ---
// http://localhost:3000/swagger-ui/
// http://localhost:3000/api-docs/openapi.json
//---------------------------------

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = init_app().await?;
    let shared_state = config.app_state.clone(); // 상태 복제

    let admin_routes = Router::new()
        .route("/get_app_state", get(handlers::get_app_state)
            .route_layer(axum::middleware::from_fn_with_state(
                shared_state.clone(), // 복제된 상태 사용
                middleware::auth_middleware,
            ))
        )
        .with_state(shared_state.clone()); // admin_routes에도 상태 적용

    // SwaggerUi 객체를 생성 (별도 변수 제거)
    let swagger_route: SwaggerUi = SwaggerUi::new("/swagger-ui")
        .url("/api-docs/openapi.json", ApiDoc::openapi())
        .into();

    let app = Router::new()
        .merge(swagger_route) 
        .route("/", get(|| async { "hello, Rust!" }))
        .route("/create-user", post(handlers::create_user))
        .route("/create-user-db", post(handlers::create_user_db))
        .route("/users", get(handlers::list_users))
        .route("/axum-users", get(handlers::list_users_db))
        .route("/item/:id", get(handlers::show_item))
        .route("/add-item", post(handlers::add_item))
        .route("/delete-user/:id", delete(handlers::delete_user))
        .nest("/admin", admin_routes)
        .layer(axum::middleware::from_fn(middleware::logging_middleware))
        .layer(axum::extract::Extension(config.db_pool))
        .with_state(shared_state); // 최종적으로 전체 라우터에 상태 적용

    let addr_str = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr_str).await?; // TcpListener 사용
    println!("서버가 http://{} 에서 실행 중입니다", listener.local_addr()?); // listener.local_addr() 사용

    // app을 ConnectInfo를 제공하는 서비스로 변환
    let app_with_connect_info = app.into_make_service_with_connect_info::<std::net::SocketAddr>();

    axum::serve(listener, app_with_connect_info) // 수정된 서비스 사용
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tokio::time::sleep(std::time::Duration::from_secs(2)).await; // 이 부분은 테스트용으로 유지
    println!("서버가 성공적으로 종료되었습니다.");

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
    println!("SIGINT 신호 수신, 서버 종료 시작...");
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