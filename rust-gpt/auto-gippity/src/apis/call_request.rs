use crate::apis::client::GptClient;
use crate::helpers::{env::ENV, error::BoxError, gpt};
use crate::models::general::llm::{ChatCompletion, Message};

//Call Large Language Model (i.e. GPT-4)
pub async fn call_gpt(messages: Vec<Message<'_>>) -> Result<String, BoxError> {
    dotenv::dotenv().ok();

    // Create Client
    let client = GptClient::client()?;

    // Create chat completion
    let model = ENV::OPEN_AI_MODEL.value();
    let chat_completion = ChatCompletion::new(&model, messages, 0.1);

    // Troubleshooting
    let res_raw = client
        .post(gpt::get_completions_url())
        .json(&chat_completion)
        .send()
        .await?;

    Ok("Some string".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_call_to_openai() {
        let message = Message::new(
            "user",
            "Hi there, this is a test. Give me a short response.",
        );

        let messages = vec![message];

        call_gpt(messages).await;
    }
}
