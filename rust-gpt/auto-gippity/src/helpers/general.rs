use reqwest::{Client, Error};
use std::fs;

use crate::constants::{API_SCHEMA_PATH, CODE_TEMPLATE_PATH, EXEC_MAIN_PATH};

/// Check wether url is valid
pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, Error> {
    let res = client.get(url).send().await?;
    Ok(res.status().as_u16())
}

/// Get code template
pub fn read_code_template_contents() -> String {
    let path = String::from(CODE_TEMPLATE_PATH);
    fs::read_to_string(path).expect("Failed to read code template")
}

/// Save New Backend code
pub fn save_backend_code(contents: &str) {
    let path = String::from(EXEC_MAIN_PATH);
    fs::write(path, contents).expect("Failed to write backend code")
}

/// Save Json API Endpoint Schema
pub fn save_json_api_endpoints(api_endpoints: &str) {
    let path = String::from(API_SCHEMA_PATH);
    fs::write(path, api_endpoints).expect("Failed to write API endpoints to file")
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::apis::client::GptClient;
    use crate::helpers::gpt::get_completions_url;

    #[tokio::test]
    async fn test_check_status_code() {
        dotenv::dotenv().ok();

        let client = GptClient::client().unwrap();
        let url = get_completions_url();
        let code = check_status_code(&client, &url).await;

        match code {
            Ok(status_code) => dbg!(status_code),
            Err(_) => panic!("check_status_code failed"),
        };
    }
}
