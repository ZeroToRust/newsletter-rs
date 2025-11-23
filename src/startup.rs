use axum::{
    routing::{get, post},
    Router,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::net::TcpListener;

use crate::database::{FormUsers, UserDB};
use crate::handlers::health::health_check;
use crate::handlers::subscriptions::{get_subscribers, subscribe};

fn app_builder() -> Router {
    let default_user = HashMap::from([(1, FormUsers::new("MArthe", "marthe@gmail.com"))]);

    let db: UserDB = Arc::new(Mutex::new(default_user));
    Router::new()
        .route("/health_check", get(health_check))
        .route("/api/subscriptions", post(subscribe))
        .route("/api/subscriptions/{id}", get(get_subscribers))
        .with_state(db)
}

pub async fn serve_args() -> Result<(TcpListener, Router), std::io::Error> {
    let address = TcpListener::bind("127.0.0.1:0").await?;
    let app = app_builder();
    println!("Server serving on {}", address.local_addr().unwrap().port());
    Ok((address, app))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_app_builder() {
        let app = app_builder();

        // Test that the router is created successfully
        assert!(!format!("{:?}", app).is_empty());
    }

    #[tokio::test]
    async fn test_serve_args() {
        let result = serve_args().await;

        // Test that the function returns Ok
        assert!(result.is_ok());

        let (listener, app) = result.unwrap();

        // Test that we get a valid TcpListener
        let local_addr = listener.local_addr().unwrap();
        assert_eq!(local_addr.ip().to_string(), "127.0.0.1");
        assert!(local_addr.port() > 0);

        // Test that we get a valid Router
        assert!(!format!("{:?}", app).is_empty());
    }
}
