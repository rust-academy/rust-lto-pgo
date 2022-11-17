use std::collections::HashMap;
use std::sync::Mutex;

use actix_web::{delete, get, HttpResponse, post, ResponseError, web};
use uuid::Uuid;

use crate::error::json_error::JsonError;
use crate::model::user::User;

// === add_user  === //

// https://actix.rs/docs/extractors/
#[post("/user/add/{user_name}")]
pub async fn add_user(
    storage: web::Data<Mutex<HashMap<String, User>>>,
    path: web::Path<String>,
) -> HttpResponse
{
    let id = Uuid::new_v4();
    let name = path.into_inner();

    let res = storage
        .lock()
        .unwrap()
        .insert(id.to_string(), User::new(id.to_string(), name));

    // Inserts a key-value pair into the map.
    // If the map did not have this key present, None is returned.
    // If the map did have this key present, the value is updated and the old value is returned.
    match res {
        Some(_) => {
            HttpResponse::Ok().json("Added") // <- send response
        }
        None => {
            HttpResponse::Ok().json(id.to_string()) // <- send response
        }
    }
}

// === get_user  === //

#[get("/users/get/{user_id}")]
pub async fn get_user(
    path: web::Path<String>,
    storage: web::Data<Mutex<HashMap<String, User>>>,
) -> HttpResponse
{
    // extract user id from path
    let user_id = path.into_inner();

    // return user if found, errors otherwise
    match storage.lock().unwrap().get(user_id.as_str()) {
        Some(value) => {
            HttpResponse::Ok().json(value) // <- send response
        }
        None => {
            JsonError::new("User not found".to_string(), 500).error_response()
        }
    }
}

// === update_user  === //

#[post("/users/update/")]
pub async fn update_user(
    user: web::Json<User>,
    storage: web::Data<Mutex<HashMap<String, User>>>,
) -> HttpResponse
{
    // extract user id & name from JSON payload
    let user_id = &user.id;
    let user_name = &user.name;

    // If the map did have this key present, the value is updated and the old value is returned.
    let res = storage
        .lock()
        .unwrap()
        .insert(user_id.to_string(), User::new(user_id.to_string(), user_name.to_string()));

    match res {
        Some(_) => {
            HttpResponse::Ok().json("Updated") // <- send response
        }
        None => {
            JsonError::new("User not found".to_string(), 500).error_response()
        }
    }
}

// === delete_user  === //

#[delete("/users/delete/{user_id}")]
pub async fn delete_user(
    path: web::Path<String>,
    storage: web::Data<Mutex<HashMap<String, User>>>,
) -> HttpResponse
{
    // extract user id from path
    let user_id = path.into_inner();

    match storage.lock().unwrap().remove(user_id.as_str()) {
        Some(_) => {
            HttpResponse::Ok().json("Deleted") // <- send response
        }
        None => {
            JsonError::new("User not found".to_string(), 500).error_response()
        }
    }
}
