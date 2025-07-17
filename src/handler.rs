use serenity::all::*;
use serenity::async_trait;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn ready(&self, _: Context, ready: Ready){
    println!("{} is connected. Hello, world!", ready.user.name);
  }

  async fn message(&self, ctx: Context, msg: Message){
    if msg.content == "!ping" {
      if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
        println!("Error sending message: {:?}", why);
      }
    }
  }
}