use axum::{http::StatusCode, response::IntoResponse};

///# Health check message
pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
