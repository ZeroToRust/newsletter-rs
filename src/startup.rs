use crate::handlers::subscriptions::subscribe;
use axum::{routing::post, Router};

pub fn build_app() -> Router {
    Router::new().route("/api/subscriptions", post(subscribe))
}
