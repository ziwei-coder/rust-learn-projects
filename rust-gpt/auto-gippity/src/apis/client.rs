use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;

use crate::helpers::{env::OpenAIEnv, error::BoxError};
use crate::models::general::llm::{APIResponse, ChatCompletion};

pub struct GptClient {
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

    pub async fn get_response(
        client: Client,
        url: &str,
        chat_completion: &ChatCompletion,
    ) -> Result<APIResponse, BoxError> {
        let res: APIResponse = client
            .post(url)
            .json(chat_completion)
            .send()
            .await
            .map_err(|e| -> BoxError { Box::new(e) })?
            .json()
            .await?;

        Ok(res)
    }
}

impl GptClient {
    fn set_gpt_headers(&mut self) -> Result<(), BoxError> {
        let api_key = format!("Bearer {}", OpenAIEnv::Key.value());
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
