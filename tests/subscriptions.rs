use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use newsletter_rs::handlers::subscriptions::SubscribeRequest;
use sqlx::{PgPool, Row};
use tower::ServiceExt;
use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    GenericImage, ImageExt,
};


mod common;

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data_and_stores_user() {
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

    // Assert: Ensure the response status is 200
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"Subscription successful!");

    // Assert: Check if the user is stored in the database
    let pool: PgPool = common::get_database_pool().await;
    let row = sqlx::query("SELECT name, email FROM subscriptions WHERE email = $1")
        .bind(test_case.email())
        .fetch_one(&pool)
        .await
        .expect("Failed to fetch user from database");

    let stored_name: String = row.get("name");
    let stored_email: String = row.get("email");

    assert_eq!(stored_name, test_case.name());
    assert_eq!(stored_email, test_case.email());
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

#[tokio::test]
async fn database_connection_is_successful() {
    // Arrange
    let pool: PgPool = common::get_database_pool().await;

    // Act: Attempt to query the database
    let result = sqlx::query("SELECT 1").fetch_one(&pool).await;

    // Assert: Ensure the query succeeds
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_postgres() {
    let _container = GenericImage::new("postgres", "latest")
        .with_exposed_port(5432.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
        .with_network("bridge")
        .with_env_var("DEBUG", "1")
        .start()
        .await
        .expect("Failed to start Postgres container");
}