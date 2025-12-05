use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;
use time::OffsetDateTime;

use crate::database::{FormUsers, UserDB};

pub async fn subscribe(
    State(db): State<UserDB>,
    Form(payload): Form<FormUsers>,
) -> impl IntoResponse {

    let result = sqlx::query(
        "INSERT INTO subscriptions (id, email, name, subscribed_at)
         VALUES ($1, $2, $3, $4)"
    )
    .bind(Uuid::new_v4())
    .bind(&payload.email)
    .bind(&payload.name)
    .bind(OffsetDateTime::now_utc())
    .execute(&db.pool)
    .await;

    match result {
        Ok(_) => StatusCode::CREATED,
        Err(e) => {
            eprintln!("Database error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

// pub async fn get_subscribers(State(db): State<UserDB>, Path(id): Path<u16>) -> impl IntoResponse {
//     let users = db.lock().unwrap();
//     let user = users.get(&id).unwrap();
//     (StatusCode::OK, Json(user.clone()))
// }

// #[cfg(test)]s
// mod tests {
//     use super::*;
//     use crate::database::FormUsers;
//     use axum::extract::{Form, Path, State};
//     use std::collections::HashMap;
//     use std::sync::{Arc, Mutex};

//     fn create_test_db() -> UserDB {
//         Arc::new(Mutex::new(HashMap::new()))
//     }

//     fn create_test_user() -> FormUsers {
//         FormUsers {
//             name: "Test User".to_string(),
//             email: "test@example.com".to_string(),
//         }
//     }

//     #[tokio::test]
//     async fn test_subscribe_creates_user() {
//         let db = create_test_db();
//         let user = create_test_user();

//         let response = subscribe(State(db.clone()), Form(user.clone())).await;

//         // Check that the response is CREATED
//         let status = response.into_response().status();
//         assert_eq!(status, StatusCode::CREATED);

//         // Check that user was added to database
//         let users = db.lock().unwrap();
//         assert_eq!(users.len(), 1);
//         assert!(users.contains_key(&1));
//     }

//     #[tokio::test]
//     async fn test_subscribe_increments_id() {
//         let db = create_test_db();
//         let user1 = create_test_user();
//         let user2 = FormUsers {
//             name: "Test User 2".to_string(),
//             email: "test2@example.com".to_string(),
//         };

//         // Add first user
//         subscribe(State(db.clone()), Form(user1)).await;

//         // Add second user
//         subscribe(State(db.clone()), Form(user2)).await;

//         let users = db.lock().unwrap();
//         assert_eq!(users.len(), 2);
//         assert!(users.contains_key(&1));
//         assert!(users.contains_key(&2));
//     }

//     #[tokio::test]
//     async fn test_get_subscribers_returns_user() {
//         let db = create_test_db();
//         let user = create_test_user();

//         // First add a user
//         subscribe(State(db.clone()), Form(user.clone())).await;

//         // Then retrieve it
//         let response = get_subscribers(State(db), Path(1)).await;
//         let (status, _) = response.into_response().into_parts();

//         assert_eq!(status.status, StatusCode::OK);
//     }

//     #[tokio::test]
//     #[should_panic]
//     async fn test_get_subscribers_panics_on_nonexistent_id() {
//         let db = create_test_db();

//         // Try to get a user that doesn't exist
//         get_subscribers(State(db), Path(999)).await;
//     }
// }
