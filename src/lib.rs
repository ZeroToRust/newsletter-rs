use serde::Deserialize;
/// Returns a greeting message.
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