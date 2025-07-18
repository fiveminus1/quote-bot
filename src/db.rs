use sqlx::{SqlitePool};
use crate::types::Quote;

pub async fn setup_db() -> Result<SqlitePool, sqlx::Error>{
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
