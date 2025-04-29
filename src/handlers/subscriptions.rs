use super::config::get_database_pool;
use axum::{
    extract::Form,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Deserialize;
use std::error::Error;
use uuid::Uuid;

#[derive(Debug)]
pub enum ValidationError {
    NameRequired,
    InvalidEmail,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::NameRequired => write!(f, "Name is required"),
            ValidationError::InvalidEmail => write!(f, "Invalid email format"),
        }
    }
}

impl Error for ValidationError {}

/// Represents the request payload for a subscription.
///
/// This struct is used to deserialize the form data sent by the client.
///
/// # Fields
/// - `name`: The name of the subscriber.
/// - `email`: The email address of the subscriber.

#[derive(Debug, Deserialize)]
pub struct SubscribeRequest {
    pub name: String,
    pub email: String,
}

impl SubscribeRequest {
    pub fn new(name: String, email: String) -> Self {
        Self { name, email }
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.name.trim().is_empty() {
            return Err(ValidationError::NameRequired);
        }

        if !self.email.contains('@') || !self.email.contains('.') {
            return Err(ValidationError::InvalidEmail);
        }

        Ok(())
    }
}
///This struct is to add a subscribed user to the database database
// #[derive(Debug, sqlx::FromRow)]
pub struct SubscriptionRecord {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub subscribed_at: NaiveDateTime,
}

impl SubscriptionRecord {
    pub fn new(userdata: SubscribeRequest) -> Result<Self, ValidationError> {
        let _ = match userdata.validate() {
            Ok(_) => {}
            Err(err) => {
                println!("Error validating User: {err}");
                return Err(err);
            }
        };

        Ok(Self {
            id: Uuid::new_v4(),
            name: userdata.name,
            email: userdata.email,
            subscribed_at: Utc::now().naive_utc(),
        })
    }
}

/// Creates a subscription  request.
///
/// This function processes a POST request with a form containing the subscriber's name and email.
/// It prints the subscriber's information to the console and returns a response confirming the subscription.
///
/// # Arguments
/// - `Form(payload)`: The form data extracted from the request, deserialized into a `SubscribeRequest`.
///
/// # Returns
/// - `impl IntoResponse`: A string response confirming the subscription.
///
/// # Examples
/// ```rust
/// use axum::Form;
/// use newsletter_rs::handlers::subscriptions::{SubscribeRequest, subscribe};
///
/// #[tokio::main]
/// async fn main() {
///     let form = Form(SubscribeRequest::new(
///         "Jane Doe".to_string(),
///         "jane.doe@example.com".to_string(),
///     ));
///     let response = subscribe(form).await;
///     assert_eq!(response.status(), axum::http::StatusCode::OK);
/// }
/// ```

pub async fn subscribe(Form(userdata): Form<SubscribeRequest>) -> Response {
    if let Err(validation_err) = userdata.validate() {
        println!("Validation Error: {}", validation_err);
        return (StatusCode::UNPROCESSABLE_ENTITY, validation_err.to_string()).into_response();
    }
    // Converting the subscriptionRequest to SubscriptionRecord
    let subscription_record = match SubscriptionRecord::new(userdata) {
        Ok(record) => record,
        Err(validation_error) => {
            println!("Validation Error: {}", validation_error);
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                validation_error.to_string(),
            )
                .into_response();
        }
    };
    // Storing the user's data into the database
    if let Err(db_error) = store_subscriber(&subscription_record).await {
        println!("Database error: {}", db_error);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to store subscription".to_string(),
        )
            .into_response();
    }
    println!(
        "New subscription:\nName: {}\nEmail: {}",
        subscription_record.name, subscription_record.email
    );
    (StatusCode::OK, "Subscription successful!").into_response()
}

/// Adds the subscribed user into the database
pub async fn store_subscriber(data: &SubscriptionRecord) -> Result<(), sqlx::Error> {
    let pool = get_database_pool().await;
    sqlx::query!(
        r##"
        INSERT INTO subscriptions (id, name, email, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "##,
        data.id,
        data.name,
        data.email,
        data.subscribed_at
    )
    .execute(&pool)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_validation() {
        let valid_request =
            SubscribeRequest::new("John Doe".to_string(), "john@example.com".to_string());
        assert!(valid_request.validate().is_ok());

        let empty_name = SubscribeRequest::new("".to_string(), "john@example.com".to_string());
        assert!(matches!(
            empty_name.validate(),
            Err(ValidationError::NameRequired)
        ));

        let invalid_email =
            SubscribeRequest::new("John Doe".to_string(), "invalid-email".to_string());
        assert!(matches!(
            invalid_email.validate(),
            Err(ValidationError::InvalidEmail)
        ));
    }
}

#[tokio::test]
async fn test_subscribe_success() {
    let form = Form(SubscribeRequest::new(
        "John Doe".to_string(),
        "john.doe@example.com".to_string(),
    ));

    let response = subscribe(form).await;
    assert_eq!(response.status(), StatusCode::OK);
}
