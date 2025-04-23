use axum::Router;
use newsletter_rs::startup::build_app;
use tower::ServiceExt;

pub fn spawn_app() -> Router {
    build_app()
}

pub async fn send_subscription_request(
    app: &Router,
    name: &str,
    email: &str,
) -> reqwest::Response {
    let client = reqwest::Client::new();
    let form_data = format!("name={}&email={}", name, email);

    client
        .post("http://localhost:3000/api/subscriptions")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(form_data)
        .send()
        .await
        .expect("Failed to execute request")
} 