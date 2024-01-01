use crate::{Context, StdResult};

#[poise::command(slash_command)]
pub async fn leave(
   ctx: Context<'_>
) -> StdResult<()> {
   let guild_id = ctx.guild_id().expect("Couldn't get guild_id for leave");
   let manager = songbird::get(ctx.serenity_context())
      .await
      .expect("Songbird Voice client placed in at initialisation.")
      .clone();
   let has_handler = manager.get(guild_id).is_some();

   if has_handler {
      if let Err(e) = manager.remove(guild_id).await {
         if let Err(e) = ctx.say(format!("Oops something went wrong: {:?}", e)).await {
               panic!("Failed send error for leave: {:?}", e);
         }
      }

      if let Err(e) = ctx.say("Snooze time?").await {
         panic!("Failed to kiss goodnight: {:?}",e);
      }
   } else {
      if let Err(e) = ctx.say("Really goonna me hanging huh? (need to be in VC)").await {
         panic!("Failed to send failed leave call: {:?}", e);
      }
   }

   Ok(())
}