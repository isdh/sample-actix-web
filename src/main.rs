mod hoge;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(hoge::index))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
