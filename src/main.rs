use std::sync::Arc;
use tokio::sync::Mutex;

use poise::serenity_prelude as serenity;
use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone)]
struct Quote {
  quoted_by: serenity::UserId,
  quoted_user: serenity::UserId,
  quoted_text: String,
}

#[derive(Default)]
struct Data{
  quotes: Arc<Mutex<Vec<Quote>>>,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command)]
async fn quote(
  ctx: Context<'_>,
  #[description = "Who are you quoting?"] user: serenity::User,
  #[description = "What did they say?"] text: String,
) -> Result<(), Error> {
  let quote = Quote{
    quoted_by: ctx.author().id,
    quoted_user: user.id,
    quoted_text: text.clone(),
  };

  {
    let mut quotes = ctx.data().quotes.lock().await;
    quotes.push(quote.clone());
  }

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
async fn ping(
  ctx: Context<'_>,
) -> Result<(), Error> {
  ctx.say("Pong!").await?;
  Ok(())
}

#[tokio::main]
async fn main() {
  dotenv().ok();
  let token = env::var("DISCORD_TOKEN").expect("Error: missing Discord Token in .env");

  let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::GUILD_MESSAGES | serenity::GatewayIntents::MESSAGE_CONTENT;

  let framework = poise::Framework::builder()
    .options(poise::FrameworkOptions::<Data, Error> {
      commands: vec![ping(), quote()],
      ..Default::default()
    })
    .setup(|ctx, _ready, framework| {
      Box::pin(async move {
        let guild_id = serenity::GuildId::new(1073078539051614259);
        poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id).await?; // using guild id temp for dev
        println!("{} is connected. Hello, world!", ctx.cache.current_user().name);
        Ok(Data {
          quotes: Arc::new(Mutex::new(Vec::new())),
        })
      })
    })
    .build();
  
  let client = serenity::ClientBuilder::new(token, intents)
    .framework(framework)
    .await;
  client.unwrap().start().await.unwrap();
}
