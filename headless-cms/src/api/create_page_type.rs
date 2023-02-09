use actix_web::{post, web::Json, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Field {
    field_type: String,
    name: String,
}

#[derive(Deserialize)]
pub struct PageType {
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
    println!("{}", page_type.fields[3].name);
    HttpResponse::Ok().body("new page type")
}
