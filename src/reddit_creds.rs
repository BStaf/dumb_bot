use std::fs;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct RedditApiCreds {
    pub client_id: String,
    pub client_secret: String,
    pub username: String,
    pub password: String,
}

pub fn get_creds_file(path: &str) -> String{
    fs::read_to_string(path)
        .expect("failed to read credential file.")
}

pub fn get_creds_from_json(jsonw: String) -> serde_json::Result<RedditApiCreds> {
    serde_json::from_str(&jsonw
        .replace("\n", "")
        .replace(" ", ""))
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn get_creds_from_json_test() {
        let str = r#"{
            "client_id": "test_client_id",
            "client_secret": "test_client_secert",
            "username": "test_username",
            "password": "test_password"
        }"#.to_string();

        let expected = RedditApiCreds{
            client_id: String::from("test_client_id"),
            client_secret: String::from("test_client_secert"),
            username: String::from("test_username"),
            password: String::from("test_password")
        };

        let creds = get_creds_from_json(str).expect("oops");

        assert_eq!(creds.client_id,expected.client_id);
        assert_eq!(creds.client_secret,expected.client_secret);
        assert_eq!(creds.username,expected.username);
        assert_eq!(creds.password,expected.password);
    }
}