
use sqlx::postgres::PgPool;
use testcontainers_modules::{
    postgres::Postgres,
    testcontainers::{runners::AsyncRunner, ImageExt},
};
async fn setup_database() -> PgPool {
    let image = Postgres::default().with_tag("17.5"); 
    let container = image.start().await.expect("failed to launch image");

    // println!("The container log are {}", container.logs().stdout_utf8().unwrap_or_else(|_| "unable to get logs".to_string()));
    let port = container
        .get_host_port_ipv4(5432)
        .await
        .expect("failed to find port");
    let conn_str = format!("postgres://postgres:postgres@localhost:{:?}/postgres", port);

    let pool = PgPool::connect(&conn_str)
        .await
        .unwrap_or_else(|e| panic!("Database connection failed: {}", e));

        //.expect("Failed to connect to PostgreSQL");

        // let pool2 = pool.clone();
    sqlx::migrate!("./migrations") // assumes "migrations/" dir exists
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}


#[tokio::test]
async fn test_sqlx_interacts_with_postgres_container() {
    let pool = setup_database().await;
    sqlx::query("SELECT 1")
    .execute(&pool)
    .await
    .expect("database not ready");
    // Insert a test user
    sqlx::query("INSERT INTO users (username) VALUES ($1)")
        .bind("alice")
        .execute(&pool)
        .await
        .expect("Failed to insert user");

    // Query it back
    let row: (String,) = sqlx::query_as("SELECT username FROM users WHERE username = $1")
        .bind("alice")
        .fetch_one(&pool)
        .await
        .expect("Failed to fetch user");

    assert_eq!(row.0, "alice");
}
