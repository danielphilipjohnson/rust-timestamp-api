use crate::error::ApiError;
use actix_web::{web, HttpResponse};
use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::Tz;
use serde::Serialize;

#[derive(Serialize)]
pub struct TimeResponse {
    unix: i64,
    utc: String,
    timezone: String,
    local_time: String,
}

pub async fn get_time(
    date: Option<web::Path<String>>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<HttpResponse, ApiError> {
    let time = match date {
        Some(path_date) => {
            let date_str = path_date.as_str();
            chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                .map(|naive_date| Utc.from_utc_datetime(&naive_date.and_hms_opt(0, 0, 0).unwrap()))
                .map_err(|e| ApiError::InvalidDate(e.to_string()))?
        }
        None => Utc::now(),
    };

    let timezone_str = query.get("timezone").map(|s| s.as_str()).unwrap_or("UTC");

    let local_time = if timezone_str == "UTC" {
        time.format("%Y-%m-%d %H:%M:%S +0000").to_string()
    } else {
        convert_to_timezone(time, timezone_str)?
            .format("%Y-%m-%d %H:%M:%S %z")
            .to_string()
    };

    let response = TimeResponse {
        unix: time.timestamp() * 1000,
        utc: time.to_rfc2822(),
        timezone: timezone_str.to_string(),
        local_time,
    };

    Ok(HttpResponse::Ok().json(response))
}

fn convert_to_timezone(date: DateTime<Utc>, timezone: &str) -> Result<DateTime<Tz>, ApiError> {
    timezone
        .parse::<Tz>()
        .map(|tz| date.with_timezone(&tz))
        .map_err(|_| ApiError::InvalidTimezone("Invalid timezone format".to_string()))
}
