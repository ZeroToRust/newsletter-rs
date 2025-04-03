use std::error::Error;

use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new().route("/health_check", get(health_check));
    let address = TcpListener::bind("0.0.0.0:8080").await?;
   println!("Server serving on {}", address.local_addr()?);
    let _server = axum::serve(address, app).await?;
    Ok(())
}
///# Health check message
async fn health_check() -> String {
    "Health Check Successful".into()
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
        let app = Router::new().route("/health-check", get(health_check));

        // This Simulate a GET request to the `/health-check` route like a client
        let request = Request::builder()
            .uri("/health-check")
            .method("GET")
            .body(Body::default())
            .unwrap();
        // simulate the process of sending the request to the app like when a client navigate to the path on the browser
        let response = app.oneshot(request).await.unwrap();

        // Checking if the response was a successful one
        assert_eq!(response.status(), StatusCode::OK);
    }
}
