use poise::serenity_prelude as serenity;
use chrono::Utc;
use crate::types::{Context, Error};

#[derive(Debug, Clone)]
struct Quote {
  quoted_by: serenity::UserId,
  quoted_user: serenity::UserId,
  quoted_text: String,
  quote_time: String,
}

#[poise::command(slash_command)]
pub async fn quote(
  ctx: Context<'_>,
  #[description = "Who are you quoting?"] user: serenity::User,
  #[description = "What did they say?"] text: String,
) -> Result<(), Error> {
  let quote = Quote{
    quoted_by: ctx.author().id,
    quoted_user: user.id,
    quoted_text: text.clone(),
    quote_time: Utc::now().to_rfc3339(),
  };

  sqlx::query(
    "INSERT INTO quotes (quoted_by, quoted_user, quoted_text, quote_time)
    VALUES (?, ?, ?, ?)"
  )
  .bind(quote.quoted_by.to_string())
  .bind(quote.quoted_user.to_string())
  .bind(&quote.quoted_text)
  .bind(&quote.quote_time)
  .execute(&ctx.data().db)
  .await?;

  ctx.say(format!(
    "Logged quote. Quoted by <@{}>\nQuoted user<@{}>\n> {}",
    quote.quoted_by,
    quote.quoted_user,
    quote.quoted_text
  ))
  .await?;
  
  Ok(())
}

#[poise::command(slash_command)]
pub async fn ping(
  ctx: Context<'_>,
) -> Result<(), Error> {
  ctx.say("Pong!").await?;
  Ok(())
}