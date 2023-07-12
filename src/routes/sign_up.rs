use crate::config::AppState;
use crate::models::Customer;
use actix_web::{web, HttpResponse};
use chrono::Utc;
// use sqlx::PgPool;
use uuid::Uuid;

use crate::helpers::pass_helpers::hash_password;

pub async fn sign_up(req: web::Json<Customer>, connection: web::Data<AppState>) -> HttpResponse {
    let hashed_password = match hash_password(&req.password) {
        Ok(hashed) => Some(hashed),
        Err(err) => {
            println!("Error hashing password: {}", err);
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

            if let Ok(email) = email_exists {
                if email.is_some() {
                    tracing::info!(
                        "request_id {} - Email '{:?}' already exists",
                        request_id,
                        email
                    );
                    println!("Email already exists {:?}", email);
                    return actix_web::HttpResponse::Conflict().json("Email already exists");
                }
            }

            tracing::info!(
                "request_id {} - Saving new subscriber details in the database",
                request_id
            );
            // save user into database
            match sqlx::query!(
        r#"
INSERT INTO customers (id, email, fname, lname, password, is_verified, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7)
"#,
        Uuid::new_v4(),
        req.email,
        req.fname,
        req.lname,
        hashed_password,
        req.is_verified_user,
        Utc::now()
    )
    // We use `get_ref` to get an immutable reference to the `PgConnection`
    // wrapped by `web::Data`.
    .execute(&connection.db)
    .await
    {
        Ok(_) => {
            tracing::info!(
            "request_id {} - New customer details have been saved", request_id
            );
            actix_web::HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
            "request_id {} - Failed to execute query: {:?}",
            request_id,
            e
            );
            actix_web::HttpResponse::InternalServerError().finish()
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
