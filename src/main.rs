use poise::serenity_prelude as serenity;
use dotenvy::dotenv;
use std::env;
use sqlx::SqlitePool;

mod commands;
mod types;
use crate::commands::{quote, ping};
use crate::types::{Data, Error};

#[tokio::main]
async fn main() {
  dotenv().ok();
  let token = env::var("DISCORD_TOKEN").expect("Error: missing Discord Token in .env");

  let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::GUILD_MESSAGES | serenity::GatewayIntents::MESSAGE_CONTENT;

  let framework = poise::Framework::builder()
    .options(poise::FrameworkOptions::<Data, Error> {
      commands: vec![ping(), quote()],
      ..Default::default()
    })
    .setup(|ctx, _ready, framework| {
      Box::pin(async move {
        let guild_id = serenity::GuildId::new(1073078539051614259);
        poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id).await?; // using guild id temp for dev
        println!("{} is connected. Hello, world!", ctx.cache.current_user().name);

        let db = SqlitePool::connect("sqlite:quotes.db").await?;
        sqlx::query(
          "CREATE TABLE IF NOT EXISTS quotes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            quoted_by TEXT NOT NULL,
            quoted_user TEXT NOT NULL,
            quoted_text TEXT NOT NULL,
            quote_time TEXT NOT NULL
          );"
        )
        .execute(&db)
        .await?;

        Ok(Data { db })
      })
    })
    .build();
  
  let client = serenity::ClientBuilder::new(token, intents)
    .framework(framework)
    .await;
  client.unwrap().start().await.unwrap();
}
