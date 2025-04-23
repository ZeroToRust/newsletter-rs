use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use newsletter_rs::handlers::subscriptions::SubscribeRequest;
use tower::ServiceExt;

mod common;

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    // Arrange
    let app = common::spawn_app();
    let test_case = SubscribeRequest::new("John Doe".to_string(), "john@example.com".to_string());

    // Act
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/subscriptions")
                .method("POST")
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(Body::from(format!(
                    "name={}&email={}",
                    test_case.name(),
                    test_case.email()
                )))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"Subscription successful!");
}

#[tokio::test]
async fn subscribe_returns_422_for_missing_data() {
    // Arrange
    let app = common::spawn_app();

    // Act
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/subscriptions")
                .method("POST")
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(Body::from("name=John%20Doe"))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn subscribe_returns_422_for_invalid_email() {
    // Arrange
    let app = common::spawn_app();

    // Act
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/subscriptions")
                .method("POST")
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(Body::from("name=John%20Doe&email=invalid-email"))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}
