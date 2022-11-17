use std::collections::HashMap;
use std::sync::Mutex;
use actix_web::{App, HttpServer, middleware, web};
use crate::config::init;
use crate::handler::stop::{stop, StopHandle};

mod config;
mod handler;
mod error;

const NUMBER_WORKERS: usize = 6;
const ADDRESS: &str = "0.0.0.0:8080";

struct StateWithCounter {
    counter: Mutex<u64>, // <- Mutex is necessary to mutate safely across threads
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // create shared mutex counter https://actix.rs/docs/application/#state
    let counter = web::Data::new(StateWithCounter {
        counter: Mutex::new(0),
    });

    // create data store container https://doc.rust-lang.org/rust-by-example/std/hash.html
    let storage: web::Data<Mutex<HashMap<String, String>>> =
        web::Data::new(Mutex::new(HashMap::new()));

    // create the stop handle container
    let stop_handle = web::Data::new(StopHandle::default());

    // start server as normal but don't .await after .run() yet
    println!("HTTP server at http://localhost:8080");
    let srv = HttpServer::new(move || {
        App::new()
            .app_data(storage.clone()) // register the data store container
            .app_data(counter.clone()) // register the created counter
            .configure(init)    //  see config.rs for details on initialization
            .service(stop)  // attach stop handler
            .wrap(middleware::Logger::default())
    })
        .bind(ADDRESS)?.workers(NUMBER_WORKERS).run();

    stop_handle.clone().register(srv.handle());  // register the stop handler with the server

    srv.await // run server until stopped (either by ctrl-c or stop endpoint)
}