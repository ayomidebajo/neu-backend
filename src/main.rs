pub mod authentication;
pub mod error;
pub mod helpers;
pub mod models;
pub mod routes;
use env_logger::Env;
use neu_backend::config::get_configuration;
use neu_backend::run;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
pub mod config;
pub mod session_state;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;

    let port = listener.local_addr().unwrap().port();

    let random_addr = format!("http://127.0.0.1:{}", port);

    println!("listening on {}", random_addr);

    let connection_pool = PgPoolOptions::new().max_connections(10).connect_lazy("postgres://postgres:password@0.0.0.0:5432/neudb").expect("connection error");

    run(listener, connection_pool, configuration.config)?.await.expect("eorrr");
    Ok(())
}
