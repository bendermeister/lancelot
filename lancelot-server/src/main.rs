use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
         App::new()
             .route("/health_check", health_check)
    })
    .bind(("127.0.0.1", 8080))?
    .run().await?;

    Ok(())
}
