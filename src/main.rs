// To use the below bot, you need a file named .bot.env in the root of the project with the
// following fields:
// openai_api_key="your openai api key"
// discord_app_id=your discord app id
// discord_token="your discord bot token"
use chatgpt::prelude::*;
use chatgpt::types::CompletionResponse;
use crab_gpt_bot::creds::CrabCredentials;
use serenity::{
    async_trait,
    framework::standard::StandardFramework,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler {
    bot_client: ChatGPT,
}

impl Handler {
    pub fn new(bot_creds: CrabCredentials) -> Self {
        let bot_client = ChatGPT::new(bot_creds.openai_api_key)
            .expect("[error] Unable to create ChatGPT client");
        Self { bot_client }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.to_lowercase().starts_with("yo crabby") {
            let prompt = msg
                .content
                .to_lowercase()
                .trim_start_matches("yo crabby")
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
    let creds = CrabCredentials::new();
    let handler = Handler::new(creds.clone());
    let framework = StandardFramework::new().configure(|c| c.prefix("yo crabby"));
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
