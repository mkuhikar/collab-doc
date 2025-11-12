use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use dotenvy::dotenv;

pub async fn init_db() -> PgPool {
    dotenv().ok(); // load .env file
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5) // adjust depending on your workload
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database")
}
