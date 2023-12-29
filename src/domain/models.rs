use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: Pool<Postgres>,
}
#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub lastname: String,
    pub team: String,
    pub nationality: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerDTO {
    pub name: String,
    pub lastname: String,
    pub team: String,
    pub nationality: String,
}
