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

use crate::commands::{quote, ping, leaderboard};
use crate::types::{Data, Error};
use crate::db::setup_db;

#[tokio::main]
async fn main() {
  dotenv().ok();
  let token = env::var("DISCORD_TOKEN").expect("Error: missing Discord Token in .env");

  let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::GUILD_MESSAGES | serenity::GatewayIntents::MESSAGE_CONTENT;

  let framework = poise::Framework::builder()
    .options(poise::FrameworkOptions::<Data, Error> {
      commands: vec![ping(), quote(), leaderboard()],
      ..Default::default()
    })
    .setup(|ctx, _ready, framework| {
      Box::pin(async move {
        let guild_id = serenity::GuildId::new(1073078539051614259);
        poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id).await?; // using guild id temp for dev
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
  client.unwrap().start().await.unwrap();
}
