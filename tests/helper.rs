
use testcontainers_modules::{
    postgres::Postgres,
    testcontainers::{self, runners::AsyncRunner, ImageExt},
};

pub struct TestDb {
    pub connection_string: String,
    _container: testcontainers::ContainerAsync<Postgres>,
}

impl TestDb {
    pub async fn new() -> Self {
        let image = Postgres::default().with_tag("17.5"); // fixed tag
        let container = image.start().await.expect("failed to launch image");

        let port = container
            .get_host_port_ipv4(5432)
            .await
            .expect("failed to get port");
        let conn_str = format!("postgres://postgres:postgres@localhost:{}/postgres", port);

        Self {
            connection_string: conn_str,
            _container: container,
        }
    }
}



#[tokio::test]
async fn test_postgres_query() {
    let test_db = TestDb::new().await;
    let conn_str = &test_db.connection_string;

    let (client, connection) = tokio_postgres::connect(conn_str, tokio_postgres::NoTls)
        .await
        .expect("Failed to connect to Postgres");

    tokio::spawn(connection);

    let rows = client.query("SELECT 42", &[]).await.expect("Query failed");
    assert_eq!(rows[0].get::<_, i32>(0), 42);
}
