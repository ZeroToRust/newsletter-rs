use axum::{body::Body, http::{Request, StatusCode}};
use tower::ServiceExt;
use newsletter_rs::startup::build_app;

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let app = build_app();

    let req = Request::builder()
    .method("POST")
    .uri("/api/subscriptions")
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body(Body::from("name=Jane+Doe&email=jane%40example.com"))
    .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}