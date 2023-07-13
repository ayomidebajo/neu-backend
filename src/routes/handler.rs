use crate::routes::{
    auth::{get_user, logout, sign_in, sign_up},
    health_check::health_check,
    home_page::home_page,
};
use actix_web::web;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_check)
        .service(sign_up)
        .service(sign_in)
        .service(logout)
        .service(get_user)
        .service(home_page);

    conf.service(scope);
}
