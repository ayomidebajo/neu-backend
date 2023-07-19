use crate::routes::{
    auth::{get_user, logout, merchant_sign_in, merchant_sign_up, sign_in, sign_up, update_user},
    health_check::health_check,
    home_page::home_page,
    merchant::create_business_profile,
};
use actix_web::web::{self};

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_check)
        .service(sign_up)
        .service(sign_in)
        .service(get_user)
        .service(home_page)
        .service(logout)
        .service(update_user)
        .service(create_business_profile)
        .service(merchant_sign_up)
        .service(merchant_sign_in);

    conf.service(scope);
}
