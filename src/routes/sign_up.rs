use crate::models::{Customer, Subcriptions};
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn sign_up(req: web::Json<Customer>, connection: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        r#"
INSERT INTO customers (id, email, fname, lname, is_merchant, password, is_verified, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
"#,
        Uuid::new_v4(),
        req.email,
        req.fname,
        req.lname,
        req.is_merchant,
        req.password,
        req.is_verified_user,
        Utc::now()
    )
    // We use `get_ref` to get an immutable reference to the `PgConnection`
    // wrapped by `web::Data`.
    .execute(connection.get_ref())
    .await
    {
        Ok(_) => {
            return HttpResponse::Ok().finish();
        }
        Err(e) => {
            println!("Failed to execute query: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
    // HttpResponse::Ok().body(format!("Welcome {} {}", req.fname, req.email))
}
