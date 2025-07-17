use poise::serenity_prelude as serenity;
use dotenvy::dotenv;
use std::env;

struct Data{}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

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
      commands: vec![ping()],
      ..Default::default()
    })
    .setup(|ctx, _ready, framework| {
      Box::pin(async move {
        let guild_id = serenity::GuildId::new(1073078539051614259);
        poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id).await?; //temp for guild
        println!("{} is connected. Hello, world!", ctx.cache.current_user().name);
        Ok(Data {})
      })
    })
    .build();
  
  let client = serenity::ClientBuilder::new(token, intents)
    .framework(framework)
    .await;
  client.unwrap().start().await.unwrap();
}
