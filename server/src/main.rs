use axum::{Router};
mod auth;
mod doc;
use auth::routes as auth_routes;
use doc::routes as doc_routes;
use doc::handlers::{autosave_loop};
use doc::sessions::DocSession;
use tower_http::cors::{CorsLayer, Any};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use dashmap::DashMap;
use sqlx::PgPool;
use uuid::Uuid;

use doc::handlers::AppState;
// use auth::models::*;
mod db;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();

    // init DB pool
    let pool: PgPool = db::init_db().await;

    // sessions map is itself inside an Arc so it can be cheaply cloned and shared
    let sessions: Arc<DashMap<Uuid, Arc<tokio::sync::Mutex<DocSession>>>> =
        Arc::new(DashMap::new());

    // Build an AppState value (clone of pool + sessions Arc)
    // Make sure AppState derives or implements Clone if you want to clone it.
    let app_state = AppState {
        pool: pool.clone(),      // PgPool is cheap to clone
        sessions: sessions.clone(),
    };
    // Wrap the AppState in an Arc for shared ownership across tasks/handlers
    let shared_state: Arc<AppState> = Arc::new(app_state);

    // Spawn the autosave loop using Arc<AppState>
    {
        let autosave_state = Arc::clone(&shared_state);
        tokio::spawn(async move {
            // autosave_loop should accept Arc<AppState>
            autosave_loop(autosave_state).await;
        });
    }

    // Build routers:
    // - auth_routes requires just a PgPool
    // - doc_routes requires an owned AppState (we clone the inner AppState)
    //
    // Important: doc_routes::create_routes takes `AppState` (owned). We have Arc<AppState>,
    // so we clone the inner AppState using (*shared_state).clone()
    // (This requires AppState: Clone. If AppState does not implement Clone, make doc_routes accept Arc<AppState> instead.)
    let auth_router = auth_routes::create_routes(pool.clone());
    let doc_router = doc_routes::create_routes((*shared_state).clone());

    let app = Router::new()
        .merge(auth_router)
        .merge(doc_router);


    // bind & serve
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
   axum::serve(listener, app).await?;


    Ok(())
}