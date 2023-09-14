pub enum ENV {
    OPEN_AI_BASE_URL,
    OPEN_AI_KEY,
    OPEN_AI_MODEL,
    OPEN_AI_ORG,
}

impl ENV {
    pub fn value(&self) -> String {
        match self {
            ENV::OPEN_AI_BASE_URL => Self::get_variable("OPEN_AI_BASE_URL"),
            ENV::OPEN_AI_KEY => Self::get_variable("OPEN_AI_KEY"),
            ENV::OPEN_AI_MODEL => Self::get_variable("OPEN_AI_MODEL"),
            ENV::OPEN_AI_ORG => Self::get_variable("OPEN_AI_ORG"),
        }
    }

    fn get_variable(key: &str) -> String {
        let msg = format!("{key} not found in environment variables!");
        std::env::var(key).expect(&msg)
    }
}
