use actix_web::{ get, HttpResponse, HttpServer, Responder, App};

#[get("/")]
// ここの関数名とサービスの関数名は合わせる必要がある
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(index))
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
