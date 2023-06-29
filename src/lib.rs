use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
pub mod config;
pub mod db;
pub mod helpers;
pub mod models;
use tracing_actix_web::TracingLogger;
pub mod routes;

pub fn run(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route(
                "/health_check",
                web::get().to(routes::health_check::health_check),
            )
            .route("/home", web::get().to(routes::home_page::home_page))
            .route("/sign_up", web::post().to(routes::sign_up::sign_up))
            .route("/login", web::post().to(routes::login::sign_in))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
