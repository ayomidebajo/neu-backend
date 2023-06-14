use actix_web::{Responder, HttpResponse};


pub async fn home_page() -> impl Responder {
	HttpResponse::Ok().body("This is the home page")
}