use axum::{routing::{post,get},Router};
use crate::models::{create_user,get_user};
use sqlx::PgPool;
pub fn create_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/user", post(create_user))
        .route("/user/{id}", get(get_user))
        .with_state(pool)
}