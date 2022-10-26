use actix_web::web;
use crate::handler::health::healthcheck;
use crate::handler::index::index;
use crate::handler::add_user::add_user;
use crate::handler::get_user::get_user;


pub fn init(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(index)
            .service(healthcheck)
            .service(add_user)
            .service(get_user)
    );
}
