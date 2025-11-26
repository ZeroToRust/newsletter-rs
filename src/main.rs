// use eyre::Result;

use newsletter_rs::startup::serve_builder;
use newsletter_rs::configuration::get_configuration;

/// Entry point for different services
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let setting = get_configuration().expect("Faild to load configuration");
    let (address, app) = serve_builder(setting).await?;
    axum::serve(address, app).await.unwrap();

    Ok(())
}
