use crate::models::LoginUser;
use crate::session_state::TypedSession;
// use crate::utils::{e500, see_other};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
// use actix_web::error::{InternalError, ErrorBadRequest};
use actix_web::{FromRequest, HttpMessage, HttpResponse};
use actix_web_lab::middleware::Next;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use reqwest::header::LOCATION;

pub fn see_other(location: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, location))
        .finish()
}

// Middleware for JWT authentication
pub async fn authenticate(
    req: actix_web::HttpRequest,
    _payload: actix_web::dev::Payload,
) -> Result<actix_web::HttpRequest, actix_web::Error> {
    // Extract the JWT token from the request headers
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| {
            if value.starts_with("Bearer ") {
                Some(value.trim_start_matches("Bearer ").to_owned())
            } else {
                None
            }
        });

    // Verify and decode the JWT token
    let validation = Validation::new(Algorithm::HS256);
    const SECRET_KEY: &[u8] = b"secret";
    let token_data = match token {
        Some(t) => decode::<LoginUser>(&t, &DecodingKey::from_secret(SECRET_KEY), &validation)
            .expect("error decoding"),
        None => return Err(actix_web::error::ErrorUnauthorized("Unauthorized")),
    };

    // Attach the decoded user data to the request
    let request = req
        .clone()
        .extensions_mut()
        .insert(token_data.claims)
        .map(|_| req)
        .ok_or(actix_web::error::ErrorInternalServerError(
            "Internal Server Error",
        ))?;

    Ok(request)
}

pub async fn reject_anonymous_users(
    mut req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let session = {
        let (http_request, payload) = req.parts_mut();
        TypedSession::from_request(http_request, payload).await
    }?;

    match session
        .get_user_id()
        .map_err(|_e| actix_web::error::ErrorInternalServerError("server error"))?
    {
        Some(user_id) => {
            req.extensions_mut().insert(user_id);
            next.call(req).await
        }
        None => {
            let _response = see_other("/login");
            // let e = Err(ErrorBadRequest("something went wrong"));
            Err(actix_web::error::ErrorInternalServerError(
                "something went wrong while getting user id",
            ))
        }
    }
}

// Function to extract the bearer token from the header value
fn _extract_bearer_token(header_value: &str) -> Option<&str> {
    // Check if the header value starts with "Bearer "
    if header_value.starts_with("Bearer ") {
        // Extract the token by skipping the first 7 characters
        Some(&header_value[7..])
    } else {
        None
    }
}

// // Example route
// async fn index(req: HttpRequest) -> Result<&'static str> {
//     Ok("Hello World")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .wrap(BearerAuthMiddleware)
//             .service(web::resource("/").to(index))
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }
