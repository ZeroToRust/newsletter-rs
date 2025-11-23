use reqwest;
// use axum::http::StatusCode;

#[tokio::main]
async fn main() {
    let port = 51067;
    let form_data = vec![
        "email=rolland%40gmail.com&name=rolland",
        "name=raol%20del&email=raol.del%40gmail.com",
    ];

    let mut response = Vec::new();
    let client = reqwest::Client::new();
    for user in form_data.iter() {
        let resp = client
            .post(&format!("http://127.0.0.1:{}/api/subscriptions", port))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(*user)
            .send()
            .await
            .unwrap();
        response.push((resp.status(), "0".to_string()));
    }

    for user_id in 1..=form_data.len() as u16 {
        // let user_id = 1u16;
        let resp = client
            .get(&format!(
                "http://127.0.0.1:{port}/api/subscriptions/{user_id}"
            ))
            .send()
            .await
            .unwrap();
        response.push((resp.status(), resp.text().await.unwrap()));
    }

    println!("status: {:#?}", response);
}
