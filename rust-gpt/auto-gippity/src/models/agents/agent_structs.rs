use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct RouteObject {
    pub is_route_dynamic: String,
    pub method: String,
    pub request_body: serde_json::Value,
    pub response: serde_json::Value,
    pub route: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ProjectScope {
    pub is_crud_required: bool,
    pub is_user_login_and_logout: bool,
    pub is_external_urls_required: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Factsheet {
    pub project_description: String,
    pub project_scope: Option<ProjectScope>,
    pub external_urls: Option<Vec<String>>,
    pub backend_code: Option<String>,
    pub api_end_point_schema: Option<Vec<RouteObject>>,
}

impl Factsheet {
    pub fn new(description: String) -> Self {
        Self {
            project_description: description,
            project_scope: None,
            external_urls: None,
            backend_code: None,
            api_end_point_schema: None,
        }
    }
}
