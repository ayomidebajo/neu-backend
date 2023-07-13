use crate::routes::{
    auth::{get_user, sign_in, sign_up, logout},
    health_check::health_check,
};
use actix_web::web;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_check)
        .service(sign_up)
        .service(sign_in)
        .service(logout)
        .service(get_user);

    conf.service(scope);
}
