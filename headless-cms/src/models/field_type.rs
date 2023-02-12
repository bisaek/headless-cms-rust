// use std::str;

use crate::schema::field_type;

#[derive(Insertable)]
#[table_name = "field_type"]
pub struct New<'a> {
    pub f_type: &'a str,
    pub name: &'a str,
    pub page_type: i32,
}

// #[derive(Debug, Queryable, AsChangeset)]
// pub struct FieldType {
//     pub id: i32,
//     pub f_type: String,
//     pub name: String,
//     pub page_type: i32,
// }
