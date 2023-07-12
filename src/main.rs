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
pub mod jwt_auth;
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

    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());

    run(listener, connection_pool, configuration.config)?.await
}
