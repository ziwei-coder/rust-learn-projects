use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;

use crate::helpers::{env::ENV, error::BoxError};

pub(super) struct GptClient {
    headers: HeaderMap,
}

impl GptClient {
    pub fn client() -> Result<Client, BoxError> {
        let mut gpt = Self {
            headers: HeaderMap::new(),
        };

        gpt.set_gpt_headers()?;

        Client::builder()
            .default_headers(gpt.headers)
            .build()
            .map_err(|e| -> BoxError { Box::new(e) })
    }

    fn set_gpt_headers(&mut self) -> Result<(), BoxError> {
        let api_key = format!("Bearer {}", ENV::OPEN_AI_KEY.value());
        self.set_header("Content-Type", "application/json")?;
        self.set_header("Authorization", &api_key)?;
        Ok(())
    }

    fn set_header(&mut self, key: &'static str, val: &str) -> Result<(), BoxError> {
        let val = HeaderValue::from_str(val).map_err(|e| -> BoxError { Box::new(e) })?;
        self.headers.insert(key, val);
        Ok(())
    }
}
