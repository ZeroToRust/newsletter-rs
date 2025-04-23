use newsletter_rs::handlers::subscriptions::SubscribeRequest;
use tower::ServiceExt;

mod common;

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    // Arrange
    let app = common::spawn_app();
    let test_case = SubscribeRequest {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
    };

    // Act
    let response = app
        .oneshot(
            axum::http::Request::builder()
                .uri("/api/subscriptions")
                .method("POST")
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(format!(
                    "name={}&email={}",
                    test_case.name(),
                    test_case.email()
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert_eq!(response.status(), axum::http::StatusCode::OK);
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(&body[..], b"Subscription successful!");
}

#[tokio::test]
async fn subscribe_returns_400_for_missing_data() {
    // Arrange
    let app = common::spawn_app();

    // Act
    let response = app
        .oneshot(
            axum::http::Request::builder()
                .uri("/api/subscriptions")
                .method("POST")
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body("name=John%20Doe")
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert_eq!(response.status(), axum::http::StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn subscribe_returns_400_for_invalid_email() {
    // Arrange
    let app = common::spawn_app();

    // Act
    let response = app
        .oneshot(
            axum::http::Request::builder()
                .uri("/api/subscriptions")
                .method("POST")
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body("name=John%20Doe&email=invalid-email")
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert_eq!(response.status(), axum::http::StatusCode::BAD_REQUEST);
} 