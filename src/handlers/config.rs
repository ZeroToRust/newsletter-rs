use dotenvy::{self, dotenv, var};
use sqlx::PgPool;
pub async fn get_database_pool() -> PgPool {
    dotenvy::from_filename(".env").ok();
    // Load the DATABASE_URL from the environment
    dotenv().ok(); // Loads env variables from the .env file
    let durl = var("DATABASE_URL").unwrap(); // Gets the database url from the .env file
                                             // Create and return the connection pool
    let pool = PgPool::connect(&durl)
        .await
        .expect("Failed to connect to database");
    pool
}
