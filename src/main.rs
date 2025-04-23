use axum::serve;
use newsletter_rs::startup::build_app;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Build the application
    let app = build_app();

    // Run the application
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Server Running on http://0.0.0.0:8080");
    serve(listener, app).await.unwrap();
}
