use sqlx::SqlitePool;
use poise::serenity_prelude::UserId;
use chrono::{DateTime, Local};
use notion_client::endpoints::Client as NotionClient;
use crate::user_map::UserMap;

pub struct Quote {
  pub quoted_by: UserId,
  pub quoted_user: UserId,
  pub quoted_text: String,
  pub quote_time: DateTime<Local>,
}

pub struct Data {
  pub db: SqlitePool,
  pub notion: NotionClient,
  pub notion_db_id: String,
  pub user_map: UserMap,
}

#[derive(poise::ChoiceParameter)]
pub enum LeaderboardType {
  #[name = "Most Quoted"]
  MostQuoted,
  #[name = "Most Quotes"]
  MostQuotes,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
