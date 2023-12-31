use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::log::private::error;
use rocket::response::status;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use rocket::{delete, fairing, get, post, put, routes, Build, Rocket};

use rocket_db_pools::{Connection, Database};

mod models;
mod repositories;

use models::NewProduct;
use repositories::ProductsRepo;

#[derive(Database)]
#[database("sqlite_products")]
struct DB(sqlx::SqlitePool);

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

#[get("/<id>")]
async fn view_product(mut db: Connection<DB>, id: i64) -> ResResult {
    let product = ProductsRepo::find(&mut db, id).await;
    handle_query(product)
}

#[get("/")]
async fn get_products(mut db: Connection<DB>) -> ResResult {
    let products = ProductsRepo::find_all(&mut db).await;
    handle_query(products)
}

#[post("/", format = "json", data = "<new_product>")]
async fn create_product(mut db: Connection<DB>, new_product: Json<NewProduct>) -> ResResult {
    let result = ProductsRepo::create(&mut db, new_product.into_inner()).await;
    handle_query(result)
}

#[post("/list", format = "json", data = "<new_products>")]
async fn create_products(mut db: Connection<DB>, new_products: Json<Vec<NewProduct>>) -> ResResult {
    let result = ProductsRepo::create_all(&mut db, new_products.into_inner()).await;
    handle_query(result)
}

#[put("/<id>", format = "json", data = "<new_product>")]
async fn update_product(
    mut db: Connection<DB>,
    id: i64,
    new_product: Json<NewProduct>,
) -> ResResult {
    let result = ProductsRepo::update(&mut db, id, new_product.into_inner()).await;
    handle_query(result)
}

#[delete("/<id>")]
async fn delete_product(mut db: Connection<DB>, id: i64) -> Value {
    let product = ProductsRepo::delete(&mut db, id).await;

    match product {
        Ok(data) => json!(data),
        Err(e) => json!(e.to_string()),
    }
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match DB::fetch(&rocket) {
        Some(db) => match sqlx::migrate!().run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLX database: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .attach(DB::init())
        .attach(AdHoc::try_on_ignite("SQLX Migrations", run_migrations))
        .mount(
            "/product",
            routes![
                view_product,
                get_products,
                create_product,
                create_products,
                update_product,
                delete_product
            ],
        )
        .launch()
        .await?;

    Ok(())
}
