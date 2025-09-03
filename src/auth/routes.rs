use axum::{routing::{post,get},Router};
use crate::auth::handlers::{signup,login,get_user};
use sqlx::PgPool;
pub fn create_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/auth/signup", post(signup))
        .route("/auth/login", post(login))
        .route("/user/{id}", get(get_user))
        .with_state(pool)
}