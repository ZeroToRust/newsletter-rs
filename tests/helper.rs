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
        let container = image.start().await.unwrap();

        let port = container.get_host_port_ipv4(5432).await.unwrap();
        let conn_str = format!("postgres://postgres:postgres@localhost:{}/postgres", port);

        Self {
            connection_string: conn_str,
            _container: container,
        }
    }
}
