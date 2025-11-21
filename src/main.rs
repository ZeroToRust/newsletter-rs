use eyre::Result;
use newsletter_rs::build_app;

/// Entry point for different services
#[tokio::main]
async fn main() -> Result<()> {
    let (app, address) = build_app().await?;
    println!("Server serving on {}", address.local_addr()?);
    axum::serve(address, app).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_build_app_creates_router_and_listener() {
        let result = build_app().await;
        assert!(result.is_ok());

        let (_, listener) = result.unwrap();
        assert!(listener.local_addr().is_ok());
    }
}
