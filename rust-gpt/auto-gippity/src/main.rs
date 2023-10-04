mod ai_functions;
mod apis;
mod constants;
mod helpers;
mod macros;
mod models;

use helpers::command_line;
use models::agents_manager::managing_agent::ManagingAgent;

#[tokio::main]
async fn main() {
    let user_req = command_line::get_user_response("What web server are you building today?")
        .expect("Failed to get user response");

    let mut manager_agent = ManagingAgent::new(&user_req).await;
    manager_agent.execute_projects().await;

    dbg!(user_req);
}
