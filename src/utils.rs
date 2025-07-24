use poise::serenity_prelude as serenity;
use ::serenity::all::{Colour, CreateEmbedAuthor, CacheHttp};
use crate::types::{LeaderboardType};

pub fn format_leaderboard_page(
  results: &[(String, i64)],
  page: usize,
) -> String {
  let per_page = 5; 
  let start = page * per_page;
  let end = (start + per_page).min(results.len());

  let mut content = String::new();

  for (i, (user_id, count)) in results[start..end].iter().enumerate() {
    content.push_str(&format!(
      "{}. <@{}> - {} {}\n",
      start + i + 1,
      user_id,
      count,
      if *count == 1 { "quote" } else { "quotes" }
    ));
  }

  content
}

pub fn create_nav_buttons(
    page: usize,
    total_pages: usize,
) -> Vec<serenity::CreateActionRow> {
    let prev_btn = serenity::CreateButton::new("prev_page")
        .label("< Prev")
        .style(serenity::ButtonStyle::Primary)
        .disabled(page == 0);

    let next_btn = serenity::CreateButton::new("next_page")
        .label("Next >")
        .style(serenity::ButtonStyle::Primary)
        .disabled(page + 1 >= total_pages);

    vec![serenity::CreateActionRow::Buttons(vec![prev_btn, next_btn])]
}

pub fn create_leaderboard_embed(
  ctx: &impl CacheHttp,
  kind: &LeaderboardType,
  description: String,
  page: usize,
  total_pages: usize
) -> serenity::CreateEmbed {
  let title = match kind {
    LeaderboardType::MostQuoted => "Most Quoted 🗣️",
    LeaderboardType::MostQuotes => "Most Quotes 👀",
  };

  serenity::CreateEmbed::default()
    .author(get_embed_author(ctx))
    .title(format!("{}", title))
    .description(format!("{}", description))
    .footer(serenity::CreateEmbedFooter::new(format!("Page {}/{}", page + 1, total_pages)))
    .color(Colour::MAGENTA)
}

pub fn get_embed_author(ctx: &impl CacheHttp) -> CreateEmbedAuthor {
  if let Some(cache) = ctx.cache(){
    let bot_user = cache.current_user();
    let avatar_url = bot_user.avatar_url().unwrap_or_else(|| bot_user.default_avatar_url());
    CreateEmbedAuthor::new(bot_user.name.clone()).icon_url(avatar_url)
  } else {
    CreateEmbedAuthor::new("Quote Bot")
  }
}