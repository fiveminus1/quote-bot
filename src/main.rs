use serenity::{all::EventHandler, async_trait};
use serenity::model::gateway::Ready;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn ready(&self, _: Context, ready: Ready){
    println!("{} is connected. Hello, world!", ready.user.name);
  }

  async fn message(&self, ctx: Context, msg: Message){
    if msg.content == "!ping" {
      if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
        println!("Error sending message: {:?}", why);
      }
    }
  }
}


#[tokio::main]
async fn main() {
  dotenvy::dotenv().ok();
  let token = env::var("DISCORD_TOKEN").expect("Error: expected Discord Token in .env");

  let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
  let mut client = Client::builder(&token, intents)
    .event_handler(Handler)
    .await
    .expect("Error creating client");

  if let Err(why) = client.start().await {
    println!("Client error: {:?}", why);
  }
}
