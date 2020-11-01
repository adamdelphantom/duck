use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

const QUACK_MESSAGE: &str = "🦆 quack!";
const QUACK_COMMAND: &str = "quack";

const FLY_MESSAGE: &str = "🦆 Can you fly, Bobby?";
const FLY_COMMAND: &str = "fly";

const LAY_MESSAGE: &str = "🥚";
const LAY_COMMAND: &str = "lay";

const SLEEP_MESSAGE: &str = "😴";
const SLEEP_COMMAND: &str = "sleep";


struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == QUACK_COMMAND {
            if let Err(e) = msg.channel_id.say(&ctx.http, QUACK_MESSAGE).await {
                println!("Error sending message: {:?}", e);
            }
        } else if msg.content == FLY_COMMAND {
            if let Err(e) = msg.channel_id.say(&ctx.http, FLY_MESSAGE).await {
                println!("Error sending message: {:?}", e);
            }
        } else if msg.content == LAY_COMMAND {
            if let Err(e) = msg.channel_id.say(&ctx.http, LAY_MESSAGE).await {
                println!("Error sending message: {:?}", e);
            }
        } else if msg.content == SLEEP_COMMAND {
            if let Err(e) = msg.channel_id.say(&ctx.http, SLEEP_MESSAGE).await {
                println!("Error sending message: {:?}", e);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::new(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(e) = client.start().await {
        println!("Client error: {:?}", e);
    }
}
 