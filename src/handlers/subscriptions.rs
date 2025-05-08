use super::config::get_database_pool;
use axum::{
    extract::Form,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use regex::Regex;
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

        if !validate_email(self.email.clone()) {
            return Err(ValidationError::InvalidEmail);
        }

        Ok(())
    }
}
///This struct is to add a subscribed user to the database database
#[derive(Debug, sqlx::FromRow)]
pub struct SubscriptionRecord {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub subscribed_at: DateTime<Utc>,
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
            subscribed_at: Utc::now(),
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
        "New subscription:\nID: {}\nName: {}\nEmail: {}\nSubcribed_At: {}",
        subscription_record.id,
        subscription_record.name,
        subscription_record.email,
        subscription_record.subscribed_at
    );
    (StatusCode::OK, "Subscription successful!").into_response()
}

/// Adds the subscribed user into the database
pub async fn store_subscriber(data: &SubscriptionRecord) -> Result<(), sqlx::Error> {
    let pool = get_database_pool().await;
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, name, email, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        data.id,
        data.name,
        data.email,
        data.subscribed_at
    )
    .execute(&pool)
    .await?;
    Ok(())
}

lazy_static! {
    // RFC 5322 compliant regex with some practical constraints
    // Regex is a complex to compute so static allows us to increase performance by compiling it only once and share it across different threads for email validation.
    static ref EMAIL_REGEX: Regex = Regex::new(r"^(?i)[a-z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:\.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)*$").unwrap();
}

pub fn validate_email(email: String) -> bool {
    // Check length constraints
    if email.is_empty() || email.len() > 254 {
        return false;
    }

    // Check basic format with regex
    if !EMAIL_REGEX.is_match(email.as_str()) {
        return false;
    }

    // Additional checks
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }

    let local_part = parts[0];
    let domain_part = parts[1];

    // Local part length check
    if local_part.len() > 64 {
        return false;
    }

    // Domain part checks
    if domain_part.len() > 253 {
        return false;
    }

    // Check for consecutive dots
    if local_part.contains("..") || domain_part.contains("..") {
        return false;
    }

    // Check for valid TLD (at least 2 chars)
    let domain_parts: Vec<&str> = domain_part.split('.').collect();
    if domain_parts.len() < 2 {
        return false;
    }

    let tld = domain_parts.last().unwrap();
    if tld.len() < 2 {
        return false;
    }

    // Check for valid domain parts
    for part in domain_parts {
        if part.is_empty() || part.len() > 63 {
            return false;
        }
        if part.starts_with('-') || part.ends_with('-') {
            return false;
        }
    }

    true
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
        assert!(
            matches!(empty_name.validate().err(), Some(e) if e.to_string() == "Name is required")
        );

        let invalid_email =
            SubscribeRequest::new("John Doe".to_string(), "invalid-email".to_string());
        assert!(
            matches!(invalid_email.validate().err(), Some(e) if e.to_string() == "Invalid email format")
        );

        let empty_email = SubscribeRequest::new("John Doe".to_string(), "".to_string());
        assert!(
            matches!(empty_email.validate().err(), Some(e) if e.to_string() == "Invalid email format")
        );
    }
    // To see if the user has been added with success
    #[tokio::test]
    #[ignore = "needs to access database and testcontainers are not yet available"]
    async fn test_subscribe_success() {
        let form = Form(SubscribeRequest::new(
            "John Doe".to_string(),
            "johnda11@example.com".to_string(),
        ));

        let response = subscribe(form).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    fn test_invalid_emails() {
        let invalid_emails = vec![
            "plainaddress".to_string(),            // Missing @
            "@no-local-part.com".to_string(),      // Missing local part
            "no-at-sign.net".to_string(),          // Missing @
            "email..email@domain.com".to_string(), // Consecutive dots
            "email@domain..com".to_string(),       // Consecutive dots in domain
            "email@-domain.com".to_string(),       // Leading hyphen in domain
            "email@domain-.com".to_string(),       // Trailing hyphen in domain
            "a@b.c".to_string(),                   // TLD too short
            "email@[123.123.123.123".to_string(),  // Unclosed IP literal
            "@domain.com".to_string(),             // Empty local part
            "test@.com".to_string(),               // Empty domain segment
            "あいうえお@domain.com".to_string(),   // Unicode is valid
        ];

        let valid_emails = vec![
            "user@domain.com".to_string(),
            "user.name@domain.com".to_string(),
            "user+tag@domain.com".to_string(),
            "user@subdomain.domain.com".to_string(),
            "email.address@domain.com".to_string(),
            "firstname.lastname@domain.com".to_string(),
            "email@subdomain.domain.com".to_string(),
            "firstname+lastname@domain.com".to_string(),
            "1234567890@domain.com".to_string(),
        ];

        // Test invalid emails
        for email in &invalid_emails {
            assert!(
                !validate_email(email.clone()),
                "Email should be invalid but was accepted: {email}"
            );
        }

        // Test valid emails
        for email in &valid_emails {
            assert!(
                validate_email(email.clone()),
                "Email should be valid but was rejected: {email}"
            );
        }
    }
}
