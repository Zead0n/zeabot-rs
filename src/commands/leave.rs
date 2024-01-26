use crate::*;
use helper::*;

#[poise::command(slash_command)]
pub async fn leave(
   ctx: Context<'_>
) -> StdResult<()> {
   if !has_perm(&ctx).await {
      return Ok(());
   }

   let guild_id = ctx.guild_id().expect("Couldn't get guild_id for leave");
   let manager = songbird::get(ctx.serenity_context())
      .await
      .expect("Songbird Voice client placed in at initialisation.")
      .clone();
   let has_handler = manager.get(guild_id).is_some();

   if has_handler {
      check_result(manager.remove(guild_id).await);
      check_result(ctx.say("Left the channel").await);
   } else {
      check_result(ctx.say("Not even there").await);
   }

   Ok(())
}