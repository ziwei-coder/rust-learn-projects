use std::io::{stdin, stdout, Error};

use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};

#[derive(Debug, PartialEq)]
pub enum PrintCommand {
    AICall,
    UnitTest,
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_pos: &str, agent_statement: &str) {
        let mut stdout = stdout();

        // Decide ont the print color
        let statement_color = match self {
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Magenta,
        };

        // print the agent statement in a specific color
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent: {agent_pos}: ");

        // Make selected color
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{agent_statement}");

        // Reset color
        stdout.execute(ResetColor).unwrap();
    }
}

// Get user request
pub fn get_user_response(question: &str) -> Result<String, Error> {
    let mut stdout = stdout();

    // Print question in a special color
    stdout.execute(SetForegroundColor(Color::Blue))?;
    println!();
    println!("{}", question);

    // Reset color
    stdout.execute(ResetColor)?;

    // Read user input
    let mut user_response = String::new();
    stdin().read_line(&mut user_response)?;

    // trim whitespace and return
    Ok(user_response.trim().to_string())
}

/// Get user response that is safe to execute
pub fn confirm_safe_code() -> bool {
    let mut stdout = stdout();

    loop {
        // Print the question in specified color
        stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
        println!();
        print!("WARNING: You are about to run code written entirely by AI. ");
        println!("Review you code and confirm you wish to continue.");

        // Reset color
        stdout.execute(ResetColor).unwrap();

        // Print Options with different colors
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        println!("[1] All good");
        stdout.execute(SetForegroundColor(Color::DarkRed)).unwrap();
        println!("[2] Lets stop this project");

        // Reset color
        stdout.execute(ResetColor).unwrap();

        // Read user input
        let mut human_response = String::new();
        stdin()
            .read_line(&mut human_response)
            .expect("Failed to read response");

        // Trim whitespace an convert to lowercase
        let human_response = human_response.trim().to_lowercase();

        // Match Response
        match human_response.as_str() {
            "1" | "y" | "ok" => return true,
            "2" | "n" | "no" => return false,
            _ => {
                println!("Invalid input. Please select '1' or '2'");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_agent_message() {
        PrintCommand::AICall
            .print_agent_message("Managing Agent", "Testing testing, processing something");
    }
}
