#[macro_use] extern crate rocket;

use rocket_db_pools::{Database, Connection};
use rocket_db_pools::sqlx::{self, Row};

#[derive(Database)]
#[database("sqlite_logs")]
struct Logs(sqlx::SqlitePool);

#[get("/<id>")]
async fn read(mut db: Connection<Logs>, id: i64) -> Option<String> {
   sqlx::query("SELECT content FROM logs WHERE id = ?").bind(id)
       .fetch_one(&mut *db).await
       .and_then(|r| Ok(r.try_get(0)?))
       .ok()
}

#[launch]
fn rocket() -> _ {
   rocket::build().attach(Logs::init()).mount("/", routes![read])
}