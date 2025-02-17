use crate::error::ApiError;
use actix_web::HttpResponse;

pub async fn get_timezones() -> Result<HttpResponse, ApiError> {
    let timezones: Vec<String> = chrono_tz::TZ_VARIANTS
        .iter()
        .map(|tz| String::from(tz.name()))
        .collect();

    if timezones.is_empty() {
        return Err(ApiError::InternalError(
            "No timezones available".to_string(),
        ));
    }

    Ok(HttpResponse::Ok().json(timezones))
}
