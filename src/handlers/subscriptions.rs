use axum::{
    extract::Form,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use std::error::Error;

/// Represents the request payload for a subscription.
///
/// This struct is used to deserialize the form data sent by the client.
///
/// # Fields
/// - `name`: The name of the subscriber.
/// - `email`: The email address of the subscriber.
///
/// # Methods
/// - `name(&self) -> &str`: Returns the name of the subscriber.
/// - `email(&self) -> &str`: Returns the email address of the subscriber.
/// - `set_name(&mut self, name: String)`: Sets the name of the subscriber.
/// - `set_email(&mut self, email: String)`: Sets the email address of the subscriber.
#[derive(Debug, Deserialize)]
pub struct SubscribeRequest {
    /// The name of the subscriber.
    name: String,

    /// The email address of the subscriber.
    email: String,
}

impl SubscribeRequest {
    pub fn new(name: String, email: String) -> Self {
        Self { name, email }
    }

    /// Returns the name of the subscriber.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the email address of the subscriber.
    pub fn email(&self) -> &str {
        &self.email
    }

    /// Sets the name of the subscriber.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Sets the email address of the subscriber.
    pub fn set_email(&mut self, email: String) {
        self.email = email;
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

/// Handles the subscription request.
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
pub async fn subscribe(Form(payload): Form<SubscribeRequest>) -> Response {
    if let Err(error) = payload.validate() {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            format!("Validation error: {}", error),
        )
            .into_response();
    }

    let subscription_info = format!(
        "New subscription:\nName: {}\nEmail: {}",
        payload.name(),
        payload.email()
    );
    println!("{}", subscription_info);

    (StatusCode::OK, "Subscription successful!").into_response()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subscribe_request_getters() {
        let request = SubscribeRequest::new("John Doe".to_string(), "john@example.com".to_string());

        assert_eq!(request.name(), "John Doe");
        assert_eq!(request.email(), "john@example.com");
    }

    #[test]
    fn test_subscribe_request_setters() {
        let mut request =
            SubscribeRequest::new("John Doe".to_string(), "john@example.com".to_string());

        request.set_name("Jane Doe".to_string());
        request.set_email("jane@example.com".to_string());

        assert_eq!(request.name(), "Jane Doe");
        assert_eq!(request.email(), "jane@example.com");
    }

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
