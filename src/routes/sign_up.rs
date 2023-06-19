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

    match sqlx::query!(
        r#"
INSERT INTO customers (id, email, fname, lname, is_merchant, password, is_verified, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
"#,
        Uuid::new_v4(),
        req.email,
        req.fname,
        req.lname,
        req.is_merchant,
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
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
    // HttpResponse::Ok().body(format!("Welcome {} {}", req.fname, req.email))
}
