use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;

use crate::helpers::env::ENV;

pub(super) struct GptClient {
    headers: HeaderMap,
}

impl GptClient {
    pub fn client(self) -> Client {
        Client::builder()
            .default_headers(self.headers)
            .build()
            .unwrap()
    }

    pub fn new() -> Self {
        let mut gpt = Self {
            headers: HeaderMap::new(),
        };

        gpt.set_content_type();
        gpt.set_api_key_header();

        gpt
    }

    fn set_content_type(&mut self) {
        self.headers.insert(
            "Content-Type",
            HeaderValue::from_str("application/json").unwrap(),
        );
    }

    fn set_api_key_header(&mut self) {
        // Extract API Key information
        let api_key = &ENV::OPEN_AI_KEY.value();

        self.headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {api_key}")).unwrap(),
        );
    }

    #[allow(unused)]
    fn set_api_org_header(&mut self) {
        // Extract API Key information
        let api_key = &ENV::OPEN_AI_ORG.value();

        self.headers.insert(
            "OpenAI-organization",
            HeaderValue::from_str(api_key).unwrap(),
        );
    }
}
