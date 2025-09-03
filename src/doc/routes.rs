use axum::{Router, routing::{post, get, put, delete}};
use sqlx::PgPool;

use crate::doc::handlers::{create_doc, get_doc, update_doc, delete_doc};

pub fn create_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/docs", post(create_doc))
        .route("/docs/{id}", get(get_doc).put(update_doc).delete(delete_doc))
        .with_state(pool)
}
