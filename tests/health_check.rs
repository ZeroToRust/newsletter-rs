use axum::{http::StatusCode, routing::get, Router};
use tokio::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = Router::new().route("/health_check", get(health_check));
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    
    // Start the server in a background task
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    // Act
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("http://{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn health_check() -> impl axum::response::IntoResponse {
    StatusCode::OK
}