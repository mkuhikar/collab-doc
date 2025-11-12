use axum::{routing::{post,get},Router};
use crate::auth::handlers::{signup,login,get_user, get_user_by_email};
use tower_http::cors::{CorsLayer, Any};

use sqlx::PgPool;
pub fn create_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/auth/signup", post(signup))
        .route("/auth/login", post(login))
        .route("/user/{id}", get(get_user))
        .route("/user/email/{email}", get(get_user_by_email))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:5173".parse::<axum::http::HeaderValue>().unwrap()) // your frontend URL
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(pool)
}