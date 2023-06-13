use actix_web::{App, HttpServer, dev::Server, web};
use std::net::TcpListener;
pub mod controller;
pub mod db;


pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(controller::health_check)))
        .listen(listener)?
        .run();
    Ok(server)
}