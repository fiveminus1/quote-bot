use poise::serenity_prelude as serenity;
use chrono::{Local};
use crate::types::{Context, Error, Quote, LeaderboardType};
use crate::db::{insert_quote, get_most_quoted, get_most_quotes};
use interim::{parse_date_string, Dialect};
use crate::helpers::{format_leaderboard_page, create_nav_buttons};

#[poise::command(slash_command)]
pub async fn quote(
  ctx: Context<'_>,
  #[description = "Who are you quoting?"] user: serenity::User,
  #[description = "What did they say?"] text: String,
  #[description = "When did they say it? (Optional)"] time: Option<String>, //todo: better description on how to describe a date
) -> Result<(), Error> {
  let quote_time = match &time {
    Some(t) => match parse_date_string(&t, Local::now(), Dialect::Us) {
      Ok(parsed) => parsed,
      Err(_) => {
        ctx.send(poise::CreateReply::default()
          .content("Error: invalid date or couldn't parse.")
          .ephemeral(true)
        )
        .await?;
        
        return Ok(());
      }
    },
    None => Local::now(),
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
pub async fn leaderboard(
  ctx: Context<'_>,
  #[description = "Type of leaderboard"] kind: LeaderboardType,
) -> Result<(), Error> {
  let results = match kind {
    LeaderboardType::MostQuoted => get_most_quoted(&ctx.data().db).await?,
    LeaderboardType::MostQuotes => get_most_quotes(&ctx.data().db).await?,
  };

  let page = 0;
  let per_page = 5; // 15 board members, 3 total pages?
  let total_pages = (results.len() + per_page - 1) / per_page;

  let content = format_leaderboard_page(&results, kind, page, total_pages);
  let components = create_nav_buttons(page, total_pages);

  ctx.send(poise::CreateReply::default()
    .content(content)
    .components(components)
    .ephemeral(true)
  )
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