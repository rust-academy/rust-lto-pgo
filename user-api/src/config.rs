use actix_web::web;
use crate::handler::health::healthcheck;
use crate::handler::index::index;
use crate::handler::user::{add_user, delete_user, update_user, get_user};

pub fn init(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(index)
            .service(healthcheck)
            .service(add_user)
            .service(delete_user)
            .service(update_user)
            .service(get_user)
    );
}
