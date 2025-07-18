use poise::serenity_prelude as serenity;
use crate::types::{LeaderboardType};

pub fn format_leaderboard_page(
  results: &[(String, i64)],
  kind: LeaderboardType,
  page: usize,
  total_pages: usize
) -> String {
  let per_page = 5;
  let start = page * per_page;
  let end = (start + per_page).min(results.len());

  let mut content = format!(
    "**{} Leaderboard \nPage {}/{}\n\n",
    match kind {
      LeaderboardType::MostQuoted => "Most Quoted",
      LeaderboardType::MostQuotes => "Most Quotes",
    },
    page + 1,
    total_pages
  );

  for (i, (user_id, count)) in results[start..end].iter().enumerate() {
    content.push_str(&format!(
      "**{}.** <@{}> - {} quotes\n",
      start + i + 1,
      user_id,
      count
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