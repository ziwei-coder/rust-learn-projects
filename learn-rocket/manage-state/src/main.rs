use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::sync::Mutex;
use rocket::{delete, get, post, put, routes, State};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
struct Person {
    id: usize,
    name: String,
    age: u8,
}

// map -> mutex -> state
type PersonItems = Mutex<HashMap<usize, Person>>;
type Context<'s> = &'s State<PersonItems>;

#[get("/person/<id>")]
async fn get_person(id: usize, state: Context<'_>) -> Json<Person> {
    let people_map = state.lock().await;

    let default_person = Person {
        id: 0,
        name: "_".to_string(),
        age: 0,
    };

    if id == 0 {
        return Json(default_person);
    }

    match people_map.get(&id) {
        Some(p) => Json(p.clone()),
        None => Json(default_person),
    }
}

#[post("/person", format = "json", data = "<person>")]
async fn create_person(person: Json<Person>, state: Context<'_>) -> Value {
    let mut people_map = state.lock().await;
    let new_person = person.into_inner();

    match people_map.contains_key(&new_person.id) {
        true => json!({"res":"error"}),
        false => {
            people_map.insert(new_person.id, new_person);
            json!({"res":"ok"})
        }
    }
}

#[put("/person/<id>", format = "json", data = "<person>")]
async fn update_person(id: usize, person: Json<Person>, state: Context<'_>) -> Value {
    let mut people_map = state.lock().await;
    let new_person = person.into_inner();

    if id != new_person.id {
        return json!({"res":"error"});
    }

    match people_map.contains_key(&new_person.id) {
        true => {
            people_map.insert(new_person.id, new_person);
            json!({"res":"ok"})
        }
        false => json!({"res":"error"}),
    }
}

#[delete("/person/<id>")]
async fn delete_person(id: usize, state: Context<'_>) -> Value {
    let mut people_map = state.lock().await;

    match people_map.contains_key(&id) {
        true => {
            people_map.remove(&id);
            json!({"res":"ok"})
        }
        false => json!({"res":"error"}),
    }
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rocket::build()
        .manage(PersonItems::new(HashMap::new()))
        .mount(
            "/",
            routes![get_person, create_person, update_person, delete_person],
        )
        .launch()
        .await?;

    Ok(())
}
