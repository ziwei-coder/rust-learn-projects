use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Message<'a> {
    role: &'a str,
    content: &'a str,
}

impl<'a> Message<'a> {
    pub fn new(role: &'a str, content: &'a str) -> Self {
        Self { role, content }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct ChatCompletion<'a> {
    model: &'a str,
    messages: Vec<Message<'a>>,
    temperature: f32,
}

impl<'a> ChatCompletion<'a> {
    pub fn new(model: &'a str, messages: Vec<Message<'a>>, temperature: f32) -> Self {
        Self {
            model,
            messages,
            temperature,
        }
    }
}
