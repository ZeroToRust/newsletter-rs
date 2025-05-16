use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use eyre::Result;
use http_body_util::BodyExt;
use newsletter_rs::handlers::subscriptions::SubscribeRequest;

use tower::ServiceExt;
mod common;

#[tokio::test]
#[ignore = "Needs database for storing user info. And testcontainer database are not yet ready"]
async fn subscribe_returns_200_for_valid_form_data_and_stores_user() -> Result<()> {
    // Arrange
    let app = common::spawn_app().await;
    let test_case = SubscribeRequest::new("John Doe".to_string(), "john1@example.com".to_string());

    // Act
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/subscriptions")
                .method("POST")
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(Body::from(format!(
                    "name={}&email={}",
                    test_case.name, test_case.email
                )))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert: Ensure the response status is 200
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("New subscription:"));
    assert!(body_str.contains("John Doe"));
    assert!(body_str.contains("john1@example.com"));

    Ok(())
}

#[tokio::test]
#[ignore = "Requires database access"]
async fn subscribe_returns_422_for_missing_data() -> Result<()> {
    // Arrange
    let app = common::spawn_app().await;

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

    Ok(())
}

#[tokio::test]
#[ignore = "Requires database access"]
async fn subscribe_returns_422_for_invalid_email() -> Result<()> {
    // Arrange
    let app = common::spawn_app().await;

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

    Ok(())
}

#[tokio::test]
#[ignore = "Requires database access"]
async fn subscribe_returns_400_for_invalid_form_data() -> Result<()> {
    // Arrange
    let app = common::spawn_app().await;

    // Act
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/subscriptions")
                .method("POST")
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(Body::from("invalid-form-data"))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    Ok(())
}
