use super::config::AppState;
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use validator::{Validate, ValidationErrors};

/// This is a custorm error
#[derive(Debug)]
pub enum SubscriptionError {
    ValidationError(ValidationErrors),
    DatabaseError(sqlx::Error),
    EmailExists(String),
    DatabaseConnectionError(String),
}

//  this implemetation is to distinguish the errors that can occur
// during subscription of the user
impl IntoResponse for SubscriptionError {
    fn into_response(self) -> Response {
        match self {
            Self::ValidationError(err) => {
                (StatusCode::UNPROCESSABLE_ENTITY, err.to_string()).into_response()
            }
            Self::DatabaseError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "A database error occurred.".to_string(),
            )
                .into_response(),
            Self::EmailExists(email) => (
                StatusCode::CONFLICT,
                format!("Email {} is already registered", email),
            )
                .into_response(),
            Self::DatabaseConnectionError(err) => (
                StatusCode::SERVICE_UNAVAILABLE,
                format!("Database connection error: {}", err),
            )
                .into_response(),
        }
    }
}

/// Represents the request payload for a subscription.
///
/// This struct is used to deserialize the form data sent by the client.
///
/// # Fields
/// - `name`: The name of the subscriber.
/// - `email`: The email address of the subscriber.

#[derive(Debug, Deserialize, Validate)]
pub struct SubscribeRequest {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(email)]
    pub email: String,
}

impl SubscribeRequest {
    pub fn new(name: String, email: String) -> Self {
        Self { name, email }
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
    pub fn new(userdata: SubscribeRequest) -> Result<Self, ValidationErrors> {
        userdata.validate()?;

        Ok(Self {
            id: Uuid::new_v4(),
            name: userdata.name,
            email: userdata.email,
            subscribed_at: Utc::now(),
        })
    }
}

///This function is to check if the email the user is inputing already exists in the database and return appropriate error message to the user
async fn email_exists(email: &str, pool: &Pool<Postgres>) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        SELECT EXISTS(SELECT 1 FROM subscriptions WHERE email = $1)
        "#,
        email
    )
    .fetch_one(pool)
    .await?;

    Ok(result.exists.unwrap_or(false))
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

pub async fn subscribe(
    State(state): State<AppState>,
    Form(userdata): Form<SubscribeRequest>,
) -> Result<impl IntoResponse, SubscriptionError> {
    // Create subscription record
    let subscription_record =
        SubscriptionRecord::new(userdata).map_err(SubscriptionError::ValidationError)?;

    // Store in database
    store_subscriber(&subscription_record, &state.db).await?;

    // Format success response
    let subscribe_info = format!(
        "New subscription:\nID: {}\nName: {}\nEmail: {}\nSubcribed_At: {}",
        subscription_record.id,
        subscription_record.name,
        subscription_record.email,
        subscription_record.subscribed_at
    );
    println!("{subscribe_info}\n"); // logging successful subscription and sending reply to user
    Ok((StatusCode::OK, subscribe_info))
}

/// Adds the subscribed user into the database
async fn store_subscriber(
    data: &SubscriptionRecord,
    pool: &Pool<Postgres>,
) -> Result<(), SubscriptionError> {
    // Check database connection
    if let Err(e) = pool.acquire().await {
        return Err(SubscriptionError::DatabaseConnectionError(e.to_string()));
    }
    // Check if email exists
    match email_exists(&data.email, &pool).await {
        Ok(true) => return Err(SubscriptionError::EmailExists(data.email.clone())),
        Ok(false) => (), // proceed with insertion
        Err(e) => return Err(SubscriptionError::DatabaseError(e)),
    }
    // Then insert the new  subscription
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
    .execute(pool)
    .await
    .map_err(SubscriptionError::DatabaseError)?;
    Ok(())
}

lazy_static! {
    // RFC 5322 compliant regex with some practical constraints
    // Regex is a complex to compute so static allows us to increase performance by compiling it only once and share it across different threads for email validation.
    static ref EMAIL_REGEX: Regex = Regex::new(r"^(?i)[a-z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:\.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)*$").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::{dotenv, var};
    use sqlx::postgres::{PgPool, PgPoolOptions};

    // Helper function to create a test database pool
    async fn setup_test_db() -> PgPool {
        dotenv().ok();
        let database_url = var("DATABASE_URL").expect("DATABASE_URL must be set for tests");

        PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .expect("Failed to create test database pool")
    }

    // Helper function to create test app state
    async fn setup_test_state() -> AppState {
        AppState {
            db: setup_test_db().await,
        }
    }

    #[test]
    fn test_email_validation() {
        // Invalid email addresses
        let invalid_emails = vec![
            "",                          // Empty string
            "plainaddress",              // Missing @ and domain
            "@missinglocal.org",         // Missing local part
            "two@@domain.com",           // Multiple @ symbols
            "invalid@domain@domain.com", // Multiple @ symbols
            "invalid.com",               // Missing @
            "invalid@domain.com.",       // Trailing dot in domain
            "invalid@.com",              // Leading dot in domain
            "invalid@domain..com",       // Multiple dots in domain
            "invalid@domain@.com",       // Invalid domain format
            "spaces in@domain.com",      // Spaces in local part
            "invalid@dom ain.com",       // Spaces in domain
        ];

        // Valid email addresses
        let valid_emails = vec![
            "email@domain.com",              // Basic valid email
            "firstname.lastname@domain.com", // With dot
            "email@subdomain.domain.com",    // With subdomain
            "firstname+lastname@domain.com", // With plus
            "email@123.123.123.123",         // IP format domain
            "1234567890@domain.com",         // Numeric local part
            "email@domain-one.com",          // Domain with hyphen
            "_______@domain.com",            // Underscores
            "email@domain.name",             // Generic TLD
            "email@domain.co.jp",            // Country code TLD
            "firstname-lastname@domain.com", // With hyphen
            "email@domain.web",              // Modern TLD
        ];

        for email in invalid_emails {
            let request = SubscribeRequest::new("Test Name".to_string(), email.to_string());
            let result = SubscriptionRecord::new(request);
            assert!(
                result.is_err(),
                "Email '{}' should be invalid but was accepted",
                email
            );
        }

        for email in valid_emails {
            let request = SubscribeRequest::new("Test Name".to_string(), email.to_string());
            let result = SubscriptionRecord::new(request);
            assert!(
                result.is_ok(),
                "Email '{}' should be valid but was rejected",
                email
            );
        }
    }

    #[tokio::test]
    #[ignore = "Email already exists in the database"]
    async fn test_subscribe_success() -> Result<(), Box<dyn std::error::Error>> {
        let state = setup_test_state().await;
        let form = Form(SubscribeRequest::new(
            "John Doe".to_string(),
            "johdoe@example.com".to_string(),
        ));

        let response = subscribe(State(state), form)
            .await
            .expect("Failed to subscribe user");

        assert_eq!(
            response.into_response().status(),
            StatusCode::OK,
            "Expected successful subscription"
        );
        Ok(())
    }

    #[tokio::test]
    #[ignore = "Email already exists in the database"]
    async fn test_duplicate_email() -> Result<(), Box<dyn std::error::Error>> {
        let state = setup_test_state().await;
        let email = "test@example.com";

        // First subscription
        let form1 = Form(SubscribeRequest::new(
            "Test User 1".to_string(),
            email.to_string(),
        ));
        subscribe(State(state.clone()), form1)
            .await
            .expect("Failed to subscribe user");

        // Second subscription with same email
        let form2 = Form(SubscribeRequest::new(
            "Test User 2".to_string(),
            email.to_string(),
        ));

        let result = subscribe(State(state), form2).await;

        match result {
            Err(SubscriptionError::EmailExists(e)) => {
                assert_eq!(e, email, "Expected email '{}' in error", email);
                Ok(())
            }
            other => panic!(
                "Expected EmailExists error, got {:?}",
                other.unwrap().into_response()
            ),
        }
    }

    #[tokio::test]
    #[ignore = "Database connection error"]
    async fn test_database_connection_error() -> Result<(), Box<dyn std::error::Error>> {
        // Create state with invalid database connection
        let state = AppState {
            db: PgPool::connect("postgresql://invalid:5432/nonexistent")
                .await
                .expect("Should fail to connect"),
        };

        let form = Form(SubscribeRequest::new(
            "Test User".to_string(),
            "test@example.com".to_string(),
        ));

        let result = subscribe(State(state), form).await;

        match result {
            Err(SubscriptionError::DatabaseConnectionError(_)) => Ok(()),
            other => panic!(
                "Expected DatabaseConnectionError, got {:?}",
                other.unwrap().into_response()
            ),
        }
    }

    #[tokio::test]
    async fn test_validation_error() -> Result<(), Box<dyn std::error::Error>> {
        let state = setup_test_state().await;
        let form = Form(SubscribeRequest::new(
            "".to_string(), // Empty name should fail validation
            "valid@email.com".to_string(),
        ));

        let result = subscribe(State(state), form).await;

        match result {
            Err(SubscriptionError::ValidationError(_)) => Ok(()),
            other => panic!(
                "Expected ValidationError, got {:?}",
                other.unwrap().into_response()
            ),
        }
    }
}
