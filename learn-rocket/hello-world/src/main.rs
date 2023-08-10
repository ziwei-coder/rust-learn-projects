use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::{delete, get, launch, post, put, routes};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Person<'a> {
	id: usize,
	name: &'a str,
	age: u8,
}

#[get("/")]
fn index<'a>() -> Json<Person<'a>> {
	Json(Person {
		id: 0,
		name: "Tome",
		age: 9,
	})
}

#[get("/ex/<id>")]
async fn get_ex(id: usize) -> Value {
	json!(Person { id, name: "Joy", age: 10 })
}

// Restful Api
#[get("/ex")]
async fn get_exs() -> Value {
	json!(vec![
		Person {
			id: 0,
			name: "Tome",
			age: 9,
		},
		Person {
			id: 1,
			name: "Joy",
			age: 10
		},
	])
}

#[post("/ex", data = "<person>")]
fn post_ex(person: Json<Person<'_>>) -> Json<Person> {
	person
}

#[put("/ex/<id>", data = "<person>")]
async fn put_ex(id: usize, person: Json<Person<'_>>) -> Value {
	json!({
		"code":1,
		"person": Person {
			id,
			name:person.name,
			age:person.age
		}
	})
}

#[delete("/ex/<id>")]
async fn delete_ex(id: usize) -> Value {
	json!({"code":1})
}

#[launch]
fn rocket() -> _ {
	rocket::build()
		.mount("/", routes![index])
		.mount("/base", routes![get_exs, get_ex, post_ex, put_ex, delete_ex])
}
