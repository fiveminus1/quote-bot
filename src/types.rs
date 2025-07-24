use sqlx::SqlitePool;
use poise::serenity_prelude::UserId;
use chrono::{DateTime, Local};
use notion_client::endpoints::Client as NotionClient;
use std::{collections::HashMap, fs};
use serde::Deserialize;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

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

#[derive(Debug, Deserialize)]
pub struct UserMap(pub HashMap<String, String>); //todo: move to types if no further logic for this file

impl UserMap {
    pub fn load_from_file(path: &str) -> Result<Self, Error> {
        let file_content = fs::read_to_string(path)?;
        let map: HashMap<String, String> = serde_json::from_str(&file_content)?;
        Ok(UserMap(map))
    }

    pub fn resolve(&self, id: &str) -> String {
        self.0.get(id).cloned().unwrap_or_else(|| id.to_string())
    }
}
