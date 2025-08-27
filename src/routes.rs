use axum::{routing::post,Router};
use crate::models::create_user;
use sqlx::PgPool;
pub fn create_routes(pool: PgPool) -> Router {
    Router::new().route("/user",post(create_user)).with_state(pool)
}