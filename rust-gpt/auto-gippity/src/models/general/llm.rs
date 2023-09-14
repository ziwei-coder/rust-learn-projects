use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl Message {
    pub fn new(role: String, content: String) -> Self {
        Self { role, content }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct ChatCompletion {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

impl ChatCompletion {
    pub fn new(model: String, messages: Vec<Message>, temperature: f32) -> Self {
        Self {
            model,
            messages,
            temperature,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct APIMessage {
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct APIChoice {
    pub message: APIMessage,
}

#[derive(Debug, Deserialize)]
pub struct APIResponse {
    pub choices: Vec<APIChoice>,
}
