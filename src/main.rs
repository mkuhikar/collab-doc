use axum::Router;
mod routes;
mod models;
mod db;


#[tokio::main]
async fn main() {
    let pool = db::init_db().await;
    // build our application with a single route
    let app = Router::new().merge(routes::create_routes(pool));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

