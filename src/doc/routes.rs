use axum::{Router, routing::{post, get, put, delete}};
use sqlx::PgPool;

use crate::doc::handlers::{create_doc, get_doc, update_doc, delete_doc, share_doc, get_collaborators, get_user_docs};

pub fn create_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/docs", post(create_doc))
        .route("/docs/{id}", get(get_doc))
        .route("/docs/{id}", put(update_doc))
        .route("/docs/{id}", delete(delete_doc))
        .route("/users/{id}/docs", get(get_user_docs))
        .route("/docs/{id}/share", post(share_doc))
        .route("/docs/{id}/collaborators", get(get_collaborators))
        .with_state(pool)
}


