use actix_web::{Responder, HttpResponse};
use postgres::{Client, Error};

use crate::db;

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn home_page() -> impl Responder {
	HttpResponse::Ok().body("This is the home page")
}

pub fn sign_up(conn: &mut Client, user: db::User) -> Result<(), Error> {
	conn.execute(
		"INSERT INTO users (fname, lname, email, password) VALUES ($1, $2, $3, $4)",
		&[&user.name, &user.name, &user.email, &user.password],
	)
	.map(drop)
}