mod reddit_creds;
mod reddit_auth;

#[tokio::main]
async fn main() -> Result<(), String> {
    let access_token_url = "https://www.reddit.com/api/v1/access_token";

    let reddit_creds = reddit_creds::get_creds_from_json(
        reddit_creds::get_creds_file("creds.json")
    ).expect("failed to get creds");

    let token = reddit_auth::get_bearer_token(&access_token_url, &reddit_creds).await?;

    println!("Status: {}", token);

    reddit_auth::do_stuff(token).await;

    Ok(())
}
