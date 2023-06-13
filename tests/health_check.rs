use neu_backend::models;
use neu_backend::run;
use reqwest;
use std::net::TcpListener;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    // We return the application address to the caller!
    format!("http://127.0.0.1:{}", port)
}

#[actix_rt::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        // Use the returned application address
        .get(&format!("{}/health_check", &address))
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
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        // Use the returned application address
        .get(&format!("{}/home", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(response.content_length().unwrap() > 0, true);
}

#[actix_rt::test]
async fn sign_up_works() {
    // ARRANGE
    let address = spawn_app();
    let client = reqwest::Client::new();
    let cus = models::Customer {
        fname: "John".to_string(),
        lname: "Doe".to_string(),
        email: "johndoe@gmail.com".to_string(),
        password: "password".to_string(),
        phone_no: "08012345678".to_string(),
        is_merchant: false,
        is_verified_user: false,
    };

    let json_body = serde_json::to_string(&cus).unwrap();

    // ACT
    let response = client
        .post(&format!("{}/sign_up", address))
        .header("Content-Type", "application/json")
        .body(json_body)
        .send()
        .await
        .expect("Failed to execute rewuest");

    // Assert
    assert!(response.status().is_success());

    assert_eq!(200, response.status().as_u16());
}

#[actix_rt::test]
async fn sign_up_fails_when_data_is_missing() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    let cus = "{
		\"fname\": \"John\",
	}";

    let json_body = serde_json::to_string(&cus).unwrap();

    let response = client
        .post(&format!("{}/sign_up", address))
        .header("Content-Type", "application/json")
        .body(json_body)
        .send()
        .await
        .expect("Failed to execute rewuest");

    assert_eq!(400, response.status().as_u16());
}
