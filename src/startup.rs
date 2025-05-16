use crate::handlers::{config::AppState, health_check::health_check, subscriptions::subscribe};
use axum::{
    routing::{get, post},
    Router,
};

pub fn build_app(state: AppState) -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/api/subscriptions", post(subscribe))
        .with_state(state)
}
