pub struct DiscordConnection {
    pub token: String
}

impl DiscordConnection {
    pub fn new(user_authorization_token: String) -> Self {
        DiscordConnection { token: user_authorization_token }
    }

    pub async fn init(&self) -> Result<serde_json::Value, String> {
        let client = reqwest::Client::new();

        let response = client
            .get("https://discord.com/api/v9/users/@me")
            .header(reqwest::header::AUTHORIZATION, &self.token)
            .send()
            .await;

        if !response.is_ok() { return Err(String::from("Couldn't make a GET request")) }

        let status = response.as_ref().unwrap().status();
        if status.is_client_error() { return Err(String::from("Invalid authorization token")) }
        if status.is_server_error() { return Err(String::from("Encountered a discord server error")) }
        
        let response_text = response.unwrap().text().await.unwrap();
        let json_data = serde_json::from_str(response_text.as_str()).unwrap();

        Ok(json_data)
    }
}
