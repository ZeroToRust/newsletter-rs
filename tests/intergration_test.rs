use postgres::Postgres;
use testcontainers_modules::{postgres, testcontainers::runners::AsyncRunner};
use tokio_postgres::NoTls;


#[tokio::test]
async fn test_postgres_version() {
    let container = Postgres::default().start().await.unwrap();
    let host = container.get_host().await.unwrap();
    let port = container.get_host_port_ipv4(5432).await.unwrap();

    let conn_str = format!(
        "host={} port={} user=postgres password=postgres dbname=postgres",
        host, port
    );

    let (client, connection) = tokio_postgres::connect(&conn_str, NoTls).await.unwrap();
    tokio::spawn(connection);

    let row = client.query_one("SELECT version();", &[]).await.unwrap();
    let version: &str = row.get(0);
    println!("PostgreSQL version: {}", version);
    assert!(version.to_lowercase().contains("postgresql"));
}

#[tokio::test]
async fn test_postgres_connection() {
    let container = Postgres::default().start().await.unwrap();
    let host = container.get_host().await.unwrap();
    let port = container.get_host_port_ipv4(5432).await.unwrap();

    let conn_str = format!(
        "host={} port={} user=postgres password=postgres dbname=postgres",
        host, port
    );

    let (client, connection) = tokio_postgres::connect(&conn_str, NoTls).await.unwrap();
    tokio::spawn(connection);

    let row = client.query_one("SELECT 1 + 1", &[]).await.unwrap();
    let sum: i32 = row.get(0);
    assert_eq!(sum, 2);
}
