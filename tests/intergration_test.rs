mod helper;
use helper::TestDb;

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
