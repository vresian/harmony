use adw::gtk::gdk::Texture;
use adw::glib::Bytes;
use tokio::runtime::Runtime;
use std::sync::OnceLock;

pub fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| {
        Runtime::new().expect("Setting up tokio runtime needs to succeed.")
    })
}

pub struct DiscordConnection {
    pub token: String
}

impl DiscordConnection {
    pub fn new(user_authorization_token: String) -> Self {
        DiscordConnection { token: user_authorization_token }
    }

    pub async fn init(&self) -> Result<serde_json::Value, String> {
        if self.token.is_empty() { return Err(String::from("Authorization token cannot be empty")) }

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

    pub async fn get_profile_picture(avatar: String, user_id: String, size: i16) -> Result<Texture, std::io::Error> {
        let client = reqwest::Client::new();        
        let avatar_url = format!("https://cdn.discordapp.com/avatars/{user_id}/{avatar}.jpg?size={}", size);

        let response = client
            .get(avatar_url)
            .send()
            .await;
        
        let image_bytes = response.unwrap().bytes().await.unwrap().to_vec();

        Ok(Texture::from_bytes(&Bytes::from(&image_bytes)).unwrap())
    }
}
