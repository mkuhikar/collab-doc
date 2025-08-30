use axum::{extract::{State,Json},http::StatusCode};
use serde::{Deserialize,Serialize};
use sqlx::PgPool;
use axum::extract::Path;

#[derive(Debug,Deserialize,Serialize)]
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

pub async fn get_user(
    State(pool): State<PgPool>, // DB pool from app state
    Path(user_id): Path<i32>,   // Extract ID from URL
) -> Result<Json<User>, (axum::http::StatusCode, String)> {
    let result = sqlx::query_as!(
    User,
    "SELECT id, name, email FROM users WHERE id = $1",
    user_id
)
.fetch_one(&pool)
.await;


    match result {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err((
            axum::http::StatusCode::NOT_FOUND,
            format!("User not found: {}", e),
        )),
    }
}
