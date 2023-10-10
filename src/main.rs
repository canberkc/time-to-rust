use std::net::SocketAddr;

use axum::handler::HandlerWithoutStateExt;
use axum::{response::IntoResponse, routing::get};
use tracing::info;

use crate::handler::default::{healthcheck, root};

mod domain;
mod handler;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let app = axum::Router::new()
        .route("/", get(root))
        .route("/healthcheck", get(healthcheck));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Expect shutdown signal handler");
    println!("signal shutdown");
}
