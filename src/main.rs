use newsletter_rs::startup::build_app;

#[tokio::main]
async fn main() {
    // Build the application
    let app = build_app();

    // Run the application
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}