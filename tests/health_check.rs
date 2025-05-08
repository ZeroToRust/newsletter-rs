use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

mod common;

#[tokio::test]
async fn health_check_works() {
    // Arrange: Spawn the application
    let app = common::spawn_app();

    // Act: Send a GET request to the /health_check endpoint
    let response = app
        .oneshot(
            Request::builder()
                .uri("/health_check")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert: Ensure the response status is 200 OK
    assert_eq!(response.status(), StatusCode::OK);
}
