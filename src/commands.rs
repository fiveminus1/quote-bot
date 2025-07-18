use poise::serenity_prelude as serenity;
use chrono::{Local};
use crate::types::{Context, Error, Quote, LeaderboardType};
use crate::db::{insert_quote, get_most_quoted, get_most_quotes};
use interim::{parse_date_string, Dialect};

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

  let mut leaderboard = String::new();
  for (i, (user_id, count)) in results.iter().enumerate(){
    leaderboard.push_str(&format!("**{}.** <@{}> - **{}** quotes\n", i+1, user_id, count));
  }

  ctx.send(poise::CreateReply::default()
    .content(format!("**Leaderboard - {:?}:**\n{}", 
    match kind {
      LeaderboardType::MostQuoted => "Most Quoted",
      LeaderboardType::MostQuotes => "Most Quotes",
    }, leaderboard))
    .ephemeral(true))
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