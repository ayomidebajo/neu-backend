use actix_web::{Responder, HttpResponse, web};
use postgres::{Client, Error};

use crate::db;
use crate::models::Customer;

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn home_page() -> impl Responder {
	HttpResponse::Ok().body("This is the home page")
}

pub async fn sign_up(req: web::Json<Customer> ) -> impl Responder {
	HttpResponse::Ok().body(format!("Welcome {} {}", req.fname, req.email))
}

