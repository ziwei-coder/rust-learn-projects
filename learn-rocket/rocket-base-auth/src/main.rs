use base64::engine::general_purpose;
use base64::Engine;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
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

    fn from_base64_encode(str: &str) -> Option<BaseAuth> {
        let decoded = general_purpose::STANDARD.decode(str).ok()?;
        let decoded_str = String::from_utf8(decoded).ok()?;
        let split = decoded_str.split(':').collect::<Vec<_>>();

        if split.len() != 2 {
            return None;
        }

        let (username, password) = (split[0].to_string(), split[1].to_string());

        Some(BaseAuth { username, password })
    }
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for BaseAuth {
    type Error = ();

    async fn from_request(req: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let header_auth = req.headers().get_one("Authorization");

        if let Some(header_auth) = header_auth {
            if let Some(auth) = Self::from_header_auth(header_auth) {
                return Outcome::Success(auth);
            }
        }

        Outcome::Failure((Status::Unauthorized, ()))
    }
}

#[get("/<hello>")]
async fn index(hello: &str, auth: BaseAuth) -> String {
    println!("username: {}, password: {}", auth.username, auth.password);
    format!("Hello {hello}")
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build().mount("/", routes![index]).launch().await?;

    Ok(())
}
