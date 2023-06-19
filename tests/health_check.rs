use neu_backend::config::get_configuration;
use neu_backend::config::DatabaseSettings;

use neu_backend::run;
use reqwest;
use sqlx::Executor;
use sqlx::{Connection, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub mod production_spawn_server_test {
    use super::*;

    #[allow(unused)]
    pub(crate) async fn spawn_app() -> TestApp {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
        let port = listener.local_addr().unwrap().port();
        let address = format!("http://127.0.0.1:{}", port);
        let configuration = get_configuration().expect("Failed to read configuration.");
        let connection_pool = PgPool::connect(&configuration.database.connection_string())
            .await
            .expect("Failed to connect to Postgres.");
        let server = run(listener, connection_pool.clone()).expect("Failed to bind address");
        let _ = tokio::spawn(server);
        dbg!("running in develop feature");
        TestApp {
            address,
            db_pool: connection_pool,
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
    let server = run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    println!("running in here in spawn dev");
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect(&config.connection_string())
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

// #[cfg(test)]
// #[cfg(feature = "dev")]
#[actix_rt::test]
async fn login_works() {
    // use dotenv::dotenv;
    use neu_backend::models;

    let app = spawn_app().await;
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();

    let _connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres");

    let client = reqwest::Client::new();

    let cus = models::Customer {
        fname: "John".to_string(),
        lname: "Doe".to_string(),
        email: "amanda@gmail.com".to_string(),
        password: "password".to_string(),
        phone_no: "08012345678".to_string(),
        is_merchant: false,
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
    assert!(response.status().is_success());
    dbg!(response.status().as_u16());
    assert_eq!(200, response.status().as_u16());

    let login_details = models::LoginUser {
        email: "ada@gmail.com".to_string(),
        password: "password".to_string(),
    };

    let json_body = serde_json::to_string(&login_details).unwrap();

    let response = client
        .post(&format!("{}/login", app.address))
        .header("Content-Type", "application/json")
        .body(json_body)
        .send()
        .await
        .expect("Failed to execute rewuest");

    //  assert!(response.status().is_success());
    dbg!(response.status().as_u16());
    assert_eq!(200, response.status().as_u16());
}

#[cfg(test)]
#[cfg(feature = "dev")]
#[actix_rt::test]
async fn sign_up_works_dev() {
    use dotenv::dotenv;
    use neu_backend::models;
    // ARRANGE
    let app = spawn_app().await;
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();

    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres");

    let client = reqwest::Client::new();
    let cus = models::Customer {
        fname: "John".to_string(),
        lname: "Doe".to_string(),
        email: "amanda@gmail.com".to_string(),
        password: "password".to_string(),
        phone_no: "08012345678".to_string(),
        is_merchant: false,
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
    assert!(response.status().is_success());
    dbg!(response.status().as_u16());
    assert_eq!(200, response.status().as_u16());

    // dotenv().ok();

    let saved = sqlx::query!("SELECT id, email FROM customers WHERE email = $1")
        .bind("ada@gmail.com".to_string())
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved customer.");
    dbg!("saved {:?}", saved);

    assert_eq!(1, 2);

    // assert_eq!(saved.email, "amanda@gmail.com");
}

#[cfg(test)]
#[cfg(feature = "prod")]
#[actix_rt::test]
async fn sign_up_works_prod() {
    use dotenv::dotenv;
    use neu_backend::models;
    // ARRANGE
    let app = production_spawn_server_test::spawn_app().await;
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();

    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres");

    let client = reqwest::Client::new();
    let cus = models::Customer {
        fname: "John".to_string(),
        lname: "Doe".to_string(),
        email: "ade@gmail.com".to_string(),
        password: "password".to_string(),
        phone_no: "08012345678".to_string(),
        is_merchant: false,
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
    assert!(response.status().is_success());
    dbg!(response.status().as_u16());
    assert_eq!(200, response.status().as_u16());

    dotenv().ok();

    let saved = sqlx::query!("SELECT email FROM customers")
        .fetch_all(&mut connection)
        .await
        .expect("Failed to fetch saved customer.");
    dbg!("{:?}", saved);
    assert_eq!(saved.email, "ade@gmail.com");
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
