use serde::Deserialize;

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
#[derive(Deserialize)]
pub struct SubscribeRequest {
    /// The name of the subscriber.
    name: String,

    /// The email address of the subscriber.
    email: String,
}

impl SubscribeRequest {
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

use axum::{extract::Form, response::IntoResponse};
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
pub async fn subscribe(Form(payload): Form<SubscribeRequest>) -> impl IntoResponse {
    let subscription_info = format!(
        "New subscription:\nName: {}\nEmail: {}",
        payload.name(),
        payload.email()
    );
    println!("{}", subscription_info);

    "Subscription successful!".to_string()
}
