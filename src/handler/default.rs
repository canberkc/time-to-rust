use axum::extract::{Path, State};
use axum::Json;
use uuid::Uuid;

use crate::domain::models::{AppState, Player, PlayerDTO};
use crate::service::{delete_player, insert_player, list_players};

pub async fn root() -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::OK, "Rust AXUM".to_string())
}

pub async fn healthcheck() -> (axum::http::StatusCode, String) {
    (
        axum::http::StatusCode::OK,
        "Hi I am great, don't worry :)".to_string(),
    )
}

pub async fn players(State(state): State<AppState>) -> Json<Vec<Player>> {
    let players = list_players(&state.pool)
        .await
        .expect("Could not list players");
    Json(players)
}

pub async fn add_player(
    State(state): State<AppState>,
    Json(player_dto): Json<PlayerDTO>,
) -> Json<Uuid> {
    let player = Player {
        id: Uuid::new_v4(),
        name: player_dto.name,
        lastname: player_dto.lastname,
        team: player_dto.team,
        nationality: player_dto.nationality,
    };

    let player_id = insert_player(&state.pool, player)
        .await
        .expect("Could not add player");

    Json(player_id)
}

pub async fn del_player(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> (axum::http::StatusCode, Json<serde_json::Value>) {
    let affected_rows = delete_player(&state.pool, id)
        .await
        .expect("Could not delete player");

    if affected_rows == 0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Note with ID: {} not found", id)
        });
        (axum::http::StatusCode::OK, Json(error_response))
    } else {
        let message = serde_json::json!({"message":format!("Player with id: {} has been deleted.", id.to_string())});
        (axum::http::StatusCode::OK, Json(message))
    }
}
