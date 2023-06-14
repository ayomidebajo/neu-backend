use serde_derive::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]

pub struct Customer {
    pub fname: String,
    pub lname: String,
    pub email: String,
    pub phone_no: String,
    pub password: String,
    pub is_merchant: bool,
    pub is_verified_user: bool,
    // pub created_at: DateTime<Utc>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Subcriptions {
    pub email: String,
    pub subscribed_at: String,
    pub name: String,
}
