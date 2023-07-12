use neu_backend::config::get_configuration;
use neu_backend::config::DatabaseSettings;

use neu_backend::run;
use reqwest;
use sqlx::Executor;
use sqlx::{Connection, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub mod production_spawn_server_test {
    use sqlx::postgres::PgPoolOptions;

    use super::*;

    #[allow(unused)]
    pub(crate) async fn spawn_app() -> TestApp {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
        let port = listener.local_addr().unwrap().port();
        let address = format!("http://127.0.0.1:{}", port);
        let configuration = get_configuration().expect("Failed to read configuration.");
        let connection_pool = PgPoolOptions::new()
            .connect_timeout(std::time::Duration::from_secs(2))
            .connect_lazy_with(configuration.database.with_db());
        let connect_copy = connection_pool.clone();
        let server = run(listener, connect_copy.clone(), configuration.config)
            .expect("Failed to bind address");
        let _ = tokio::spawn(server);
        dbg!("running in prod feature");
        TestApp {
            address,
            db_pool: connect_copy.clone(),
        }
    }
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database).await;
    let server = run(listener, connection_pool.clone(), configuration.config)
        .expect("Failed to bind address");
    let _ = tokio::spawn(server);
    println!("running in here in spawn dev");
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
#[actix_rt::test]
async fn health_check_works() {
    // Arrange

    let app = spawn_app().await;
    let client = reqwest::Client::new();
    // Act
    let response = client
        // Use the returned application address
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn home_page_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        // Use the returned application address
        .get(&format!("{}/home", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(response.content_length().unwrap() > 0, true);
}

#[cfg(test)]
#[cfg(feature = "prod")]
#[actix_rt::test]
async fn sign_up_works_prod() {
    // use dotenv::dotenv;
    use neu_backend::models::{self, LoginUser};
    // ARRANGE
    let app = production_spawn_server_test::spawn_app().await;
    let client = reqwest::Client::new();

    let cus = models::Customer {
        fname: "John".to_string(),
        lname: "Doe".to_string(),
        email: "ade@gmail.com".to_string(),
        password: "password".to_string(),
        phone_no: "08012345678".to_string(),
        is_verified_user: false,
    };

    let json_body = serde_json::to_string(&cus).unwrap();

    // ACT
    let response = client
        .post(&format!("{}/sign_up", app.address))
        .header("Content-Type", "application/json")
        .body(json_body)
        .send()
        .await
        .expect("Failed to execute rewuest");

    // Assert
    // assert!(response.status().is_success());
    //   println!("nawa {:#?}", response.text_with_charset("utf-8"));
    assert_eq!(200, response.status().as_u16());

    // dotenv().ok();

    let saved =
        sqlx::query_as::<_, LoginUser>("SELECT email, password FROM customers WHERE email = $1")
            .bind(cus.email.to_string())
            .fetch_optional(&app.db_pool)
            .await
            .expect("Failed to fetch saved customer.");
    dbg!("saved {:?}", saved.clone());
    assert_eq!(saved.expect("No emailo").email, "ade@gmail.com");
}

#[actix_rt::test]
async fn sign_up_fails_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let cus = "{
		\"fname\": \"John\",
	}";

    let json_body = serde_json::to_string(&cus).unwrap();

    let response = client
        .post(&format!("{}/sign_up", app.address))
        .header("Content-Type", "application/json")
        .body(json_body)
        .send()
        .await
        .expect("Failed to execute rewuest");

    assert_eq!(400, response.status().as_u16());
}
