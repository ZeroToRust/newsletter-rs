use axum::http::StatusCode;
use newsletter_rs::build_app;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let client = reqwest::Client::new();
        let (app, listener) = build_app().await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        let response = client
            .get(format!("http://{addr}/health_check"))
            .send()
            .await
            .unwrap();
        assert_eq!(200, response.status().as_u16());

        let response = client
            .get(format!("http://{addr}/wrong_endpoint"))
            .send()
            .await
            .unwrap();

        assert_ne!(response.status(), StatusCode::OK);
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
