use newsletter_rs::{store_subscriber, SubscriptionData};

#[tokio::test]
async fn test_store_subscriber() {
    // Arrange: Create a test subscription data
    let data = SubscriptionData {
        name: "John Doe".to_string(),
        email: "john.doe@example.com".to_string(),
    };

    // Act: Call the store_subscriber function
    let result = store_subscriber(&data).await;

    // Assert: Ensure the function executed successfully
    assert!(result.is_ok());
}