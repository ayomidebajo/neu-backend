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
            // .route(
            //     "/health_check",
            //     web::get().to(routes::health_check::health_check),
            // )
            // .route("/home", web::get().to(routes::home_page::home_page))
            // .route("/sign_up", web::post().to(routes::sign_up::sign_up))
            // .route("/login", web::post().to(routes::login::sign_in))
            // .route("/me", web::get().to(routes::login::get_me_handler))
            .app_data(web::Data::new(config::AppState {
                db: connection.clone(),
                config: config.clone(),
            }))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
