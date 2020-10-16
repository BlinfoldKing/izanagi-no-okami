use dotenv::dotenv;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "\\ping" {
            if let Err(err) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", err)
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected token in environment");

    let mut client = Client::new(token).event_handler(Handler).await.unwrap();
    println!("hello");

    if let Err(err) = client.start().await {
        println!("{:?}", err)
    }
}