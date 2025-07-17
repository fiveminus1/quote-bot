mod handler;

use dotenvy::dotenv;
use serenity::prelude::*;
use std::env;
use handler::Handler;


#[tokio::main]
async fn main() {
  dotenv().ok();
  let token = env::var("DISCORD_TOKEN").expect("Error: expected Discord Token in .env");

  let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

  let mut client = Client::builder(&token, intents)
    .event_handler(Handler)
    .await
    .expect("Error creating client");

  if let Err(why) = client.start().await {
    println!("Error (client): {:?}", why);
  }
}
