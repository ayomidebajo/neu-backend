use crate::authentication::jwt_auth;
use crate::config::AppState;
use crate::helpers::parser::user_parser;
use crate::helpers::pass_helpers::hash_password;
use crate::helpers::pass_helpers::verify_password;
use crate::models::{
    Customer, FilteredMerchant, FilteredUser, GetCustomer, GetUser, LoginUser, Merchant,
    TokenClaims, UpdateCustomer,
};
use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    get, post, web, Error, HttpResponse, Responder,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;
use std::fmt::Display;
use uuid::Uuid;

// Handler for the sign-in route
#[post("/user/login")]
pub async fn sign_in(
    credentials: web::Json<LoginUser>,
    connection: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let user: Option<GetUser> =
        sqlx::query_as::<_, GetUser>("SELECT id, email, password FROM customers WHERE email = $1")
            .bind(credentials.email.to_string().clone())
            .fetch_optional(&connection.db)
            .await
            .expect("error");

    let returned_user: Option<FilteredUser> = sqlx::query_as::<_, FilteredUser>("SELECT id, email, fname, lname, email, is_verified, created_at FROM customers WHERE email = $1").bind(credentials.email.to_string()).fetch_optional(&connection.db).await.expect("error getting user");

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
                    .json(json!({"status": "success", "status_code": "200", "token": token, "message": returned_user})))
            } else {
                Err(actix_web::error::ErrorUnauthorized("Incorrect password"))
            }
        }
        // If the user does not exist, return an error
        None => Err(actix_web::error::ErrorUnauthorized("User doesn't exist")),
    }
}

// Handler for registering a user
#[post("/user/register")]
pub async fn sign_up(req: web::Json<Customer>, connection: web::Data<AppState>) -> HttpResponse {
    let hashed_password = match hash_password(&req.password) {
        Ok(hashed) => Some(hashed),
        Err(err) => {
            println!("Error hashing password: {err}");
            None
        }
    };
    tracing::info!("Creating a new user account!");

    let request_id = Uuid::new_v4();
    tracing::info!(
        "request_id {} - Adding '{}' '{}' as a new customer.",
        request_id,
        req.email,
        req.fname
    );

    // validate user fields
    let req = Customer::parse_validate(req.into_inner());

    // match result
    match req {
        // execute queries if user values are okay
        Ok(req) => {
            // check if email exists
            let email_exists =
                sqlx::query!(r#"SELECT email FROM customers WHERE email = $1"#, req.email)
                    .fetch_optional(&connection.db)
                    .await;
            println!("email doesn't exist we move to the next code");

            if let Ok(email) = email_exists {
                if email.is_some() {
                    tracing::info!(
                        "request_id {} - Email '{:?}' already exists",
                        request_id,
                        email
                    );
                    println!("Email already exists {email:?}");
                    return actix_web::HttpResponse::Conflict().json("Email already exists");
                }
            }

            tracing::info!(
                "request_id {} - Saving new subscriber details in the database",
                request_id
            );
            // save user into database

            let created_at = Utc::now();
            let query = "
        INSERT INTO customers (id, email, fname, lname, password, is_verified, is_subscribed, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)";
            let id = Uuid::new_v4();
            let returned_user = FilteredUser {
                id,
                email: req.email.clone(),
                fname: req.fname.clone(),
                lname: req.lname.clone(),
                is_verified: req.is_verified_user,
                created_at,
            };

            match sqlx::query(query)
                .bind(id)
                .bind(req.email)
                .bind(req.fname)
                .bind(req.lname)
                .bind(hashed_password)
                .bind(req.is_verified_user)
                .bind(req.is_subscribed)
                .bind(created_at)
                .execute(&connection.db)
                .await
            {
                Ok(_) => {
                    tracing::info!(
                        "request_id {} - New customer details have been saved",
                        request_id
                    );
                    actix_web::HttpResponse::Ok().json(
                        json!({"status": "success", "status_code": "201", "message": returned_user}),
                    )
                }
                Err(e) => {
                    tracing::error!(
                        "request_id {} - Failed to execute query: {:?}",
                        request_id,
                        e
                    );
                    actix_web::HttpResponse::InternalServerError().body(e.to_string())
                }
            }
        }
        // if user details are invalid return response instead of
        Err(e) => {
            tracing::warn!("request_id {} - Failed parse: {:?}", request_id, e);
            actix_web::HttpResponse::BadRequest().json(e.to_string())
        }
    }
}

#[post("/merchant/register")]
pub async fn merchant_sign_up(
    req: web::Json<Merchant>,
    connection: web::Data<AppState>,
) -> HttpResponse {
    let hashed_password = match hash_password(&req.password) {
        Ok(hashed) => Some(hashed),
        Err(err) => {
            println!("Error hashing password: {err}");
            None
        }
    };
    tracing::info!("Creating a new user account!");

    let request_id = Uuid::new_v4();
    tracing::info!(
        "request_id {} - Adding '{}' '{}' as a new customer.",
        request_id,
        req.email,
        req.fname
    );

    // validate user fields
    let req = Merchant::parse_validate(req.into_inner());

    // match result
    match req {
        // execute queries if user values are okay
        Ok(req) => {
            // check if email exists
            let email_exists: Option<GetUser> = sqlx::query_as::<_, GetUser>(
                "SELECT id, email, password FROM merchants WHERE email = $1",
            )
            .bind(req.email.to_string())
            .fetch_optional(&connection.db)
            .await
            .expect("error");

            println!("email doesn't exist we move to the next code");

            if let Some(email) = email_exists {
                tracing::info!(
                    "request_id {} - Email '{:?}' already exists",
                    request_id,
                    email
                );
                println!("Email already exists {email:?}");
                return actix_web::HttpResponse::Conflict().json("Email already exists");
            }

            tracing::info!(
                "request_id {} - Saving new subscriber details in the database",
                request_id
            );
            // save user into database

            let created_at = Utc::now();
            let query = "
        INSERT INTO merchants (id, email, fname, lname, password, business_name, is_active, is_verified, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)";
            let id = Uuid::new_v4();
            let returned_merchant = FilteredMerchant {
                id,
                fname: req.fname.clone(),
                lname: req.lname.clone(),
                email: req.email.clone(),
                is_verified: req.is_verified,
                created_at,
                business_name: req.business_name.clone(),
            };

            match sqlx::query(query)
                .bind(id)
                .bind(req.email)
                .bind(req.fname)
                .bind(req.lname)
                .bind(hashed_password)
                .bind(req.business_name)
                .bind(req.is_active)
                .bind(req.is_verified)
                .bind(created_at)
                .execute(&connection.db)
                .await
            {
                Ok(_) => {
                    tracing::info!(
                        "request_id {} - New Merchant details have been saved",
                        request_id
                    );
                    actix_web::HttpResponse::Ok().json(
                        json!({"status": "success", "status_code": "201", "message": returned_merchant}),
                    )
                }
                Err(e) => {
                    tracing::error!(
                        "request_id {} - Failed to execute query: {:?}",
                        request_id,
                        e
                    );
                    actix_web::HttpResponse::InternalServerError().body(e.to_string())
                }
            }
        }
        // if user details are invalid return response instead of
        Err(e) => {
            tracing::warn!("request_id {} - Failed parse: {:?}", request_id, e);
            actix_web::HttpResponse::BadRequest().json(e.to_string())
        }
    }
}

// Handler for the sign-in route
#[post("/merchant/login")]
pub async fn merchant_sign_in(
    credentials: web::Json<LoginUser>,
    connection: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let user: Option<GetUser> =
        sqlx::query_as::<_, GetUser>("SELECT id, email, password FROM merchants WHERE email = $1")
            .bind(credentials.email.to_string().clone())
            .fetch_optional(&connection.db)
            .await
            .expect("error");
    let returned_user: Option<FilteredMerchant> = sqlx::query_as::<_, FilteredMerchant>("SELECT id, email, fname, lname, email, is_verified, business_name, created_at FROM merchants WHERE email = $1").bind(credentials.email.to_string().clone()).fetch_optional(&connection.db).await.expect("error getting returned user");

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
                    .json(json!({"status": "success", "status_code": "200", "token": token, "message": returned_user })))
            } else {
                Err(actix_web::error::ErrorUnauthorized("Incorrect password"))
            }
        }
        // If the user does not exist, return an error
        None => Err(actix_web::error::ErrorUnauthorized("User doesn't exist")),
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

// #[allow(clippy::await_holding_refcell_ref)]
#[get("/user/me")]
pub async fn get_user(data: web::Data<AppState>, jwt: jwt_auth::JwtMiddleware) -> impl Responder {
    // let ext = req.extensions();
    let user_id = jwt.user_id;

    // // drop(ext);
    // let user_id = user_id

    let user: Option<GetCustomer> =
        sqlx::query_as::<_, GetCustomer>("SELECT * FROM customers WHERE id = $1")
            .bind(user_id)
            .fetch_optional(&data.db)
            .await
            .expect("Incorrect user id");

    let filtered_user = user_parser(user.expect("user not found"));

    let json_response = serde_json::json!({
        "status":  "success",
        "message": serde_json::json!({
            "user": filtered_user
        }),
        "status_code": "200"
    });

    HttpResponse::Ok().json(json_response)
}

#[post("/user/update")]
pub async fn update_user(
    req: web::Json<UpdateCustomer>,
    data: web::Data<AppState>,
    jwt: jwt_auth::JwtMiddleware,
) -> impl Responder {
    let user_id = jwt.user_id;
    let user: Option<GetUser> =
        sqlx::query_as::<_, GetUser>("SELECT * FROM customers WHERE id = $1")
            .bind(user_id)
            .fetch_optional(&data.db)
            .await
            .expect("Incorrect user id");

    let _user = user.unwrap();
    match sqlx::query("UPDATE customers SET fname = $1, lname = $2, phone_no = $3 WHERE id = $4")
        .bind(req.fname.clone())
        .bind(req.lname.clone())
        .bind(req.phone_no.clone())
        .bind(user_id)
        .execute(&data.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success"})),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "fail"
        })),
    }
    // HttpResponse::Ok().json(json!({"status": "success",  "data": serde_json::json!({
    //         "user": user
    //     })}))
}

#[get("/auth/logout")]
async fn logout(_: jwt_auth::JwtMiddleware) -> impl Responder {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success"}))
}
