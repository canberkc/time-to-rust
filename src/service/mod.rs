use crate::domain::models::Player;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn list_players(pool: &PgPool) -> anyhow::Result<Vec<Player>> {
    let players: Vec<Player> = sqlx::query_as("SELECT * FROM player")
        .fetch_all(pool)
        .await?;

    Ok(players)
}

pub async fn insert_player(pool: &PgPool, player: Player) -> anyhow::Result<Uuid> {
    sqlx::query(
        "INSERT INTO player (id, name, lastname, team, nationality) VALUES ($1, $2, $3,$4, $5)",
    )
    .bind(player.id)
    .bind(player.name)
    .bind(player.lastname)
    .bind(player.team)
    .bind(player.nationality)
    .execute(pool)
    .await?;

    Ok(player.id)
}

pub async fn delete_player(pool: &PgPool, id: Uuid) -> anyhow::Result<u64> {
    let affected_rows = sqlx::query("DELETE from player where id = $1")
        .bind(id)
        .execute(pool)
        .await
        .unwrap()
        .rows_affected();

    Ok(affected_rows)
}
