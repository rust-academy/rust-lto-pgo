use std::collections::HashMap;
use std::sync::Mutex;
use actix_web::{web, post, HttpResponse};
use uuid::Uuid;
use crate::model::user::{User};

/// Inserts new user with name defined in form.
#[post("/user/add/{user_name}")]
pub async fn add_user(
    storage: web::Data<Mutex<HashMap<String, User>>>,
    path: web::Path<String>,
) -> HttpResponse {

    let id = Uuid::new_v4();
    let name = path.into_inner();

    let res = storage
        .lock()
        .unwrap()
        .insert(id.to_string(), User::new(id.to_string(),name));

    // Inserts a key-value pair into the map.
    // If the map did not have this key present, None is returned.
    // If the map did have this key present, the value is updated and the old value is returned.
    match res{
        Some(_) => {
            HttpResponse::Ok().json("Updated") // <- send response
        }
        None => {
            HttpResponse::Ok().json(id.to_string()) // <- send response
        }
    }
}