
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn response() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ =
        HttpServer::new(|| App::new().route("/health_check", web::get().to(response)))
            .bind(("127.0.0.1", 8080))?
            .run()
            .await;

    Ok(())
