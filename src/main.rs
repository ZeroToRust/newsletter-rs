use axum::serve;
use eyre::Result;
use newsletter_rs::{handlers::config::AppState, startup::build_app};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    // Build the application
    let state = AppState::new().await;
    let app = build_app(state);

    // Run the application
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Server Running on http://0.0.0.0:8080");
    serve(listener, app).await?;

    Ok(())
}
