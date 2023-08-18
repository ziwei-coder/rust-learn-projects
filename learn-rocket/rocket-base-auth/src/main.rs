use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome, Request};
use rocket::{get, routes};

struct BaseAuth {
    username: String,
    password: String,
}

impl BaseAuth {
    pub fn from_header_auth(auth: &str) -> Option<Self> {
        let split = auth.split_whitespace().collect::<Vec<_>>();

        if split.len() != 2 {
            return None;
        }

        if split[0] != "Basic" {
            return None;
        }

        Self::from_base64_encode(split[1])
    }

    fn from_base64_encode(split: &str) -> Option<BaseAuth> {
        todo!()
    }
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for BaseAuth {
    type Error = ();

    async fn from_request(req: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let header_auth = req.headers().get_one("Authorization");

        if let Some(header_auth) = header_auth {}

        Outcome::Failure((Status::Unauthorized, ()))
    }
}

#[get("/hello")]
async fn index() -> String {
    "hello world!".to_string()
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build().mount("/", routes![index]).launch().await?;

    Ok(())
}
