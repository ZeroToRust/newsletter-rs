use axum::{response::IntoResponse, http::StatusCode};


///# Health check message
pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}