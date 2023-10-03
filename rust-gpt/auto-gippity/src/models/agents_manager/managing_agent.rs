use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;
use crate::get_function_string;
use crate::helpers::gpt::ai_task_request;
use crate::models::agent_basic::basic_agent::BasicAgent;
use crate::models::agent_basic::basic_traits::BasicTraits;
use crate::models::agents::agent_architect::AgentSolutionArchitect;
use crate::models::agents::agent_structs::Factsheet;
use crate::models::agents::agent_traits::SpecialFunctions;

#[derive(Debug)]
pub struct ManagingAgent {
    attributes: BasicAgent,
    factsheet: Factsheet,
    agents: Vec<Box<dyn SpecialFunctions>>,
}

impl ManagingAgent {
    pub async fn new(user_request: &str) -> Self {
        let position = "Project Manager".to_string();
        let objective =
            "Manage agent who are building an excellent website for the user".to_string();

        let attributes = BasicAgent::new(objective, position.clone());

        let project_description = ai_task_request(
            user_request,
            &position,
            get_function_string!(convert_user_input_to_goal),
            convert_user_input_to_goal,
        )
        .await;

        let agents = vec![];

        let factsheet = Factsheet::new(project_description);

        Self {
            attributes,
            agents,
            factsheet,
        }
    }

    fn add_agent(&mut self, agent: Box<dyn SpecialFunctions>) {
        self.agents.push(agent)
    }

    fn create_agents(&mut self) {
        self.add_agent(Box::new(AgentSolutionArchitect::new()));

        // TODO add backend agent
    }

    pub async fn execute_projects(&mut self) {
        self.create_agents();

        for agent in &mut self.agents {
            let agent_response = agent.execute(&mut self.factsheet).await;
            let agent_info = agent.get_attributes_from_agent();

            dbg!(agent_info);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_managing_agent() {
        let user_request="Need a full stack app that fetching and tracks my fitness progress. Need to include timezone!";
        let mut manage_agent = ManagingAgent::new(user_request).await;
        manage_agent.execute_projects().await;

        dbg!(manage_agent.factsheet);
    }
}
