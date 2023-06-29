use sqlx;

use serde_derive::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]

pub struct Customer {
    pub fname: String,
    pub lname: String,
    pub email: String,
    pub phone_no: String,
    pub password: String,
    pub is_verified_user: bool,
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

#[derive(Deserialize, Serialize, Debug)]
pub struct Subcriptions {
    pub email: String,
    pub subscribed_at: String,
    pub name: String,
}
