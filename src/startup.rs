use crate::handlers::{health_check::health_check, subscriptions::subscribe};
use axum::{
    routing::{get, post},
    Router,
};

pub fn build_app() -> Router {
    Router::new()
        .route("/api/subscriptions", post(subscribe))
        .route("/health_check", get(health_check))
}
