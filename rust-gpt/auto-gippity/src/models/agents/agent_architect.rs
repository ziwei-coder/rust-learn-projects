use std::time::Duration;

use async_trait::async_trait;
use reqwest::Client;

use crate::ai_functions::aifunc_architect::{print_project_scope, print_site_urls};
use crate::get_function_string;
use crate::helpers::command_line::PrintCommand;
use crate::helpers::general::check_status_code;
use crate::helpers::gpt::ai_task_request_decode;
use crate::models::agent_basic::{
    basic_agent::{AgentState, BasicAgent},
    basic_traits::BasicTraits,
};

use super::agent_structs::{Factsheet, ProjectScope};
use super::agent_traits::SpecialFunctions;

#[derive(Debug)]
pub struct AgentSolutionArchitect {
    attributes: BasicAgent,
}

impl AgentSolutionArchitect {
    pub fn new() -> Self {
        let objective =
            "Gathers information and design solutions for website development".to_string();
        let position = "Solutions Architect".to_string();
        let attributes = BasicAgent::new(objective, position);
        Self { attributes }
    }

    /// Retrieve Project scope
    async fn call_project_scope(&mut self, factsheet: &mut Factsheet) -> ProjectScope {
        let msg_context = factsheet.project_description.clone();

        let ai_response = ai_task_request_decode::<ProjectScope>(
            &msg_context,
            self.attributes.get_position(),
            get_function_string!(print_project_scope),
            print_project_scope,
        )
        .await;

        factsheet.project_scope = Some(ai_response.clone());
        self.attributes.update_state(AgentState::Finished);
        ai_response
    }

    /// Retrieve Project external urls
    async fn call_determine_external_urls(&mut self, factsheet: &mut Factsheet, msg_context: &str) {
        let ai_response = ai_task_request_decode::<Vec<String>>(
            msg_context,
            self.attributes.get_position(),
            get_function_string!(print_site_urls),
            print_site_urls,
        )
        .await;

        factsheet.external_urls = Some(ai_response);
        self.attributes.update_state(AgentState::UnitTesting);
    }
}

#[async_trait]
impl SpecialFunctions for AgentSolutionArchitect {
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
                }
                AgentState::UnitTesting => {
                    self.handle_unit_testing_state(factsheet).await;
                }
                _ => self.attributes.update_state(AgentState::Finished),
            }
        }

        Ok(())
    }
}

impl AgentSolutionArchitect {
    async fn handle_discovery_state(&mut self, factsheet: &mut Factsheet) {
        let project_scope = self.call_project_scope(factsheet).await;

        // Confirm if external urls
        if project_scope.is_external_urls_required {
            self.call_determine_external_urls(
                factsheet,
                factsheet.project_description.clone().as_str(),
            )
            .await;
            self.attributes.update_state(AgentState::UnitTesting);
        }
    }

    async fn handle_unit_testing_state(&mut self, factsheet: &mut Factsheet) {
        let mut exclude_urls: Vec<String> = Vec::new();

        let client = Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();

        // find faulty urls
        let external_urls = factsheet.external_urls.clone();
        let urls = external_urls.expect("No URL Object on factsheet");

        for url in urls {
            let endpoint_str = format!("Testing URL Endpoint: {}", url);

            PrintCommand::UnitTest
                .print_agent_message(self.attributes.get_position(), &endpoint_str);

            // Perform URL Test
            match check_status_code(&client, &url).await {
                Ok(status_code) => {
                    if status_code != 200 {
                        exclude_urls.push(url);
                    }
                }
                Err(e) => {
                    println!("Error checking {url}: {e}");
                }
            }

            // Exclude any faulty urls
            if !exclude_urls.is_empty() {
                let new_urls: Vec<String> = factsheet
                    .external_urls
                    .as_ref()
                    .unwrap()
                    .iter()
                    .filter(|url| !exclude_urls.contains(url))
                    .cloned()
                    .collect();

                factsheet.external_urls = Some(new_urls);
            }

            // confirm done
            self.attributes.update_state(AgentState::Finished);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_solution_architect() {
        let mut agent = AgentSolutionArchitect::new();
        let mut factsheet =
            Factsheet::new("Build a full stack website that shows latest Forex prices".to_string());

        agent
            .execute(&mut factsheet)
            .await
            .expect("Unable to execute Solutions Architect Agent!");

        assert!(factsheet.project_scope.is_some());
        assert!(factsheet.external_urls.is_some());

        dbg!(factsheet);
    }
}
