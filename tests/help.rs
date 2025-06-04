use testcontainers_modules::postgres::Postgres; // testcontainers::{runners::AsyncRunner, ImageExt}};
use testcontainers_modules::{
    testcontainers,
    testcontainers::{runners::AsyncRunner, ImageExt},
};
async fn start_postgres_container() -> (testcontainers::ContainerAsync<Postgres>, String) {
    let image = Postgres::default().with_tag("17.5"); // fixed tag
    let container = image.start().await.expect("failed to launch image");

    let port = container
        .get_host_port_ipv4(5432)
        .await
        .expect("failed to get port");
    let conn_str = format!("postgres://postgres:postgres@localhost:{}/postgres", port);

    (container, conn_str)
}
use sqlx::migrate;
use sqlx::{postgres::PgPoolOptions, PgPool};

async fn setup_database_pool(database_url: &str) -> PgPool {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to create pool");

    migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}
