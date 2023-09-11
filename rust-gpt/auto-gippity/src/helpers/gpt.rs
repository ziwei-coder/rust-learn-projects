use super::env::ENV;

pub enum Role {
    User,
    System,
}

impl Role {
    pub fn value(&self) -> String {
        match self {
            Self::User => "user".to_string(),
            Self::System => "system".to_string(),
        }
    }
}

pub fn get_completions_url() -> String {
    let base_url = ENV::OPEN_AI_BASE_URL.value();
    format!("{base_url}v1/chat/completions")
}
