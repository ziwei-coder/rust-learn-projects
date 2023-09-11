use crate::apis::client::GptClient;
use crate::helpers::{env::ENV, gpt};
use crate::models::general::llm::{ChatCompletion, Message};

//Call Large Language Model (i.e. GPT-4)
pub async fn call_gpt(messages: Vec<Message<'_>>) {
    dotenv::dotenv().ok();

    // Create Client
    let client = GptClient::new().client();

    // Create chat completion
    let model = ENV::OPEN_AI_MODEL.value();
    let chat_completion = ChatCompletion::new(&model, messages, 0.1);

    dbg!(&chat_completion);
    dbg!(&client);

    // Troubleshooting
    let res_raw = client
        .post(gpt::get_completions_url())
        .json(&chat_completion)
        .send()
        .await
        .unwrap();

    dbg!(res_raw.text().await.unwrap());
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
