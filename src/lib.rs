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
// use actix_session::{ Session};

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

// create a table token_cache
// add rows (user_id, token_string)
// add expiring date
// create a cron job that will be updating this table every minute
// add a helper function that fires when a user logout or when the token is expired.