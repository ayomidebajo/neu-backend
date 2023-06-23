pub mod db;
pub mod error;
pub mod helpers;
pub mod models;
pub mod routes;
use env_logger::Env;
use neu_backend::config::get_configuration;
use neu_backend::run;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = TcpListener::bind("127.0.0.1:0")?;
    let port = address.local_addr().unwrap().port();

    let random_addr = format!("http://127.0.0.1:{}", port);
    println!("listening on {}", random_addr);

    let postgres_conn = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    run(address, postgres_conn)?.await
}
