use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ =
        HttpServer::new(|| App::new().route("/health_check", web::get().to(|| HttpResponse::Ok())))
            .bind(("127.0.0.1", 8080))?
            .run()
            .await;

    Ok(())
}
