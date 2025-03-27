#![cfg(test)]
mod tests {
    use actix_web::{test, web, App, HttpResponse, Responder};
    #[actix_web::test]
    async fn health_check_test_get() {
        //creates and instance of a service app
        let app =
            test::init_service(App::new().route("/health-check", web::get().to(response))).await;
        // This creates a request instance
        let request = test::TestRequest::get().uri("/health-check").to_request();

        let response = test::call_service(&app, request).await;

        assert!(response.status().is_success())
    }

    async fn response() -> impl Responder {
        HttpResponse::Ok()
    }

    #[test]
    #[should_panic]
    async fn failing_health_check_get() {
        //creates and instance of a service app
        let app =
            test::init_service(App::new().route("/health-check", web::get().to(response))).await;
        // This creates a request default instance that will request a failure status
        let request = test::TestRequest::default().to_request();

        let response = test::call_service(&app, request).await;

        assert!(response.status().is_success())
    }
}
