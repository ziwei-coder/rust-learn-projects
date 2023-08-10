use crate::schema::products;
use diesel::{AsChangeset, Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Queryable, AsChangeset)]
#[serde(crate = "rocket::serde")]
pub struct Product {
    pub id: i32,
    pub title: String,
    pub description: String,
    #[serde(skip_deserializing)]
    pub create_at: String,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub title: String,
    pub description: String,
}
