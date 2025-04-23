use axum::Router;
use newsletter_rs::startup::build_app;

pub fn spawn_app() -> Router {
    build_app()
}
