use dotenvy::{dotenv, var};
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

impl AppState {
    pub async fn new() -> Self {
        dotenv().ok();
        let database_url = var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = PgPool::connect(&database_url)
            .await
            .expect("Failed to create database connection pool");

        Self { db: pool }
    }
}
