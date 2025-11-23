use axum::{http::StatusCode, response::IntoResponse};

///# Health check message
pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

#[cfg(test)]
mod health_test {

    use super::health_check;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::get,
        Router,
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_check() {
        let app = Router::new().route("/health", get(health_check));
        let req = Request::builder()
            .uri("/health")
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
