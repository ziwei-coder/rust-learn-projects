use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use rocket::serde::Serialize;
use rocket::{delete, get, launch, routes};

use rocket_db_pools::{Connection, Database};

mod models;
mod repositories;

use repositories::ProductsRepo;

#[derive(Database)]
#[database("sqlite_products")]
struct DB(sqlx::SqlitePool);

#[derive(sqlx::FromRow, Serialize)]
#[serde(crate = "rocket::serde")]
struct Product {
    id: i32,
    title: String,
    description: String,
}

#[get("/")]
async fn get_products(mut db: Connection<DB>) -> Value {
    let products = ProductsRepo::find_all(&mut db).await;

    match products {
        Ok(data) => json!(data),
        Err(e) => json!(e.to_string()),
    }
}

#[get("/<id>")]
async fn get_product(mut db: Connection<DB>, id: i32) -> Value {
    let product = ProductsRepo::find(&mut db, id).await;

    match product {
        Ok(data) => json!(data),
        Err(e) => json!(e.to_string()),
    }
}

#[delete("/<id>")]
async fn delete_product(mut db: Connection<DB>, id: i32) -> Value {
    let product = ProductsRepo::delete(&mut db, id).await;

    match product {
        Ok(data) => json!(data),
        Err(e) => json!(e.to_string()),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DB::init())
        .mount("/", routes![get_products, get_product, delete_product])
}
