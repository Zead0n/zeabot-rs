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

   check_result(commands::join_channel(ctx).await);

   check_result(ctx.say("Joined Channel").await);

   Ok(())
}