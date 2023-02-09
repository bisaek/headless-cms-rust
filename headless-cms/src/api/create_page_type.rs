use actix_web::{post, web::Json, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Field {
    fieldType: String,
    name: String,
}

#[derive(Deserialize)]
struct PageType {
    fields: Vec<Field>,
}
// impl PageType {
//     fn newField(&mut self, fieldType: String, name: String) {
//         self.fields.push(Field {
//             fieldType: fieldType,
//             name: name,
//         })
//     }
// }

#[post("createPageType")]
pub async fn create_page_type(page_type: Json<PageType>) -> impl Responder {
    println!("{}", pageType.fields[3].name);
    HttpResponse::Ok().body("new page type")
}
