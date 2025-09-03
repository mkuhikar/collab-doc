use axum::Router;
mod auth;
mod doc;
use auth::routes as auth_routes;
use doc::routes as doc_routes;

// use auth::models::*;
mod db;


#[tokio::main]
async fn main() {
    let pool = db::init_db().await;
    // build our application with a single route
    let app = Router::new()
        .merge(auth_routes::create_routes(pool.clone()))
        .merge(doc_routes::create_routes(pool));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

