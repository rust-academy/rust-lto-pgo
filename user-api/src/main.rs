use std::collections::HashMap;
use std::sync::Mutex;

use actix_web::{App, HttpServer, middleware, web};

use crate::config::init;
use crate::handler::stop::{stop, StopHandle};
use crate::model::user::User;

mod config;
mod handler;
mod model;
mod error;

const NUMBER_WORKERS: usize = 6;
const ADDRESS: &str = "0.0.0.0:8080";

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // create data store container https://doc.rust-lang.org/rust-by-example/std/hash.html
    let storage: web::Data<Mutex<HashMap<String, User>>> =
        web::Data::new(Mutex::new(HashMap::new()));

    // create the stop handle container
    let stop_handle = web::Data::new(StopHandle::default());

    // start server as normal but don't .await after .run() yet
    println!("HTTP server at http://localhost:8080");
    let srv = HttpServer::new(move || {
        App::new()
            .app_data(storage.clone())
            .configure(init)    //  see config.rs for details on initialization
            .service(stop)  // attach stop handler
            .wrap(middleware::Logger::default())
    })
        .bind(ADDRESS)?.workers(NUMBER_WORKERS).run();

    stop_handle.clone().register(srv.handle());  // register the server handle with the stop handle

    srv.await // run server until stopped (either by ctrl-c or stop endpoint)
}