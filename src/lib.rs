use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
pub mod authentication;
pub mod config;
pub mod helpers;
pub mod models;
pub mod session_state;
use tracing_actix_web::TracingLogger;
pub mod routes;

pub fn run(
    listener: TcpListener,
    connection: PgPool,
    config: config::ConfigJwt,
) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .configure(routes::handler::config)
            .app_data(web::Data::new(config::AppState {
                db: connection.clone(),
                config: config.clone(),
            }))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
