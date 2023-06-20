use actix_web::{HttpResponse, Responder};
use tracing;
use uuid::Uuid;

pub async fn home_page() -> impl Responder {
    let request_id = Uuid::new_v4();
    tracing::info!("request_id {} - rendering home page", request_id,);
    HttpResponse::Ok().body("This is the home page")
}
