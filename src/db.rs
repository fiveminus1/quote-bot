use chrono::Utc;
use sqlx::{PgPool, Row};
use crate::types::Quote;
use std::env;
use log::{info, error};

pub async fn setup_db() -> Result<PgPool, sqlx::Error>{
  let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "postgresql://postgres:6346@localhost:5432/quotebookdb".to_string());

  let db = PgPool::connect(&db_url)
    .await
    .map_err(|e| {
      error!("Error (postgres): failed to connect to database - {}", e);
      e
    })?;
  
  sqlx::migrate!("./migrations")
    .run(&db)
    .await
    .map_err(|e| {
      error!("Error (postgres): failed to run migrations - {}", e);
      e
    })?;
  info!("Successfully set up postgres db");
  
  Ok(db)
}

pub async fn insert_quote(db: &PgPool, quote: &Quote) -> Result<(), sqlx::Error> {
  sqlx::query(
    "INSERT INTO quotes (quoted_by, quoted_user, quoted_text, quote_time)
    VALUES ($1, $2, $3, $4)"
  )
  .bind(quote.quoted_by.to_string())
  .bind(quote.quoted_user.to_string())
  .bind(&quote.quoted_text)
  .bind(quote.quote_time.with_timezone(&Utc))
  .execute(db)
  .await?;

  Ok(())
}

pub async fn get_most_quoted(db: &PgPool) -> Result<Vec<(String, i64)>, sqlx::Error> {
  let rows = sqlx::query(
    "SELECT quoted_user, COUNT(*) as count FROM quotes GROUP BY quoted_user ORDER BY count DESC LIMIT 5"
  )
      .fetch_all(db)
      .await?;

  Ok(rows
    .into_iter()
    .map(|row| {
      let user_id: String = row.get("quoted_user");
      let count: i64 = row.get("count");
      (user_id, count)
    })
    .collect())
}

pub async fn get_most_quotes(db: &PgPool) -> Result<Vec<(String, i64)>, sqlx::Error> {
  let rows = sqlx::query(
    "SELECT quoted_by, COUNT(*) as count FROM quotes GROUP BY quoted_by ORDER BY count DESC LIMIT 5"
  )
      .fetch_all(db)
      .await?;

  Ok(rows
    .into_iter()
    .map(|row| {
      let user_id: String = row.get("quoted_by");
      let count: i64 = row.get("count");
      (user_id, count)
    })
    .collect())
}
