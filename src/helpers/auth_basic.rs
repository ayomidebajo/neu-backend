use std::string::FromUtf8Error;

use actix_web::http::header::HeaderMap;
use base64::{engine::general_purpose::STANDARD, Engine};

fn _basic_authentication(headers: &HeaderMap) -> Result<String, FromUtf8Error> {
    let header_value = headers
        .get("Authorization")
        .expect("The auth header was missing")
        .to_str()
        .expect("The authorization header was not a valid utf-8 string");

    let base64encoded = header_value
        .strip_prefix("Basic ")
        .expect("The auth scheme wasn't basic");

    let decoded_bytes = Engine::decode(&STANDARD, base64encoded).expect("Couldn't decode bytes");

    let decoded_cred = String::from_utf8(decoded_bytes)?;

    Ok(decoded_cred)
}
