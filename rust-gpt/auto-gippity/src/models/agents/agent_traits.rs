use async_trait::async_trait;
use std::fmt::Debug;

use crate::models::agent_basic::basic_agent::BasicAgent;
use crate::models::agents::agent_structs::FactSheet;

#[async_trait]
pub trait SpecialFunctions: Debug {
    /// Used to that manager can get attribute from agents
    fn get_attributes_from_agent(&self) -> &BasicAgent;

    /// This function will allow agents to execute their logic
    async fn execute(
        &mut self,
        factsheet: &mut FactSheet,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
