use std::sync::{Arc};
use sqlx::PgPool;
// use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FormUsers {
    pub name: String,
    pub email: String,
}

impl FormUsers {
    pub fn new(name: &str, email: &str) -> Self {
        Self {
            name: name.to_string(),
            email: email.to_string(),
        }
    }
}

pub struct AppState{
    pub pool: PgPool,
}

pub type UserDB = Arc<AppState>;