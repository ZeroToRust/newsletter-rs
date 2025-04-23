use axum::{body::Body, http::Request, Router};
use http_body_util::BodyExt;
use newsletter_rs::startup::build_app;
use tower::ServiceExt;

pub fn spawn_app() -> Router {
    build_app()
}

pub async fn _send_subscription_request(
    app: &Router,
    name: &str,
    email: &str,
) -> reqwest::Response {
    let form_data = format!("name={}&email={}", name, email);

    // Use the app parameter to make the request
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/subscriptions")
                .method("POST")
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(Body::from(form_data))
                .unwrap(),
        )
        .await
        .unwrap();

    // Convert the response to reqwest::Response
    let body = response.into_body().collect().await.unwrap().to_bytes();

    // Create a new reqwest client and make the request
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:3000/api/subscriptions")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();

    response
}
