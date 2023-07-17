use crate::routes::{
    auth::{sign_in, sign_up, get_user, update_user},
    // auth::sign_in,
    health_check::health_check,
    home_page::home_page,
};
use actix_web::web::{self};

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_check)
        .service(sign_up)
        .service(sign_in)
        .service(get_user)
        .service(home_page)
        .service(update_user);

    conf.service(scope);
}
