use actix_web::{get, App, HttpResponse, HttpServer};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(hello)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
