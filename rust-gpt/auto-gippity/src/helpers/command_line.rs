use std::io::{stdin, stdout, Error};

use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};

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
