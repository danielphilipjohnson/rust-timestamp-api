mod error;
mod handlers;

use crate::handlers::{get_time, get_timezones, get_weekday};
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting on localhost:8080");

    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .service(web::resource("/timezones").route(web::get().to(get_timezones)))
                .service(web::resource("/weekday/{date}").route(web::get().to(get_weekday)))
                .service(web::resource("/{date}").route(web::get().to(get_time)))
                .service(web::resource("").route(web::get().to(get_time))),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
