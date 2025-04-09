use serde::Deserialize;
/// Returns a greeting message.
use sqlx::PgPool;
use 

#[derive(Deserialize)]
struct SubscriptionData {
    name: String,
    email: String,
}

async fn store_subscriber(data: &SubscriptionData) -> Result<(), sqlx::Error> {
    let durl = str::env::var("DATABASE_URL");
    let pool = PgPool::connect(&durl).await?;

    sqlx::query!(
        r##"
        INSERT INTO subscriptions (name, email)
        VALUES ($1, $2)
        "##,
        data.name,
        data.email
    )
    .execute(pool)
    .await?;
    Ok(())
}