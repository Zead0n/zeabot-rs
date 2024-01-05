use std::sync::Arc;
use serenity::prelude::Mutex;
use songbird::input::YoutubeDl;
use songbird::Call;

use crate::{Context, StdResult, commands};

#[poise::command(slash_command)]
pub async fn play(
   ctx: Context<'_>,
) -> StdResult<()> {
   if let Err(e) = ctx.say("Should require subsommand").await {
      panic!("Failed send play is subcommand nitification: {:?}", e);
   }

   Ok(())
}

///Play via URL
#[poise::command(slash_command)]
pub async fn url(
   ctx: Context<'_>,
   #[description = "Enter a URL"] url: String
) -> StdResult<()> {
   if let Some(handler) = commands::handler_exist(ctx).await {
      if let Err(e) = queue_up(ctx, url, handler).await {
         panic!("Error queuing music: {:?}", e);
      }
   } else {
      let new_handler = commands::join_channel(ctx).await?;
      if let Err(e) = queue_up(ctx, url, new_handler).await {
         panic!("Error queuing music from joining: {:?}", e);
      }
   }

   Ok(())
}

async fn queue_up(ctx: Context<'_>, url: String, handler: Arc<Mutex<Call>>) -> StdResult<()> {
   let http_client = ctx.data().http_key.clone();
   let mut handler_lock = handler.lock().await;
   let src = YoutubeDl::new(http_client, url);

   handler_lock.enqueue_input(src.into()).await;

   if let Err(e) = ctx.say("Successfully found a track").await {
      panic!("Failed to send success play notice: {:?}", e);
   }

   Ok(())
}