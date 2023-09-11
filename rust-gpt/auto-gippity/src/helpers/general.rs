use crate::helpers::gpt::Role;
use crate::models::general::llm::Message;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn test_extend_ai_function() {
        let extended_msg = extend_ai_function(convert_user_input_to_goal, "dummy_variable");
        dbg!(extended_msg.content);
        assert_eq!(extended_msg.role, Role::System.value());
    }
}
