use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, sqlx::FromRow)]
#[serde(crate = "rocket::serde")]
pub struct Product {
    pub id: i64,
    pub title: String,
    pub description: String,
}

#[derive(Deserialize, sqlx::FromRow)]
#[serde(crate = "rocket::serde")]
pub struct NewProduct {
    pub title: String,
    pub description: Option<String>,
}
