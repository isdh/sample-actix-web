// pub mod hoge {
//     // 中に書く必要がある
    use actix_web::{get, HttpResponse, Responder};

    #[get("/")]
    // ここの関数名とサービスの関数名は合わせる必要がある
    pub async fn index() -> impl Responder {
        HttpResponse::Ok().body("Hello world!")
    }
// }
