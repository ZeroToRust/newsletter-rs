use std::error::Error;

use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new().route("/health-check", get(health_check));
    let address = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Server serving on http://localhost:8080/health-check");
    let _server = axum::serve(address, app).await?;
    Ok(())
}

async fn health_check() -> String {
    "Health Check Successful".into()
}
