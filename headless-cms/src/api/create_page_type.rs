use crate::db::establish_connection;
use crate::models::page_type::{self, NewPageType};
use actix_web::{post, web::Json, HttpResponse, Responder};
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Field {
    field_type: String,
    name: String,
}

#[derive(Deserialize)]
pub struct PageType {
    fields: Vec<Field>,
    name: String,
}
// impl PageType {
//     fn newField(&mut self, fieldType: String, name: String) {
//         self.fields.push(Field {
//             fieldType: fieldType,
//             name: name,
//         })
//     }
// }

pub fn add_to_db(info: Json<PageType>) {
    println!("{}", info.name);

    // use crate::schema::page_type;
    use crate::schema::page_type::dsl::*;

    let mut connection = establish_connection();

    let new_page_type = NewPageType { name: &info.name };

    diesel::insert_into(crate::schema::page_type::dsl::page_type)
        .values(&new_page_type)
        .execute(&mut connection)
        .expect("Error saving new page type");
}

#[post("createPageType")]
pub async fn post(info: Json<PageType>) -> impl Responder {
    // println!("{}", page_type.fields[3].name);
    add_to_db(info);
    HttpResponse::Ok().body("new page type")
}
