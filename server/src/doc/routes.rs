use axum::{Router, extract::State, routing::{delete, get, post, put}};
use sqlx::PgPool;

use crate::doc::handlers::{AppState,create_doc, get_doc, update_doc, delete_doc, share_doc, get_doc_collaborators, get_user_docs, ws_handler};
use tower_http::cors::{CorsLayer, Any};

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/docs", post(create_doc))
        .route("/docs/{id}", get(get_doc))
        .route("/docs/{id}", put(update_doc))
        .route("/docs/{id}", delete(delete_doc))
        .route("/users/docs", get(get_user_docs))
        .route("/docs/{id}/share", post(share_doc))
        .route("/docs/{id}/collaborators", get(get_doc_collaborators))
        .route("/ws/docs/{id}", get(ws_handler))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:5173".parse::<axum::http::HeaderValue>().unwrap())
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(state)
    
}


