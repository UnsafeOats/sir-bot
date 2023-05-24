use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::env;

#[derive(Deserialize, Debug, Clone)]
pub struct SirCredentials {
    pub discord_app_id: u64,
    pub discord_token: String,
    pub discord_name: String,
    pub openai_api_key: String,
    pub trigger_phrase: String,
    pub temperature: f32,
    pub engine: String,
}

impl SirCredentials {
    pub fn from<T: AsRef<PathBuf>>(env_file: T) -> Self {
        let file = fs::read_to_string(env_file.as_ref()).expect("[error] Unable to read bot config file");
        toml::from_str(&file).expect("[error] Unable to parse bot config file")
    }

    pub fn new() -> Self {
        SirCredentials {
            discord_app_id: env::var("DISCORD_APP_ID").expect("DISCORD_APP_ID not set").parse::<u64>().expect("DISCORD_APP_ID is not a valid u64"),
            discord_token: env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set"),
            discord_name: env::var("DISCORD_NAME").expect("DISCORD_NAME not set"),
            openai_api_key: env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set"),
            trigger_phrase: env::var("TRIGGER_PHRASE").unwrap_or("sir".to_string()),
            temperature: env::var("OPENAI_TEMPERATURE").unwrap_or("0.9".to_string()).parse::<f32>().expect("OPENAI_TEMPERATURE is not a valid f32"),
            engine: env::var("OPENAI_ENGINE").unwrap_or("Gpt4".to_string()),
        }
    }
}
