#[macro_use]
extern crate diesel;

mod api;
mod db;
mod models;
mod schema;
mod utils;

use actix_web::{error, get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    println!("{}", req_body);
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // change json extractor configuration
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .service(api::create_page::create_page)
            .service(api::create_page_type::post)
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                error::InternalError::from_response(
                    "",
                    HttpResponse::BadRequest()
                        .content_type("application/json")
                        .body(format!(r#"{{"error":"{}"}}"#, err)),
                )
                .into()
            }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
