use crate::schema::page_type;

#[derive(Insertable)]
#[table_name = "page_type"]
pub struct NewPageType<'a> {
    pub name: &'a str,
}

// #[derive(Debug, Queryable, AsChangeset)]
// pub struct PageType {
//     pub id: i32,
//     pub name: String,
// }
