use super::env::ENV;
use crate::{
    apis::call_request::call_gpt, helpers::command_line::PrintCommand,
    models::general::llm::Message,
};

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

pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_func_str = ai_func(func_input);

    // Extend the string to encourage only print the output
    let msg = format!(
        "FUNCTION: {ai_func_str}
    INSTRUCTION: You are a function printer. You ONLY print the result of the functions.
    Nothing else. No commentary. Here is the input to the function: {func_input}.
    Print out what the function will return."
    );

    // Return Message
    Message::new(Role::System.value(), msg)
}

pub async fn ai_task_request(
    msg_content: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: fn(&str) -> &'static str,
) -> String {
    // Extend AI function
    let extend_msg = extend_ai_function(function_pass, &msg_content);

    // Print current status
    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

    // Get LLM response
    let llm_response = call_gpt(vec![extend_msg.clone()]).await;

    // Return success or try again
    match llm_response {
        Ok(res) => res,
        Err(_) => call_gpt(vec![extend_msg])
            .await
            .expect("Failed twice to call LLM"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn test_get_completions_url() {
        dotenv::dotenv().ok();
        let url = get_completions_url();

        dbg!(&url);
        assert!(url.len() > "v1/chat/completions".len());
    }

    #[test]
    fn test_extend_ai_function() {
        let extended_msg = extend_ai_function(convert_user_input_to_goal, "dummy_variable");
        dbg!(extended_msg.content);
        assert_eq!(extended_msg.role, Role::System.value());
    }

    ///! Run this test will cause money
    #[tokio::test]
    async fn test_ai_task_request() {
        let ai_func_param =
            "Build me a web server for making stock price api requests.".to_string();

        let res = ai_task_request(
            ai_func_param,
            "Managing Agent",
            "Defining user requirements",
            convert_user_input_to_goal,
        )
        .await;

        assert!(res.len() > 20);
    }
}
