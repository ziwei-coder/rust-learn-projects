use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use rocket::serde::Serialize;
use rocket::{futures, get, launch, routes};

use rocket_db_pools::{sqlx, Connection, Database};

use futures::stream::TryStreamExt;

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
async fn list(mut db: Connection<DB>) -> Value {
    let products = sqlx::query_as::<_, Product>("SELECT id, title, description FROM products")
        .fetch(&mut *db)
        .map_ok(|record| record)
        .try_collect::<Vec<_>>()
        .await;

    match products {
        Ok(data) => json!(data),
        Err(e) => json!(e.to_string()),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(DB::init()).mount("/", routes![list])
}
