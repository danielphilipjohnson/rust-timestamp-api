use crate::error::ApiError;
use actix_web::{web, HttpResponse};
use chrono::{TimeZone, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct WeekdayResponse {
    weekday: String,
}

pub async fn get_weekday(date: web::Path<String>) -> Result<HttpResponse, ApiError> {
    let date_str = date.as_str();

    let parsed_date = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map(|naive_date| Utc.from_utc_datetime(&naive_date.and_hms_opt(0, 0, 0).unwrap()))
        .map_err(|e| ApiError::InvalidDate(e.to_string()))?;

    let weekday = parsed_date.format("%A").to_string();
    let response = WeekdayResponse { weekday };

    Ok(HttpResponse::Ok().json(response))
}
