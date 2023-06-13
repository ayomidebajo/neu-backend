
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
}