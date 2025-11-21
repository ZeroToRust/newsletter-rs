use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use tokio::net::TcpListener;
///# Health check message
pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

pub async fn build_app() -> Result<(Router, TcpListener), std::io::Error> {
    let app = Router::new().route("/health_check", get(health_check));
    let address = TcpListener::bind("127.0.0.1:0").await?;
    Ok((app, address))
}

#[tokio::test]
async fn test_health_check_returns_ok_status() {
    let status = health_check().await.into_response().status();
    assert_eq!(status, StatusCode::OK);
}
