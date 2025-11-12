use axum::http::StatusCode;
use newsletter_rs::startup::serve_args;

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

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let (listener, addr) = serve_args().await.unwrap();

    let add = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, addr).await.unwrap();
    });

    // let form_data = [("email", "rolland@gmail.com"),("name", "rolland")];
    let form_data = "email=rolland%40gmail.com&name=rolland";
    let client = reqwest::Client::new();
    let response = client
        .post(&format!("http://{}/api/subscriptions", add))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(form_data)
        .send()
        .await
        .unwrap();

    assert_eq!(201, response.status());
    // println!("status: {:?}", response);
    // let text: FormUsers = response.json().await.unwrap();
    // dbg!(text);
    // println!("status: {:?}", text);
}
