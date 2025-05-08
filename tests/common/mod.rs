use axum::Router;
use dotenvy::dotenv;
use newsletter_rs::startup::build_app;

pub fn spawn_app() -> Router {
    build_app()
}
use sqlx::PgPool;
#[allow(dead_code)]
pub async fn get_database_pool() -> PgPool {
    dotenv().ok();
    // Load the DATABASE_URL from the environment
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create and return the connection pool
    PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database")
}
