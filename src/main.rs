// To use the below bot, you need a file named .bot.env in the root of the project with the
// following fields:
// openai_api_key="your openai api key"
// discord_app_id=your discord app id
// discord_token="your discord bot token"
use chatgpt::prelude::*;
use chatgpt::types::CompletionResponse;
use crab_gpt_bot::creds::SirCredentials;
use serenity::{
    async_trait,
    framework::standard::StandardFramework,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler {
    bot_client: ChatGPT,
    trigger_phrase: String,
}

impl Handler {
    pub fn new(bot_creds: SirCredentials) -> Self {
        let bot_client = ChatGPT::new_with_config(
            bot_creds.openai_api_key,
            ModelConfigurationBuilder::default()
                .temperature(bot_creds.temperature)
                .engine(Self::map_engine(bot_creds.engine))
                .build()
                .expect("[error] Unable to create GPT ModelConfiguration"),
        ).expect("[error] Unable to create ChatGPT client");
        Self { bot_client, trigger_phrase: bot_creds.trigger_phrase }
    }

    pub fn map_engine(engine: String) -> ChatGPTEngine {
        match engine.as_str() {
            "Gpt35Turbo" => ChatGPTEngine::Gpt35Turbo,
            "Gpt35Turbo_0301" => ChatGPTEngine::Gpt35Turbo_0301,
            "Gpt4" => ChatGPTEngine::Gpt4,
            "Gpt4_32k" => ChatGPTEngine::Gpt4_32k,
            "Gpt4_0314" => ChatGPTEngine::Gpt4_0314,
            "Gpt4_32k_0314" => ChatGPTEngine::Gpt4_32k_0314,
            _ => ChatGPTEngine::Gpt35Turbo,
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.to_lowercase().starts_with(&self.trigger_phrase) {
            let prompt = msg
                .content
                .to_lowercase()
                .trim_start_matches(&self.trigger_phrase)
                .trim()
                .to_string();
            match self.bot_client.send_message(prompt).await {
                Ok(response) => Self::send_long_message(response, &msg, &ctx).await,
                Err(why) => {
                    println!("Error fetching OpenAI completion: {:?}", why);
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

impl Handler {
    async fn send_long_message(content: CompletionResponse, msg: &Message, ctx: &Context) {
        let chunk_size = 1999; // Discord character limit
        for chunk in content.message().content.chars().collect::<Vec<_>>().chunks(chunk_size) {
            let chunk_str = chunk.iter().collect::<String>();
            let mut remaining = &chunk_str[..];
            while !remaining.is_empty() {
                let end = remaining.char_indices()
                    .take(chunk_size)
                    .map(|(i, _)| i)
                    .last()
                    .unwrap_or(remaining.len());
                let chunk = &remaining[..end];
                if chunk.is_empty() {
                    break;
                } else {
                    remaining = &remaining[end..];
                    if let Err(why) = msg.channel_id.say(&ctx.http, &chunk).await {
                        println!("Error sending message: {:?}", why);
                    }
                }
            }
    }
}

}

#[tokio::main]
async fn main() {
    let creds = SirCredentials::new();
    let handler = Handler::new(creds.clone());
    let framework = StandardFramework::new().configure(|c| c.prefix(&handler.trigger_phrase));
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(creds.discord_token, intents)
        .application_id(creds.discord_app_id)
        .event_handler(handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
