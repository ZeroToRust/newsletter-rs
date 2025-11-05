use axum::{
    routing::{get, post},
    Router,
};

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
// use eyre::Result;
use tokio::net::TcpListener;

mod database;
mod handlers;

use handlers::health::health_check;
use handlers::subscriptions::*;

use database::data_base::*;

// use crate::handlers::subscriptions::*;

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
