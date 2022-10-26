use std::collections::HashMap;
use std::sync::Mutex;
use actix_web::{App, HttpServer, middleware, web};
use crate::config::init;
use crate::model::user::User;

mod config;
mod handler;
mod model;
mod error;

const NUMBER_WORKERS: usize = 6;
const ADDRESS: &str = "0.0.0.0:8080";

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let storage: web::Data<Mutex<HashMap<String, User>>> =
        web::Data::new(Mutex::new(HashMap::new()));

    println!("HTTP server at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(storage.clone())
            .configure(init)
            .wrap(middleware::Logger::default())
    })
        .bind(ADDRESS)?
        .workers(NUMBER_WORKERS)
        .run()
        .await
}

