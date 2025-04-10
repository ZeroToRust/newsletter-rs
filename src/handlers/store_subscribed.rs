use serde::Deserialize;
use sqlx::PgPool;
use dotenvy::{self, dotenv, var};

#[derive(Deserialize)]
pub struct SubscriptionData {
    pub name: String,
    pub email: String,
}

pub async fn store_subscriber(data: &SubscriptionData) -> Result<(), sqlx::Error> {
    dotenv().ok();
    let durl = var("DATABASE_URL").unwrap();
    let pool = PgPool::connect(&durl).await.expect("Failed to connect to db");

    sqlx::query!(
        r##"
        INSERT INTO subscriptions (name, email)
        VALUES ($1, $2)
        "##,
        data.name,
        data.email
    )
    .execute(&pool)
    .await?;
    Ok(())
}
#[tokio::test]
async fn test_store_subscriber() {
    // Arrange: Create a test subscription data
    let data = SubscriptionData {
        name: "John Doe".to_string(),
        email: "johndoe@example.com".to_string(),
    };
    let data1 = SubscriptionData {
        name: "Peter Doe".to_string(),
        email: "johndoe@example.com".to_string(),
    };

    // Act: Call the store_subscriber function
    let result = store_subscriber(&data).await;
    // Assert: Ensure the function executed successfully
    assert!(result.is_ok());
    let result = store_subscriber(&data1).await;
    assert!(result.is_ok());
}