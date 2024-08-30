mod reddit_creds;

use reqwest::{Error, Response, StatusCode};
use reqwest::header::USER_AGENT;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub scope: String,
}

fn get_form_data<'a>(username :&'a str, password :&'a str) -> [(&'a str, &'a str); 3]{
    [
        ("grant_type", "password"),
        ("username", username),
        ("password", password)
    ]
}

async fn try_reddit(access_token_url: &str, api_creds: &reddit_creds::RedditApiCreds) -> Result<String, String> {
    
    let form_data = get_form_data(
        &api_creds.username, 
        &api_creds.password);

    let client = reqwest::Client::new();

    let request = client.post(access_token_url)
        .basic_auth(&api_creds.client_id, Some(&api_creds.client_secret))
        .header("Content-Type", String::from("application/json"))
        .header(USER_AGENT, String::from("curl/7.88.1"))
        .form(&form_data);

    let response = match request.send().await {
        Ok(r) => r,
        Err(e) => return Err(e.to_string())
    };

    match response.status() {
        StatusCode::OK => {
            let json = match response.json::<AccessTokenResponse>().await {
                Ok(t) => t,
                Err(e) => return Err(e.to_string())
            };
            Ok(json.access_token)
        },
        status_code => Err(format!("Retuned Status Code: {status_code}"))
    }
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let access_token_url = "https://www.reddit.com/api/v1/access_token";

    let reddit_creds = reddit_creds::get_creds_from_json(
        reddit_creds::get_creds_file("creds.json")
    ).expect("failed to get creds");

    let token = try_reddit(&access_token_url, &reddit_creds).await?;

    println!("Status: {}", token);

    Ok(())
}
