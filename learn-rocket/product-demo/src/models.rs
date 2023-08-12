use rocket::serde::Serialize;

#[derive(Serialize, sqlx::FromRow)]
#[serde(crate = "rocket::serde")]
pub struct Product {
    id: i32,
    title: String,
    description: String,
}
