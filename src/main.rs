mod reddit_creds;

use reqwest::{Error, Response};
use reqwest::header::USER_AGENT;

fn get_form_data<'a>(username :&'a str, password :&'a str) -> [(&'a str, &'a str); 3]{
    [
        ("grant_type", "password"),
        ("username", username),
        ("password", password)
    ]
}

async fn try_reddit(access_token_url: &str, api_creds: &reddit_creds::RedditApiCreds) -> Result<(), reqwest::Error> {
    
    let form_data = get_form_data(
        &api_creds.username, 
        &api_creds.password);

    let client = reqwest::Client::new();
    let request = client.post(access_token_url)
        .basic_auth(&api_creds.client_id, Some(&api_creds.client_secret))
        .header("Content-Type", String::from("application/json"))
        .header(USER_AGENT, String::from("curl/7.88.1"))
        .form(&form_data);

    let response = request.send()
        .await?;
    println!("Status: {}", response.status());
    //println!("{:?}", response.);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let access_token_url = "https://www.reddit.com/api/v1/access_token";

    let reddit_creds = reddit_creds::get_creds_from_json(
        reddit_creds::get_creds_file("creds.json")
    ).expect("failed to get creds");

    try_reddit(&access_token_url, &reddit_creds).await?;
    Ok(())
}
