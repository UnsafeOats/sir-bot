use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use csv::Writer;
use std::error::Error;
use std::path::Path;
use crate::openai_client::fetch_openai_completion;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("%crabby") {
            let prompt = msg.content.trim_start_matches("%crabby").trim();
            match fetch_openai_completion(prompt).await {
                Ok(completion) => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, &completion).await {
                        println!("Error sending message: {:?}", why);
                    }
                }
                Err(why) => {
                    println!("Error fetching OpenAI completion: {:?}", why);
                }
            }
        } else if msg.content.starts_with("%collect") {
            if let Err(why) = collect_messages(&ctx, &msg).await {
                println!("Error collecting messages: {:?}", why);
            }
        } else if msg.content.to_lowercase().contains("hello") {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Yarrr me matey, howdy!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn collect_messages(ctx: &Context, msg: &Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut writer = Writer::from_path(Path::new("collected_messages.csv"))?;

    let channels = msg.guild_id.unwrap().channels(&ctx.http).await?;
    for (_channel_id, channel) in channels {
        if let Some(messages) = channel.messages(&ctx.http, |retriever| retriever.limit(100)).await.ok() {
            for message in messages {
                writer.write_record(&[message.author.name, message.content])?;
            }
        }
    }

    writer.flush()?;
    msg.channel_id.say(&ctx.http, "Messages have been collected!").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = std::env::var("CRABGPT_DISCORD_TOKEN").expect("Expected a token in the environment");

    let application_id: u64 = std::env::var("CRABGPT_APPLICATION_ID")
        .expect("Expected an application ID in the environment")
        .parse()
        .expect("Application ID is not valid");

    let mut client = Client::builder(token)
        .application_id(application_id)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

