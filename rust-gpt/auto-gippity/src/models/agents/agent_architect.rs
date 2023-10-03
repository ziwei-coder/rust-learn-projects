use async_trait::async_trait;

use crate::ai_functions::aifunc_architect::{print_project_scope, print_site_urls};
use crate::get_function_string;
use crate::helpers::gpt::ai_task_request_decode;
use crate::models::agent_basic::{
    basic_agent::{AgentState, BasicAgent},
    basic_traits::BasicTraits,
};

use super::agent_structs::{FactSheet, ProjectScope};
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
    async fn call_project_scope(&mut self, factsheet: &mut FactSheet) -> ProjectScope {
        let msg_context = factsheet.project_description.clone();

        let ai_response = ai_task_request_decode::<ProjectScope>(
            msg_context,
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
    async fn call_determine_external_urls(
        &mut self,
        factsheet: &mut FactSheet,
        msg_context: String,
    ) {
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
        factsheet: &mut FactSheet,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while self.attributes.get_state() != &AgentState::Finished {
            match self.attributes.get_state() {
                AgentState::Discovery => {
                    let project_scope = self.call_project_scope(factsheet).await;

                    // Confirm if external urls
                    if project_scope.is_external_urls_required {
                        self.call_determine_external_urls(
                            factsheet,
                            factsheet.project_description.clone(),
                        )
                        .await;
                    }
                }
                AgentState::UnitTesting => {}
                _ => self.attributes.update_state(AgentState::Finished),
            }
        }
        todo!()
    }
}
