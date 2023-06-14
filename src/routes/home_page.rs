use actix_web::{HttpResponse, Responder};

pub async fn home_page() -> impl Responder {
    HttpResponse::Ok().body("This is the home page")
}
