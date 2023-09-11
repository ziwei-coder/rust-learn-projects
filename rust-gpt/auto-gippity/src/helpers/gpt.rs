use super::env::ENV;

pub fn get_completions_url() -> String {
    let base_url = ENV::OPEN_AI_BASE_URL.value();
    format!("{base_url}v1/chat/completions")
}
