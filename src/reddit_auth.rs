use crate::reddit_creds;

use reqwest::{Response, StatusCode};
use reqwest::header::USER_AGENT;

use serde::Deserialize;

pub async fn do_stuff(token: String) {
    let url = "https://oauth.reddit.com/api/v1/me";
    let client = reqwest::Client::new();

    let request = client.get(url)
        .bearer_auth(token)
        .header("Content-Type", String::from("application/json"))
        .header(USER_AGENT, String::from("curl/7.88.1"));

    let response = request.send().await.expect("oops");

    println!("Status: {}", response.text().await.expect("oops2"));
}

pub async fn get_bearer_token(access_token_url: &str, api_creds: &reddit_creds::RedditApiCreds) -> Result<String, String> {
    
    let form_data = get_token_request_form_data(&api_creds.username, &api_creds.password);

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

    get_access_token_from_reponse(response).await
}

#[derive(Deserialize)]
struct AccessTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub scope: String,
}

fn get_token_request_form_data<'a>(username :&'a str, password :&'a str) -> [(&'a str, &'a str); 3]{
    [
        ("grant_type", "password"),
        ("username", username),
        ("password", password)
    ]
}

async fn get_access_token_from_reponse(response :Response) -> Result<String, String> {
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