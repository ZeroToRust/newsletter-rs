use axum::{
    extract::{Form, Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::database::{FormUsers, UserDB};

pub async fn subscribe(
    State(db): State<UserDB>,
    Form(payload): Form<FormUsers>,
) -> impl IntoResponse {
    let mut users = db.lock().unwrap();
    let id = users.keys().max().unwrap_or(&0) + 1;

    users.insert(id, payload);

    // (StatusCode::CREATED, Json(payload))
    StatusCode::CREATED
}

pub async fn get_subscribers(
    State(db): State<UserDB>,
     Path(id): Path<u16>) -> impl IntoResponse 
     {
    let users = db.lock().unwrap();
    let user = users.get(&id).unwrap();
    (StatusCode::OK, Json(user.clone()))
}
