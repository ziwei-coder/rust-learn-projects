use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{json, Json, Value};
use rocket::{catch, catchers, delete, get, post, put, routes};
use rocket_sync_db_pools::database;

mod basic_auth;
mod models;
mod repositories;
mod schema;

use basic_auth::BasicAuth;
use models::{NewProduct, Product};
use repositories::ProductRepo;

#[database("sqlite_path")]
struct DB(diesel::SqliteConnection);

type ResResult = Result<Value, status::Custom<Value>>;

fn handle_query<T, E>(result: Result<T, E>) -> ResResult
where
    T: rocket::serde::Serialize,
    E: std::fmt::Display,
{
    result
        .map(|data| json!(data))
        .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
}

#[get("/")]
async fn get_products(_auth: BasicAuth, db: DB) -> ResResult {
    db.run(|conn| {
        let products = ProductRepo::find_all(conn);
        handle_query(products)
    })
    .await
}

#[get("/<id>")]
async fn view_product(_auth: BasicAuth, db: DB, id: i32) -> ResResult {
    db.run(move |conn| {
        let product = ProductRepo::find(conn, id);
        handle_query(product)
    })
    .await
}

#[post("/", format = "json", data = "<new_product>")]
async fn create_product(_auth: BasicAuth, db: DB, new_product: Json<NewProduct>) -> ResResult {
    db.run(|conn| {
        let result = ProductRepo::create(conn, new_product.into_inner());
        handle_query(result)
    })
    .await
}

#[put("/<_id>", format = "json", data = "<product>")]
async fn update_product(_auth: BasicAuth, db: DB, _id: i32, product: Json<Product>) -> ResResult {
    db.run(move |conn| {
        let result = ProductRepo::update(conn, product.into_inner());
        handle_query(result)
    })
    .await
}

#[delete("/<id>")]
async fn delete_product(_auth: BasicAuth, db: DB, id: i32) -> ResResult {
    db.run(move |conn| {
        let result = ProductRepo::delete(conn, id);
        handle_query(result)
    })
    .await
}

#[catch(404)]
async fn not_found_url() -> Value {
    json!("not found!")
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rocket::build()
        .mount(
            "/product",
            routes![
                get_products,
                view_product,
                create_product,
                update_product,
                delete_product
            ],
        )
        .register("/", catchers![not_found_url])
        .attach(DB::fairing())
        .launch()
        .await?;

    Ok(())
}
