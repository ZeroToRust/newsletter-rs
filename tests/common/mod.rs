use axum::Router;
use newsletter_rs::{handlers::config::AppState, startup::build_app};

pub async fn spawn_app() -> Router {
    let state = AppState::new().await;
    build_app(state)
}
