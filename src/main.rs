// use eyre::Result;

use newsletter_rs::serve_args;

/// Entry point for different services
#[tokio::main]
async fn main() -> std::io::Result<()> {

    let(address, app) = serve_args().await?;
    axum::serve(address, app).await.unwrap();

    Ok(())
}
