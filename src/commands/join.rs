use crate::*;
use helper::*;

///Join test
#[poise::command(slash_command)]
pub async fn join(
   ctx: Context<'_>
) -> StdResult<()> {
   if !has_perm(&ctx).await {
      return Ok(());
   }

   if let Err(e) = commands::join_channel(ctx).await {
      panic!("Failed to join with '/join': {:?}", e);
   }

   check_result(ctx.say("SUISEI WAAAAA").await);

   Ok(())
}