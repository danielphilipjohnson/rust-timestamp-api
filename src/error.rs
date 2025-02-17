use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::Serialize;

#[derive(Debug, Display, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum ApiError {
    #[display(fmt = "Invalid date: {}", _0)]
    InvalidDate(String),
    #[display(fmt = "Invalid timezone: {}", _0)]
    InvalidTimezone(String),
    #[display(fmt = "Internal error: {}", _0)]
    InternalError(String),
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::InvalidDate(_) | ApiError::InvalidTimezone(_) => {
                HttpResponse::BadRequest().json(self)
            }
            ApiError::InternalError(_) => HttpResponse::InternalServerError().json(self),
        }
    }
}
