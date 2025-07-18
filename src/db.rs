use sqlx::{SqlitePool};

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