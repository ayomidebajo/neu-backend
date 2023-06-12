use postgres::{Client, Error};
use serde_derive::{Deserialize, Serialize};
use password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng};

// use error;
// use crate::error::MyError;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(name: String, email: String, password: String) -> Self {
        Self {
            name,
            email,
            password,
        }
    }
}


pub fn create_table(conn: &mut Client) -> Result<(), Error> {
    conn.execute(
        "CREATE TABLE users ( id SERIAL PRIMARY KEY, email VARCHAR UNIQUE, fname VARCHAR, lname VARCHAR, password BYTEA NOT NULL, created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, is_verified BOOLEAN, is_subsc BOOLEAN, updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP)",
        &[],
    )
    .map(drop).err();

	conn.execute(
		"CREATE TABLE merchants ( id SERIAL PRIMARY KEY, fname VARCHAR NOT NULL, lname VARCHAR NOT NULL, email VARCHAR UNIQUE, phone_no VARCHAR, business_name VARCHAR NOT NULL, password BYTEA NOT NULL, addr VARCHAR, tagline VARCHAR, no_of_staff SERIAL, is_verified BOOLEAN, created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP)",
		&[],
	)
	.map(drop).err();

	conn.execute("CREATE TABLE settings (id SERIAL PRIMARY KEY, fname VARCHAR NOT NULL, lname VARCHAR NOT NULL, email VARCHAR NOT NULL, phone_no VARCHAR, is_merchant BOOLEAN )", &[]).map(drop).err();

	conn.execute("CREATE TABLE kyc (id SERIAL PRIMARY KEY, identifcation_type VARCHAR NOT NULL, id_image VARCHAR, business_location VARCHAR, professional_cert_type VARCHAR)", &[]).map(drop)
}

pub fn _create_merchant(conn: &mut Client) -> Result<(), Error> {
	conn.execute(
		"CREATE TABLE merchants ( id SERIAL PRIMARY KEY, fname VARCHAR NOT NULL, lname VARCHAR NOT NULL, email VARCHAR UNIQUE, business_name VARCHAR NOT NULL, password VARCHAR NOT NULL, created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP)",
		&[],
	)
	.map(drop)
}

pub fn create_user(conn: &mut Client, user: &User) -> Result<(), Error> {
	let binding = PasswordHash::new(&user.password).unwrap();

 	// let password_hash = binding.as_bytes();

	print!("pass {:?}", binding);
    // conn.execute(
    //     "INSERT INTO users (email, password) VALUES ($1, $2)",
    //     &[&user.email, &user.password],
    // )
    // .map(drop)

	Ok(())
}

pub fn list_users(conn: &mut Client) -> Result<Vec<User>, Error> {
    let res = conn
        .query("SELECT fname, email FROM users", &[])?
        .into_iter()
        .map(|row| {


            User {
                name: row.get(0),
                email: row.get(1),
                   password: "password".to_string(),
            }
        })
        .collect();
    Ok(res)
}

// This function demonstrates how to insert data into the database.
