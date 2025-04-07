#[cfg(test)]
mod tests {
    use axum::{body::Body, http::{Request, StatusCode}, response::IntoResponse, routing::get, Router};
    use tower::ServiceExt; // for provide oneshot method that is used to send client request to the server

    ///# Health check message
async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

    #[tokio::test]
    async fn test_health_check() {
        // Create the app with the route
        let app = Router::new().route("/health_check", get(health_check));
        // This Simulate a GET request to the `/health_check` route like a client
        let request = Request::builder()
            .uri("/health_check")
            .method("GET")
            .body(Body::empty())
            .unwrap();
        // Sending a bad request
        let request2 = Request::builder()
            .uri("/app/check")
            .method("GET")
            .body(Body::empty())
            .unwrap();
        let response2 = app.clone().oneshot(request2).await.unwrap();

        // simulate the process of sending the request to the app like when a client navigate to the path on the browser
        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response2.status(), StatusCode::NOT_FOUND);
    }
}

