use async_trait::async_trait;
use reqwest::Client;
use std::process::{Command, Stdio};
use std::time::Duration;

use crate::ai_functions::aifunc_backend::{
    print_backend_webserver_code, print_fixed_code, print_improved_webserver_code,
    print_rest_api_endpoints,
};

use crate::constants::WEB_SERVER_PROJECT_PATH;
use crate::get_function_string;
use crate::helpers::command_line::{confirm_safe_code, PrintCommand::UnitTest};
use crate::helpers::general::{
    check_status_code, read_backend_code, read_code_template_contents, save_backend_code,
    save_json_api_endpoints,
};
use crate::helpers::gpt::ai_task_request;
use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agent_basic::basic_traits::BasicTraits;

use super::agent_structs::{Factsheet, RouteObject};
use super::agent_traits::SpecialFunctions;

#[derive(Debug)]
pub struct AgentBackendDeveloper {
    attributes: BasicAgent,
    bug_errors: Option<String>,
    bug_count: u8,
}

impl AgentBackendDeveloper {
    pub fn new() -> Self {
        let attributes = BasicAgent::new(
            "Develops backend code for webserver and json database".to_string(),
            "Backend Developer".to_string(),
        );
        Self {
            attributes,
            bug_errors: None,
            bug_count: 0,
        }
    }

    async fn call_initial_backend_code(&mut self, factsheet: &mut Factsheet) {
        let code_template = read_code_template_contents();

        let msg_context = format!(
            "CODE TEMPLATE: {} \n PROJECT_DESCRIPTION: {} \n",
            code_template, factsheet.project_description
        );

        let ai_response = ai_task_request(
            &msg_context,
            self.attributes.get_position(),
            get_function_string!(print_backend_webserver_code),
            print_backend_webserver_code,
        )
        .await;

        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);
    }

    async fn call_improve_backend_code(&mut self, factsheet: &mut Factsheet) {
        let msg_context = format!(
            "CODE TEMPLATE: {:?} \n PROJECT_DESCRIPTION: {} \n",
            factsheet.backend_code, factsheet.project_description
        );

        let ai_response = ai_task_request(
            &msg_context,
            self.attributes.get_position(),
            get_function_string!(print_improved_webserver_code),
            print_improved_webserver_code,
        )
        .await;

        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);
    }

    async fn call_fix_code_bugs(&mut self, factsheet: &mut Factsheet) {
        let msg_context = format!(
            "BROKEN CODE: {:?} \n ERROR_BUGS: {:?} \n",
            factsheet.backend_code, self.bug_errors
        );

        let ai_response = ai_task_request(
            &msg_context,
            self.attributes.get_position(),
            get_function_string!(print_fixed_code),
            print_fixed_code,
        )
        .await;

        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);
    }

    async fn call_extract_rest_api_endpoints(&self) -> String {
        let backend_code = read_backend_code();

        // Structure message content
        let msg_context = format!("CODE_INPUT: {}", backend_code);

        ai_task_request(
            &msg_context,
            self.attributes.get_position(),
            get_function_string!(print_rest_api_endpoints),
            print_rest_api_endpoints,
        )
        .await
    }
}

impl AgentBackendDeveloper {
    async fn handle_discovery_state(&mut self, factsheet: &mut Factsheet) {
        self.call_initial_backend_code(factsheet).await;
        self.attributes.update_state(AgentState::Working);
    }

    async fn handle_working_state(&mut self, factsheet: &mut Factsheet) {
        if self.bug_count == 0 {
            self.call_improve_backend_code(factsheet).await;
        } else {
            self.call_fix_code_bugs(factsheet).await;
        }

        self.attributes.update_state(AgentState::UnitTesting);
    }

    async fn handle_unit_testing_state(&mut self, factsheet: &mut Factsheet) -> AgentState {
        // Build and Test code
        let is_build_code_success = self.build_test_code();
        if !is_build_code_success {
            self.attributes.update_state(AgentState::Working);
            return AgentState::Working;
        }

        // Extract and testing api endpoints
        self.extract_test_api_endpoints(factsheet).await;

        self.attributes.update_state(AgentState::Finished);
        AgentState::Finished
    }

    fn build_test_code(&mut self) -> bool {
        self.ensure_safe_code();

        let agent_pos = self.attributes.get_position();

        // Build and Test code
        UnitTest.print_agent_message(agent_pos, "Backend Code Unit Testing: building project...");

        // Build code
        let build_backend_server = Command::new("cargo")
            .arg("build")
            .current_dir(WEB_SERVER_PROJECT_PATH)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .expect("Failed to run backend application");

        // Determine if build errors
        if build_backend_server.status.success() {
            self.bug_count = 0;
            UnitTest.print_agent_message(
                agent_pos,
                "Backend Code Unit Testing: Test server build successful...",
            );

            return true;
        }

        let error_arr = build_backend_server.stderr;
        let error_str = String::from_utf8(error_arr).unwrap();

        // update error stats
        self.bug_count += 1;
        self.bug_errors = Some(error_str);

        // Exit if too many bug
        if self.bug_count > 2 {
            UnitTest.print_agent_message(
                agent_pos,
                "Backend Code Unit Testing, Too many bugs found in code",
            );
            panic!("Error: Too many bugs");
        }

        // Pass back for rework
        false
    }

    fn ensure_safe_code(&self) {
        // Guard:: Ensure ai safety
        UnitTest.print_agent_message(
            self.attributes.get_position(),
            "Backend Code Unit Testing: Ensuring Safe Code",
        );

        if !confirm_safe_code() {
            panic!("Better go work on some AI alignment instead...");
        }
    }

    async fn extract_test_api_endpoints(&mut self, factsheet: &mut Factsheet) {
        let agent_pos = self.attributes.get_position();

        // Extract api endpoints
        let api_endpoints_str = self.call_extract_rest_api_endpoints().await;

        // Convert api endpoints into Values
        let api_endpoints: Vec<RouteObject> =
            serde_json::from_str(&api_endpoints_str).expect("Failed to decode API Endpoints");

        // Define endpoints to check
        let check_endpoints: Vec<RouteObject> = api_endpoints
            .iter()
            .filter(|&rout| rout.method == "get" && rout.is_route_dynamic == "false")
            .cloned()
            .collect();

        // store api endpoints
        factsheet.api_end_point_schema = Some(check_endpoints.clone());

        // Run backend application
        UnitTest.print_agent_message(
            agent_pos,
            "Backend Code Unit Testing: Starting web serve...",
        );

        let mut run_backend_server = Command::new("cargo")
            .arg("run")
            .current_dir(WEB_SERVER_PROJECT_PATH)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to run backend application");

        // Let user know testing on server will take place soon
        UnitTest.print_agent_message(
            agent_pos,
            "Backend Code Unit Testing: Launching tests on server in 5 seconds...",
        );

        tokio::time::sleep(Duration::from_secs(5)).await;

        // create client with timeout
        let client = Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();

        // check status code
        for endpoint in check_endpoints {
            // Confirm url testing
            let testing_msg = format!("Testing endpoint '{}'...", endpoint.route);
            UnitTest.print_agent_message(agent_pos, &testing_msg);

            // Test url
            let url = format!("http://localhost:8080{}", endpoint.route);

            match check_status_code(&client, &url).await {
                Ok(status_code) => {
                    if status_code != 200 {
                        let err_msg = format!(
                            "WARNING: Failed to call backend url endpoint {}",
                            endpoint.route
                        );

                        UnitTest.print_agent_message(agent_pos, &err_msg);
                    }
                }
                Err(e) => {
                    // kill server
                    run_backend_server
                        .kill()
                        .expect("Failed to kill backend web server");

                    let err_msg = format!("Error checking backend {e}");
                    UnitTest.print_agent_message(agent_pos, &err_msg);
                }
            }

            save_json_api_endpoints(&api_endpoints_str);

            UnitTest.print_agent_message(agent_pos, "Backend testing complete...");
            run_backend_server
                .kill()
                .expect("Failed to kill backend web server on completion")
        }
    }
}

#[async_trait]
impl SpecialFunctions for AgentBackendDeveloper {
    fn get_attributes_from_agent(&self) -> &BasicAgent {
        &self.attributes
    }

    async fn execute(
        &mut self,
        factsheet: &mut Factsheet,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while self.attributes.get_state() != &AgentState::Finished {
            match self.attributes.get_state() {
                AgentState::Discovery => {
                    self.handle_discovery_state(factsheet).await;
                    continue;
                }
                AgentState::Working => {
                    self.handle_working_state(factsheet).await;
                    continue;
                }
                AgentState::UnitTesting => {
                    let state = self.handle_unit_testing_state(factsheet).await;
                    if state == AgentState::Working {
                        continue;
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_backend_developer() {
        let mut agent = AgentBackendDeveloper::new();

        let factsheet_str = r#"
            {
                "project_description": "build a website that fetches and tracks fitness progress with timezone information",
                "project_scope": {
                    "is_crud_required": true,
                    "is_user_login_and_logout": true,
                    "is_external_urls_required": true
                },
                "external_urls": [
                    "http://worldtimeapi.org/api/timezone"
                ],
                "backend_code": null,
                "api_endpoint_schema": null
            }"#;

        let mut factsheet: Factsheet = serde_json::from_str(factsheet_str).unwrap();
        agent
            .execute(&mut factsheet)
            .await
            .expect("Failed to execute Backend Developer agent");
    }
}
