use crate::config::AppState;
use crate::helpers::pass_helpers::verify_password;
use crate::jwt_auth;
use crate::models::{GetUser, LoginUser, TestStruct, TokenClaims};
use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    web, Error, HttpMessage, HttpRequest, HttpResponse, Responder,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use std::fmt::Display;

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

// Handler for the sign-in route
pub async fn sign_in(
    credentials: web::Json<LoginUser>,
    connection: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let user: Option<GetUser> =
        sqlx::query_as::<_, GetUser>("SELECT * FROM customers WHERE email = $1")
            .bind(credentials.email.to_string())
            .fetch_optional(&connection.db)
            .await
            .expect("Incorrect email");

    // match user result and handles error gracefully
    match user.clone() {
        // If the email exists, validate password
        Some(user_data) => {
            // validates the password
            let is_valid = verify_password(&credentials.password, &user.unwrap().password);
            if is_valid {
                let now = Utc::now();
                let iat = now.timestamp() as usize;
                let exp = (now + Duration::minutes(60)).timestamp() as usize;
                let claims: TokenClaims = TokenClaims {
                    sub: user_data.id.to_string(),
                    exp,
                    iat,
                };

                let token = encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(connection.config.jwt_secret.as_ref()),
                )
                .unwrap();

                let cookie = Cookie::build("token", token.to_owned())
                    .path("/")
                    .max_age(ActixWebDuration::new(60 * 60, 0))
                    .http_only(true)
                    .finish();
                Ok(HttpResponse::Ok()
                    .cookie(cookie)
                    .json(json!({"status": "success", "token": token})))
            } else {
                Err(actix_web::error::ErrorUnauthorized("Incorrect password"))
            }
        }
        // If the user does not exist, return an error
        None => Err(actix_web::error::ErrorUnauthorized("Incorrect email")),
    }
}

// #[derive(thiserror::Error)]
#[derive(Debug)]
pub enum LoginError {
    AuthError,
    UnexpectedError,
}

impl Display for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", LoginError::AuthError)
    }
}

#[allow(clippy::await_holding_refcell_ref)]
pub async fn get_me_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    _: jwt_auth::JwtMiddleware,
) -> impl Responder {
    let ext = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();

    // // drop(ext);
    // let user_id = user_id

    let user: Option<GetUser> =
        sqlx::query_as::<_, GetUser>("SELECT * FROM customers WHERE id = $1")
            .bind(user_id)
            .fetch_optional(&data.db)
            .await
            .unwrap();

    let json_response = serde_json::json!({
        "status":  "success",
        "data": serde_json::json!({
            "user": user.unwrap()
        })
    });

    HttpResponse::Ok().json(json_response)
}
