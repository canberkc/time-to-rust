use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use std::net::SocketAddr;
use axum::handler::HandlerWithoutStateExt;

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/", get(root))
        .route("/healthcheck", get(healthcheck));


    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}


/// Tokio signal handler that will wait for a user to press CTRL+C.
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Expect shutdown signal handler");
    println!("signal shutdown");
}



pub async fn root() -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::OK, "Rust AXUM".to_string())
}


pub async fn healthcheck() -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::OK, "Hi I am great, don't worry :)".to_string())
}
