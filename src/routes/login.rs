// use actix_web::{Responder, HttpResponse};
use crate::models::{LoginUser, TestStruct};
use actix_web::{web, HttpResponse};
// use pwhash::bcrypt;
// use chrono::Utc;
use crate::helpers::pass_helpers::verify_password;
// use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::PgPool;
// use uuid::Uuid;

// pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {

//     // Hash the password using the generated salt
//     let hashed_password = hash(password, DEFAULT_COST)?;

//     Ok(hashed_password)
// }

// pub fn verify_password(password: &str, hashed_password: &str) -> bool {
//     // Verify the provided password against the stored hashed password
//     match verify(password, hashed_password) {
//         Ok(result) => result,
//         Err(_) => false, // Verification failed
//     }
// }

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
