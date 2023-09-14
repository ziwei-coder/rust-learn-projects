use crate::apis::client::GptClient;
use crate::helpers::{env::ENV, error::BoxError, gpt};
use crate::models::general::llm::{ChatCompletion, Message};

//Call Large Language Model (i.e. GPT-4)
pub async fn call_gpt(messages: Vec<Message>) -> Result<String, BoxError> {
    dotenv::dotenv().ok();

    // Create Client
    let client = GptClient::client()?;

    // Create chat completion
    let model = ENV::OPEN_AI_MODEL.value();
    let chat_completion = ChatCompletion::new(model, messages, 0.1);

    // Troubleshooting
    let url = gpt::get_completions_url();
    let res = GptClient::get_response(client, &url, &chat_completion).await?;

    let content = res
        .choices
        .get(0)
        .map(|c| c.message.content.clone())
        .ok_or("Empty response")?;

    Ok(content)
}

#[cfg(test)]
mod tests {
    use crate::helpers::gpt::Role;

    use super::*;

    ///! Run this test will cause money
    #[tokio::test]
    async fn test_call_to_openai() {
        let message = Message::new(
            Role::User.value(),
            "Hi there, this is a test. Give me a short response.".to_string(),
        );

        let messages = vec![message];

        let res = call_gpt(messages).await;

        if let Ok(res_str) = res {
            dbg!(res_str);
        } else {
            panic!("Call gpt failure");
        }
    }
}
