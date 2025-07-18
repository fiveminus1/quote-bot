use poise::serenity_prelude as serenity;
use chrono::Utc;
use crate::types::{Context, Error, Quote};
use crate::db::insert_quote;

#[poise::command(slash_command)]
pub async fn quote(
  ctx: Context<'_>,
  #[description = "Who are you quoting?"] user: serenity::User,
  #[description = "What did they say?"] text: String,
  #[description = "When did they say it? (Optional)"] time: Option<String>,
) -> Result<(), Error> {
  let quote_time = match time {
    Some(ref t) => {
      match chrono::DateTime::parse_from_rfc3339(t) {
        Ok(parsed) => parsed.with_timezone(&Utc),
        Err(_) => {
          ctx.send(poise::CreateReply::default()
            .content("Error: invalid date format")
            .ephemeral(true))
            .await ?;
          return Ok(());
        }
      }
    }
    None => Utc::now(),
  };

  let quote = Quote{
    quoted_by: ctx.author().id,
    quoted_user: user.id,
    quoted_text: text.clone(),
    quote_time,
  };

  insert_quote(&ctx.data().db, &quote).await?;

  ctx.send(poise::CreateReply::default()
    .content(format!(
      "Logged quote by <@{}>:\n> {} \nat {}",
      quote.quoted_user,
      quote.quoted_text,
      quote.quote_time
    ))
    .ephemeral(true)
  ).await?;
  
  Ok(())
}

#[poise::command(slash_command)]
pub async fn ping(
  ctx: Context<'_>,
) -> Result<(), Error> {
  ctx.say("Pong!").await?;
  Ok(())
}