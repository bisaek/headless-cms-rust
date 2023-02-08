use actix_web::{get, post, web, HttpResponse, Responder, Result};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PageInfo {
    page: String,
    title: String,
    url: String,
}

#[post("/createPage")]
pub async fn create_page(info: web::Json<PageInfo>) -> impl Responder {
    // let (name) = path.into_inner();
    println!("{}", info.title);
    HttpResponse::Ok().body("new page")
}
