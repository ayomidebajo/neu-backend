use crate::helpers::parser::name_parser;
use actix_web::Error;
use chrono::Utc;
use sqlx;
use uuid::Uuid;

use serde_derive::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Clone, Debug)]

pub struct Customer {
    pub fname: String,
    pub lname: String,
    pub email: String,
    pub phone_no: String,
    pub password: String,
    pub is_verified_user: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug, sqlx::FromRow)]
pub struct GetUser {
    pub id: Uuid,
    pub fname: String,
    pub lname: String,
    pub email: String,
    // pub phone_no: String,
    pub password: String,
    pub is_verified: bool,
    pub created_at: chrono::DateTime<Utc>,
    // pub is_merchant: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

impl Customer {
    pub fn parse_validate(cust: Customer) -> Result<Customer, Error> {
        if !name_parser(cust.clone().fname) {
            return Err(actix_web::error::ErrorUnauthorized(
                "Incorrect first name format, names must contain letters only",
            ));
        }
        if !name_parser(cust.clone().lname) {
            return Err(actix_web::error::ErrorUnauthorized(
                "Incorrect last name format, names must contain letters only",
            ));
        }
        if cust.email.len() < 3 {
            return Err(actix_web::error::ErrorUnauthorized(
                "Incorrect email format",
            ));
        }
        if cust.phone_no.len() < 3 {
            return Err(actix_web::error::ErrorUnauthorized(
                "Incorrect phone_no length",
            ));
        }
        if cust.password.len() < 3 {
            return Err(actix_web::error::ErrorUnauthorized(
                "Invalid password length, passwords must be aleast 8 characters",
            ));
        }
        Ok(cust)
    }
}
#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct TestStruct {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, sqlx::FromRow)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

// #[derive(Deserialize, Serialize, Clone, Debug, sqlx::FromRow)]

#[derive(Deserialize, Serialize, Debug)]
pub struct Subcriptions {
    pub email: String,
    pub subscribed_at: String,
    pub name: String,
}
