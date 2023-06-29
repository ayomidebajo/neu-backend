use crate::models::Customer;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::helpers::pass_helpers::hash_password;

// ADD HASHING POWER

pub async fn sign_up(req: web::Json<Customer>, connection: web::Data<PgPool>) -> HttpResponse {
    // let password = &req.password;
    // let password_hash = bcrypt::hash(password).unwrap();

    let hashed_password = match hash_password(&req.password) {
        Ok(hashed) => Some(hashed),
        Err(err) => {
            println!("Error hashing password: {}", err);
            // return;
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

    // check if the email is already saved in db
    match sqlx::query!(r#"SELECT email FROM customers WHERE email = $1"#, req.email)
        .fetch_optional(connection.get_ref())
        .await
    {
        Ok(email_exists) => {
            if let Some(email) = email_exists {
                tracing::info!(
                    "request_id {} - Email '{:?}' already exists",
                    request_id,
                    email
                );
                HttpResponse::Conflict().finish()
            } else {
                match sqlx::query!(
        r#"
INSERT INTO customers (id, email, fname, lname,password, is_verified, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7)
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
    .execute(connection.get_ref())
    .await
    {
        Ok(_) => {
            tracing::info!(
            "request_id {} - New customer details have been saved", request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
            "request_id {} - Failed to execute query: {:?}",
            request_id,
            e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
            }
        }

        Err(_) => {
            tracing::info!(
                "request_id {} - Saving new subscriber details in the database",
                request_id
            );

            HttpResponse::from_error(actix_web::error::ErrorInternalServerError(
                "Failed to connect to database",
            ))
        }
    }

    // if let Some(email) = email_exists {
    //     tracing::info!(
    //         "request_id {} - Email '{:?}' already exists",
    //         request_id,
    //         email
    //     );
    //     return HttpResponse::Conflict().finish();
    // }

    // tracing::info!(
    //     "request_id {} - Saving new subscriber details in the database",
    //     request_id
    // );

    // HttpResponse::Ok().body(format!("Welcome {} {}", req.fname, req.email))
}
