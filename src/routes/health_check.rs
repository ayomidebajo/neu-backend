use actix_web::{HttpResponse, Responder};

pub async fn health_check() -> impl Responder {
    tracing::info!("testing health check");
    HttpResponse::Ok().finish()
}
