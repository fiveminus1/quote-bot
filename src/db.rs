use sqlx::{SqlitePool, Row};
use crate::types::Quote;
use std::env;
use log::{info, error};

pub async fn setup_db() -> Result<SqlitePool, sqlx::Error>{
  let db_path = env::var("DATABASE_PATH").unwrap_or_else(|_| "/app/quotes.db".to_string());

  let db = sqlx::sqlite::SqlitePoolOptions::new()
    .max_connections(5)
    .connect_with(
      sqlx::sqlite::SqliteConnectOptions::new()
        .filename(&db_path)
        .create_if_missing(true),
    )
    .await
    .map_err(|e| {
      error!("Error (sqlite): Failed to connect to database: {}", e);
      e
    })?;
  
  sqlx::migrate!("./migrations")
    .run(&db)
    .await
    .map_err(|e| {
      error!("Error (sqlite): failed to run migrations - {}", e);
      e
    })?;
  info!("Successfully set up sqlite db");
  
  Ok(db)
}

pub async fn insert_quote(db: &SqlitePool, quote: &Quote) -> Result<(), sqlx::Error> {
  sqlx::query(
    "INSERT INTO quotes (quoted_by, quoted_user, quoted_text, quote_time)
    VALUES (?, ?, ?, ?)"
  )
  .bind(quote.quoted_by.to_string())
  .bind(quote.quoted_user.to_string())
  .bind(&quote.quoted_text)
  .bind(&quote.quote_time.to_rfc3339())
  .execute(db)
  .await?;

  Ok(())
}

pub async fn get_most_quoted(db: &SqlitePool) -> Result<Vec<(String, i64)>, sqlx::Error> {
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

pub async fn get_most_quotes(db: &SqlitePool) -> Result<Vec<(String, i64)>, sqlx::Error> {
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
