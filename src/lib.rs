use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;
pub mod controller;
pub mod db;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new().route("/health_check", web::get().to(controller::health_check)).route("/home", web::get().to(controller::home_page))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
