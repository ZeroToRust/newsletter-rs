use axum::{
    body::Body,
    http::Request,
    Router,
};
use http_body_util::BodyExt;
use newsletter_rs::startup::build_app;
use tower::ServiceExt;

pub fn spawn_app() -> Router {
    build_app()
}
