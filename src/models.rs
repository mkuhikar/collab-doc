use axum::{extract::{State,Json},http::StatusCode};
use serde::{Deserialize,Serialize};
use sqlx::PgPool;
#[derive(Deserialize,Serialize)]
pub struct User {
    id: Option<i32>,
    name: String,
    email: String,
}
#[derive(sqlx::FromRow)]
struct UserId {
    id: i32,
}

pub async fn create_user(State(pool):State<PgPool> ,Json(user): Json<User>) -> Result<Json<User>, StatusCode> {
    // Here you would typically insert the user into a database
    let row: UserId = sqlx::query_as!(
    UserId,
    "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id",
    user.name,
    user.email
)
.fetch_one(&pool)
.await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(User {
        id: Some(row.id),
        name: user.name,
        email: user.email,
    }))
}
