use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug, Clone)]
pub struct CrabCredentials {
    pub discord_app_id: u64,
    pub discord_token: String,
    pub discord_name: String,
    pub openai_api_key: String,
}

impl CrabCredentials {
    pub fn new() -> Self {
        let file = fs::read_to_string(".bot.env").expect("[error] Unable to read .bot.env file");
        toml::from_str(&file).expect("[error] Unable to parse .bot.env file")
    }
}
