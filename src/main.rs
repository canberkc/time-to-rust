use std::net::SocketAddr;
use std::str::FromStr;

use crate::domain::models::AppState;
use axum::handler::HandlerWithoutStateExt;
use axum::{response::IntoResponse, routing::get};
use axum::routing::{delete, post};
use sqlx::postgres::PgConnectOptions;
use sqlx::{ConnectOptions, PgPool, Pool, Postgres};
use tracing::info;

use crate::handler::default::{add_player, del_player, healthcheck, players, root};

mod domain;
mod handler;
mod service;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let connection_str = get_connection_string();
    let pool = get_connection_pool(connection_str.as_str()).await;
    let state = AppState { pool };
    let app = axum::Router::new()
        .route("/", get(root))
        .route("/healthcheck", get(healthcheck))
        .route("/players", get(players))
        .route("/players/player", post(add_player))
        .route("/players/:id", delete(del_player))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub fn get_connection_string() -> String {
    "postgres://test:test@localhost:5432/test".to_string()
}

pub async fn get_connection_pool(connection_string: &str) -> Pool<Postgres> {
    let options = PgConnectOptions::from_str(connection_string)
        .unwrap()
        .disable_statement_logging()
        .clone();

    PgPool::connect_with(options).await.unwrap()
}
