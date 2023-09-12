use std::io::{stdin, stdout, Error};

use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};

#[derive(Debug, PartialEq)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_pos: &str, agent_statement: &str) {
        let mut stdout = stdout();

        // Decide ont the print color
        let statement_color = match self {
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Magenta,
            Self::Issue => Color::Red,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_agent_message() {
        PrintCommand::AICall
            .print_agent_message("Managing Agent", "Testing testing, processing something");
    }
}
