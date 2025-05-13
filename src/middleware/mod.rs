use axum::{
    body::Body,
    extract::{ConnectInfo, State},
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use std::net::SocketAddr;
use std::time::Instant;
use std::sync::Arc;
use crate::AppState;


pub async fn auth_middleware(
    State(app_state): State<Arc<AppState>>,
    req: Request<Body>,
    next: Next,
) -> impl IntoResponse {
    let auth_header = req.headers()
        .get("X-Admin-API-Key")
        .and_then(|header| header.to_str().ok());

    match auth_header {
        Some(key) if key == app_state.admin_api_key => {
            next.run(req).await
        },
        _ => {
            (StatusCode::UNAUTHORIZED, "Invalid API Key").into_response()
        }
    }
}

pub async fn logging_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request<Body>,
    next: Next,
) -> impl IntoResponse {
    let start = Instant::now();

    let client_ip = req
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .map(String::from)
        .unwrap_or_else(|| addr.ip().to_string());

    let method = req.method().clone();
    let uri = req.uri().clone();

    // println!("--> {} {} {}", client_ip, method, uri);

    let response = next.run(req).await;
    let duration = start.elapsed();

    println!("|-> {} {} {} ({}) [{} ms]", client_ip, method, uri, response.status(), duration.as_millis());

    response
}

