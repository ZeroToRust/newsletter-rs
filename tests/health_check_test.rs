use axum::http::StatusCode;
use newsletter_rs::serve_args;

#[tokio::test]
async fn health_check_test() {
    let (address, app) = serve_args().await.unwrap();

    let add = address.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(address, app).await.unwrap();
    });

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("http://{}/health_check", add))
        .send()
        .await
        .unwrap();
    let status = response.status();

    assert_eq!(status, StatusCode::OK);
    assert_eq!(Some(0), response.content_length());
}
