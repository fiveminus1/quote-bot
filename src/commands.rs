use poise::serenity_prelude as serenity;
use chrono::{Local};
use crate::notion::add_quote_to_notion;
use crate::types::{Context, Error, Quote, LeaderboardType};
use crate::db::{insert_quote, get_most_quoted, get_most_quotes};
use interim::{parse_date_string, Dialect};
use crate::helpers::{create_leaderboard_embed, create_nav_buttons, format_leaderboard_page};

/// Log a quote from someone! 
#[poise::command(slash_command)]
pub async fn quote(
  ctx: Context<'_>,
  #[description = "Who said it?"] who: serenity::User,
  #[description = "What did they say?"] quote: String,
  #[description = "When did they say it? (optional, defaults to now if not specified)"] when: Option<String>,
) -> Result<(), Error> {
  let quote_time = match &when {
    Some(t) => match parse_date_string(&t, Local::now(), Dialect::Us) {
      Ok(parsed) => parsed,
      Err(_) => {
        ctx.send(poise::CreateReply::default()
          .content("Error: invalid date or couldn't parse. Try '5pm,' 'yesterday 2:30pm,' '6/19 4pm'.")
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
    quoted_user: who.id,
    quoted_text: quote.clone(),
    quote_time,
  };

  insert_quote(&ctx.data().db, &quote).await?;
  if let Err(e) = add_quote_to_notion(&ctx.data().notion, &ctx.data().notion_db_id, &quote, &ctx.data().user_map).await {
    eprintln!("Error (Notion): failed to add quote to Notion - {}", e);
  }

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

/// Check who's been quoted the most, or who has the most quotes!
#[poise::command(slash_command)]
pub async fn leaderboard(
  ctx: Context<'_>,
  #[description = "Type of leaderboard"] sort_by: LeaderboardType,
) -> Result<(), Error> {
  let results = match sort_by {
    LeaderboardType::MostQuoted => get_most_quoted(&ctx.data().db).await?,
    LeaderboardType::MostQuotes => get_most_quotes(&ctx.data().db).await?,
  };

  let page = 0;
  let per_page = 5; // 15 board members, 3 total pages?
  let total_pages = (results.len() + per_page - 1) / per_page;

  let description = format_leaderboard_page(&results, page);
  let embed = create_leaderboard_embed(&sort_by, description, page, total_pages);
  let components = create_nav_buttons(page, total_pages);

  ctx.send(poise::CreateReply::default()
    .embed(embed)
    .components(components)
  )
  .await?;

  Ok(())
}