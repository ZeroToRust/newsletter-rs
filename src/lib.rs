use axum::{routing::get, Router};
// use eyre::Result;
use tokio::net::TcpListener;

mod handlers;
use handlers::health::health_check;

pub async fn serve_args() -> Result<(TcpListener, Router), std::io::Error> {
    let address = TcpListener::bind("127.0.0.1:0").await?;
    let app = Router::new().route("/health_check", get(health_check));
    println!("Server serving on {}", address.local_addr().unwrap().port());
    Ok((address, app))
}
