use std::fmt::{Display, Formatter, Result as FmtResult};

// https://github.com/actix/examples/blob/master/json/json-error/src/main.rs
use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use serde_json::{json, to_string_pretty};

#[derive(Debug, Serialize)]
pub struct JsonError {
    error_msg: String,
    error_code: u16,
}

impl JsonError {
    pub fn new(error_msg: String, error_code: u16) -> JsonError {
        JsonError {
            error_msg,
            error_code,
        }
    }
}

impl Display for JsonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", to_string_pretty(self).unwrap())
    }
}

impl ResponseError for JsonError {
    // builds the actual response to send back when an error occurs
    fn error_response(&self) -> HttpResponse {
        let err_json = json!({ "error": self.error_msg});
        HttpResponse::build(StatusCode::from_u16(self.error_code).unwrap()).json(err_json)
    }
}
