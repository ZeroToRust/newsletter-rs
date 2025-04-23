use axum::{
    extract::Form,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use validator::Validate;

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
#[derive(Debug, Deserialize, Validate)]
pub struct SubscribeRequest {
    /// The name of the subscriber.
    #[validate(length(min = 1, message = "Name is required"))]
    name: String,

    /// The email address of the subscriber.
    #[validate(email(message = "Invalid email format"))]
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
}

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
/// use crate::SubscribeRequest;
///
/// let form = Form(SubscribeRequest {
///     name: "Jane Doe".to_string(),
///     email: "jane.doe@example.com".to_string(),
/// });
/// let response = subscribe(form).await;
/// assert_eq!(response, "Subscription successful!".to_string());
/// ```
pub async fn subscribe(Form(payload): Form<SubscribeRequest>) -> Response {
    if let Err(errors) = payload.validate() {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            format!("Validation error: {}", errors),
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
}
