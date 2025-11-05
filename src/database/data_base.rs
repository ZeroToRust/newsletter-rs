use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
// use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct FormUsers {
    pub user_name: String,
    pub user_email: String,
}

impl FormUsers {
    pub fn new(name: &str, email: &str) -> Self {
        Self {
            user_name: name.to_string(),
            user_email: email.to_string(),
        }
    }
}

pub type UserDB = Arc<Mutex<HashMap<u16, FormUsers>>>;
