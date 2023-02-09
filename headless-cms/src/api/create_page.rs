use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PageInfo {
    page: String,
    title: String,
    url: String,
}

#[post("/createPage")]
pub async fn create_page(info: web::Json<PageInfo>) -> impl Responder {
    // let (name) = path.into_inner();
    println!("title: {} \n {} \n\n\n", info.title, info.page);
    HttpResponse::Ok().body("new page")
}
