mod commands;
mod types;
mod db;
mod helpers;
mod notion;
mod user_map;

use poise::serenity_prelude as serenity;
use dotenvy::dotenv;
use std::env;
use notion_client::endpoints::Client as NotionClient;

use crate::commands::{quote, leaderboard};
use crate::types::{Data, Error};
use crate::db::setup_db;

#[tokio::main]
async fn main() {
  println!("Loading .env");
  dotenv().ok();
  println!("Loading token");
  let token = env::var("DISCORD_TOKEN").expect("Error: missing Discord Token in .env");
  println!("Loaded Discord token: {}", token);

  let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::GUILD_MESSAGES | serenity::GatewayIntents::MESSAGE_CONTENT;

  let framework = poise::Framework::builder()
    .options(poise::FrameworkOptions::<Data, Error> {
      commands: vec![quote(), leaderboard()],
      ..Default::default()
    })
    .setup(|ctx, _ready, framework| {
      Box::pin(async move {
        let env_guild_id: u64 = env::var("DISCORD_GUILD_ID")?.parse().map_err(|e| format!("Error: invalid Discord Guild ID - {}", e))?;
        let guild_id = serenity::GuildId::new(env_guild_id); // using env guild id because i'm not patient enough to wait for discord to register slashes globally
        poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id).await?; 
        let db = setup_db().await?;
        let notion_api_key = env::var("NOTION_API_KEY");
        let notion_db_id = env::var("NOTION_DB_ID")?;
        let user_map = user_map::UserMap::load_from_file("user_map.json")?; //todo: probably automate this

        let notion = NotionClient::new(notion_api_key?, None)
          .map_err(|e| format!("Error (Notion): Failed to initialize client - {}", e))?;

        println!("{} is connected. Hello, world!", ctx.cache.current_user().name);


        Ok(Data { db, notion, notion_db_id, user_map })
      })
    })
    .build();
  
  let client = serenity::ClientBuilder::new(token, intents)
    .framework(framework)
    .await;
  
  match client {
    Ok(mut c) => {
      if let Err(e) = c.start().await {
        eprintln!("Client failed to start: {:?}", e);
      }
    }
    Err(e) => {
      eprintln!("Failed to build client: {:?}", e);
  }
}
  
  // client.unwrap().start().await.unwrap();
}
