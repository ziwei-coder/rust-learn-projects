mod ai_functions;
mod apis;
mod constants;
mod helpers;
mod models;

use helpers::command_line;

fn main() {
    let user_req = command_line::get_user_response("What web server are you building today?")
        .expect("Failed to get user response");

    dbg!(user_req);
}
