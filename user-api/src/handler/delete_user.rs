use std::collections::HashMap;
use std::sync::Mutex;
use actix_web::{HttpResponse, delete, web,ResponseError};
use crate::error::json_error::JsonError;
use crate::model::user::{User};

#[delete("/users/delete/{user_id}")]
pub async fn delete_user(
    path: web::Path<String>,
    storage: web::Data<Mutex<HashMap<String, User>>>,
) -> HttpResponse {

    // extract user id from path
    let user_id = path.into_inner();

    // return user if found, errors otherwise
    match storage.lock().unwrap().remove(user_id.as_str()){
        Some(_) => {
            HttpResponse::Ok().json("Deleted") // <- send response
        }
        None => {
            JsonError::new("User not found".to_string(), 500).error_response()
        }
    }
}