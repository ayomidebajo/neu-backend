use crate::authentication::jwt_auth;
use crate::config::AppState;
use crate::models::CreateBusiness;
use crate::models::GetUser;
use actix_web::{post, web, Error, HttpResponse};
use serde_json::json;
use uuid::Uuid;

// Handler for creating a new business
#[post("/merchant/create_busi_profile")]
pub async fn create_business_profile(
    user_details: web::Json<CreateBusiness>,
    connection: web::Data<AppState>,
    jwt: jwt_auth::JwtMiddleware,
) -> Result<HttpResponse, Error> {
    let user: Option<GetUser> = sqlx::query_as::<_, GetUser>(
        "SELECT * FROM merchant_business_profile WHERE business_email = $1",
    )
    .bind(user_details.business_email.to_string())
    .fetch_optional(&connection.db)
    .await
    .expect("error");

    let merchant_id = jwt.user_id;

    let request_id = Uuid::new_v4();
    tracing::info!(
        "request_id {} - Adding '{}' as a new merchant.",
        request_id,
        user_details.business_email,
    );
    // if user exists, return error
    if user.is_some() {
        tracing::warn!(
            "request_id {} - '{}' already exists as a merchant.",
            request_id,
            user_details.business_email,
        );
        return Ok(HttpResponse::Conflict().json(json!({
            "status": "error",
            "message": "User already exists"
        })));
    }
    // if user does not exist, create profile
    let query = "INSERT INTO merchant_business_profile (id, merchant_id, business_id, no_of_employees, services, merchant_logo, business_phone, business_email, business_website, business_description) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)";

    match sqlx::query(query)
        .bind(Uuid::new_v4())
        .bind(merchant_id)
        .bind(Uuid::new_v4())
        .bind(user_details.no_of_employees)
        .bind(&user_details.services)
        .bind(&user_details.merchant_logo)
        // .bind(&user_details.business_name)
        .bind(&user_details.business_address)
        .bind(&user_details.business_phone)
        .bind(&user_details.business_email)
        .bind(&user_details.business_website)
        .bind(&user_details.business_description)
        .execute(&connection.db)
        .await
    {
        Ok(_) => {
            tracing::info!(
                "request_id {} - '{}' successfully created as a merchant.",
                request_id,
                user_details.business_email,
            );
            Ok(HttpResponse::Ok().json(json!({
                "status": "success",
                "message": "Merchant successfully created"
            })))
        }
        Err(e) => {
            tracing::error!(
                "request_id {} - An error occurred while creating '{}' as a merchant. {}",
                request_id,
                user_details.business_email,
                e
            );
            Ok(HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "An error occurred while creating merchant"
            })))
        }
    }
}
