pub enum OpenAIEnv {
    BaseUrl,
    Key,
    Model,
    #[allow(dead_code)]
    Org,
}

impl OpenAIEnv {
    pub fn value(&self) -> String {
        match self {
            OpenAIEnv::BaseUrl => Self::get_variable("OPEN_AI_BASE_URL"),
            OpenAIEnv::Key => Self::get_variable("OPEN_AI_KEY"),
            OpenAIEnv::Model => Self::get_variable("OPEN_AI_MODEL"),
            OpenAIEnv::Org => Self::get_variable("OPEN_AI_ORG"),
        }
    }

    fn get_variable(key: &str) -> String {
        let msg = format!("{key} not found in environment variables!");
        std::env::var(key).expect(&msg)
    }
}
