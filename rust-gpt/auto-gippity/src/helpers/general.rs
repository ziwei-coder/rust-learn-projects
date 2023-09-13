use reqwest::{Client, Error};

/// Check wether url is valid
pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, Error> {
    let res = client.get(url).send().await?;
    Ok(res.status().as_u16())
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
