use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
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

pub type UserDB = Arc<Mutex<HashMap<u16, FormUsers>>>;
