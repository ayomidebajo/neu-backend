// use actix_web::{Responder, HttpResponse};
use crate::helpers::pass_helpers::verify_password;
use crate::models::{LoginUser, TestStruct};
use actix_web::{web, Error, HttpResponse};
use jsonwebtoken::Algorithm;
use serde::{Deserialize, Serialize};

use sqlx::PgPool;

pub async fn login(req: web::Json<LoginUser>, connection: web::Data<PgPool>) -> HttpResponse {
    let user: Option<TestStruct> =
        sqlx::query_as::<_, TestStruct>("SELECT email, password FROM customers WHERE email = $1")
            .bind(req.email.to_string())
            .fetch_optional(connection.get_ref())
            .await
            .expect("Incorrect email");

    let matched = match user {
        Some(data) => Ok(data),
        None => Err("No data"),
    };

    if matched.clone().is_ok() {
        let is_valid = verify_password(&req.password, &matched.unwrap().password);

        if is_valid {
            HttpResponse::Ok().body("Logged in")
        } else {
            HttpResponse::Unauthorized().body("Incorrect password")
        }
    } else {
        HttpResponse::Unauthorized().body("Incorrect Email")
    }
}

// Define a struct to represent the user data
#[derive(Debug, Serialize, Deserialize)]
struct UserData {
    user_id: u32,
    email: String,
    // Other user data...
}

// Secret key used for JWT encoding and decoding
const SECRET_KEY: &[u8] = b"secret_key";

// Validate user credentials (example implementation)
fn validate_credentials(credentials: LoginUser) -> Result<UserData, Error> {
    // Perform authentication logic here (e.g., validate against a database)
    // If the credentials are valid, return the user data
    // Otherwise, return an error
    let user_id = 123456;
    let email = credentials.email;
    Ok(UserData { user_id, email })
}

// Handler for the sign-in route
pub async fn sign_in(
    credentials: web::Json<LoginUser>,
    connection: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    // Validate the user credentials (e.g., authenticate against a database)
    let credentials = LoginUser {
        email: credentials.email.clone(),
        password: credentials.password.clone(),
    };
    // If the user exists, validate the credentials
    let user: Option<LoginUser> =
        sqlx::query_as::<_, LoginUser>("SELECT email, password FROM customers WHERE email = $1")
            .bind(credentials.email.to_string())
            .fetch_optional(connection.get_ref())
            .await
            .expect("Incorrect email");

    // match user result and handles error gracefully
    match user.clone() {
        // If the email exists, validate password
        Some(data) => {
            // validates the password
            let is_valid = verify_password(&credentials.password, &user.unwrap().password);
            if is_valid {
                let valid_credentials = validate_credentials(data)?;
                let token = generate_token(valid_credentials)?;
                return Ok(actix_web::HttpResponse::Ok().json(token));
            } else {
                return Err(actix_web::error::ErrorUnauthorized("Incorrect password"));
            }
        }
        // If the user does not exist, return an error
        None => {
            return Err(actix_web::error::ErrorUnauthorized("Incorrect email"));
        }
    }
}

// Generate a JWT token for the authenticated user
fn generate_token(user_data: UserData) -> Result<String, Error> {
    // Generate a JWT token using the user data and the secret key
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::new(Algorithm::HS256),
        &user_data,
        &jsonwebtoken::EncodingKey::from_secret(SECRET_KEY),
    )
    .expect("Failed to generate token");

    Ok(token)
}
