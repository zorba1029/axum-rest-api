use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::IntoResponse,
};
use std::time::Instant;


pub async fn logging_middleware(req: Request<Body>, next: Next<Body>) -> impl IntoResponse {
    let start = Instant::now();

    let client_ip = req
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok().map(String::from))
        .or_else(|| {
            req.extensions()
                .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
                .map(|addr| addr.0.ip().to_string())
        })
        .unwrap_or_else(|| "UNKNOWN".to_string());

    let method = req.method().clone();
    let uri = req.uri().clone();

    // println!("--> {} {} {}", client_ip, method, uri);

    let response = next.run(req).await;
    let duration = start.elapsed();

    println!("|-> {} {} {} ({}) [{} ms]", client_ip, method, uri, response.status(), duration.as_millis());

    response
}