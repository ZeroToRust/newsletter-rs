use super::{db_util::get_database_pool, subscriptions::SubscribeRequest};

pub async fn store_subscriber(data: &SubscribeRequest) -> Result<(), sqlx::Error> {
    let pool = get_database_pool().await;
    sqlx::query!(
        r##"
        INSERT INTO subscriptions (name, email)
        VALUES ($1, $2)
        "##,
        data.name(),
        data.email()
    )
    .execute(&pool)
    .await?;
    Ok(())
}

#[tokio::test]
async fn test_store_subscriber() {
    // Arrange: Create a test subscription data
    let data = SubscribeRequest::new("John Doe".to_string(), "john@example.com".to_string()); // Creates a new SubscribeRequest with name "John Doe" and email "john@example.com", email)
    let data1 = SubscribeRequest::new("Peter Doe".to_string(), "peter@example.com".to_string());

    // Act: Call the store_subscriber function
    let result = store_subscriber(&data).await;
    // Assert: Ensure the function executed successfully
    assert!(result.is_ok());
    let result = store_subscriber(&data1).await;
    assert!(result.is_ok());
}
