use rocket::{get, routes};

// Rocket 路由转发, rank 越小优先级越大
#[get("/<id>", rank = 3)]
fn get_user(id: i32) -> String {
    format!("Get user 1: {id}")
}

#[get("/<id>", rank = 2)]
fn get_user2(id: bool) -> String {
    format!("Get user 2: {id}")
}

#[get("/<id>", rank = 1)]
fn get_user3(id: &str) -> String {
    format!("Get user 3: {id}")
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .mount("/user", routes![get_user, get_user2, get_user3])
        .launch()
        .await?;

    Ok(())
}
