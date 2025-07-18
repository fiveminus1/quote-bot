use sqlx::SqlitePool;
use poise::serenity_prelude::UserId;

pub struct Quote {
  pub quoted_by: UserId,
  pub quoted_user: UserId,
  pub quoted_text: String,
  pub quote_time: String,
}

pub struct Data {
  pub db: SqlitePool
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
