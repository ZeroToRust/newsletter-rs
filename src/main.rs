use axum::{response::IntoResponse, routing::get, Router};
use eyre::Result;
use hyper::StatusCode;
use tokio::net::TcpListener;

/// Entry point for different services
#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/app/login", get(|| async {}))
        .route("/app", get(|| async {}))
        .route("/app/blog", get(|| async {}))
        .route("/app/blog/list", get(|| async {}))
        .route("/app/blog/create", get(|| async {}))
        .fallback(|| async { (StatusCode::BAD_REQUEST, "Page Not Found") });

    let address = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Server serving on {}", address.local_addr()?);
    axum::serve(address, app).await?;
    Ok(())
}

///# Health check message
async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "Health Check Successful")
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt; // for `oneshot` method

    #[tokio::test]
    async fn test_health_check() {
        // Create the app with the route
        let app = Router::new().route("/health_check", get(health_check));
        // This Simulate a GET request to the `/health-check` route like a client
        let request = Request::builder()
            .uri("/health_check")
            .method("GET")
            .body(Body::empty())
            .unwrap();
        // Seding a bad request
        let request2 = Request::builder()
            .uri("/app/check")
            .method("GET")
            .body(Body::empty())
            .unwrap();

        // simulate the process of sending the request to the app like when a client navigate to the path on the browser
        let response = app.clone().oneshot(request).await.unwrap();

        let response2 = app.oneshot(request2).await.unwrap();
        // Checking if the response was a successful one
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response2.status(), StatusCode::NOT_FOUND);
    }
}
