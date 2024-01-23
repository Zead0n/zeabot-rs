use crate::{Context, StdResult, commands};

///Join test
#[poise::command(slash_command)]
pub async fn join(
   ctx: Context<'_>
) -> StdResult<()> {
   if let Err(e) = commands::join_channel(ctx).await {
      panic!("Failed to join with '/join': {:?}", e);
   }

   commands::check_message(ctx.say("SUISEI WAAAAA").await);

   Ok(())
}