use axum::{
    routing::{get, post},
    Router,
};
use std::{
    sync::Arc,
};
use tokio::net::TcpListener;
use sqlx::PgPool;
use crate::{Settings, database::{UserDB, data_base::AppState}};
use crate::handlers::health::health_check;
use crate::handlers::subscriptions::{subscribe};

fn app_builder(db_pool: PgPool) -> Router {
     
    let db: UserDB = Arc::new(AppState{pool: db_pool});
    Router::new()
    .route("/health_check", get(health_check))
    .route("/api/subscriptions", post(subscribe))
        // .route("/api/subscriptions/{id}", get(get_subscribers))
        .with_state(db)
}
    
pub async fn serve_builder(settings: Settings) -> Result<(TcpListener, Router), std::io::Error> {
    let port = settings.app_port;
    let address = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;

    let url = settings.database.get_database_url();
    let db_pool = PgPool::connect(&url)
        .await
        .expect("Failed to connect to Postgres.");

    let app = app_builder(db_pool);
    println!("Server serving on {}", address.local_addr().unwrap().port());
    Ok((address, app))
}

#[cfg(test)]
mod test {
    use super::*;
    #[tokio::test]
    async fn test_app_builder() {
        // Arrange
        let settings = Settings {app_port: 0, ..Default::default()};
        let url = settings.database.get_database_url();
        let db_pool = PgPool::connect(&url)
            .await
            .expect("Failed to connect to Postgres.");
        let app = app_builder(db_pool);

        // Test that the router is created successfully
        assert!(!format!("{:?}", app).is_empty());
    }

    #[tokio::test]
    async fn test_serve_builder() {
        let settings = Settings {app_port: 0, ..Default::default()};
        let result = serve_builder(settings).await;

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
