use actix_web::{test, web, App};
use serde_json::Value;
use timestamp_microservice::handlers::{get_time, get_timezones, get_weekday};

#[actix_web::test]
async fn test_get_time_current() {
    let app = test::init_service(App::new().route("/api", web::get().to(get_time))).await;

    let req = test::TestRequest::get().uri("/api").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());

    let body: Value = test::read_body_json(resp).await;
    assert!(body.get("unix").is_some());
    assert!(body.get("utc").is_some());
    assert_eq!(body["timezone"], "UTC");
}

#[actix_web::test]
async fn test_get_time_specific_date() {
    let app = test::init_service(App::new().route("/api/{date}", web::get().to(get_time))).await;

    let req = test::TestRequest::get().uri("/api/2025-02-17").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());

    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["timezone"], "UTC");
}

#[actix_web::test]
async fn test_get_time_invalid_date() {
    let app = test::init_service(App::new().route("/api/{date}", web::get().to(get_time))).await;

    let req = test::TestRequest::get()
        .uri("/api/invalid-date")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);

    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["type"], "InvalidDate");
    assert_eq!(body["message"], "input contains invalid characters");
}

#[actix_web::test]
async fn test_get_time_with_timezone() {
    let app = test::init_service(App::new().route("/api/{date}", web::get().to(get_time))).await;

    let req = test::TestRequest::get()
        .uri("/api/2025-02-17?timezone=America/New_York")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());

    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["timezone"], "America/New_York");
}

#[actix_web::test]
async fn test_get_weekday() {
    let app =
        test::init_service(App::new().route("/api/weekday/{date}", web::get().to(get_weekday)))
            .await;

    let req = test::TestRequest::get()
        .uri("/api/weekday/2025-02-17")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());

    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["weekday"], "Monday"); // Feb 17, 2025 is a Monday
}

#[actix_web::test]
async fn test_get_weekday_invalid_date() {
    let app =
        test::init_service(App::new().route("/api/weekday/{date}", web::get().to(get_weekday)))
            .await;

    let req = test::TestRequest::get()
        .uri("/api/weekday/invalid-date")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);

    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["type"], "InvalidDate");
    assert_eq!(body["message"], "input contains invalid characters");
}

#[actix_web::test]
async fn test_get_timezones() {
    let app =
        test::init_service(App::new().route("/api/timezones", web::get().to(get_timezones))).await;

    let req = test::TestRequest::get().uri("/api/timezones").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());

    let body: Value = test::read_body_json(resp).await;
    assert!(body.as_array().unwrap().len() > 0);
    assert!(body
        .as_array()
        .unwrap()
        .contains(&Value::String("UTC".to_string())));
    assert!(body
        .as_array()
        .unwrap()
        .contains(&Value::String("America/New_York".to_string())));
}
