use crate::prelude::*;
use crate::utils::*;

#[poise::command(slash_command)]
pub async fn leave(ctx: Context<'_>) -> StdResult<()> {
    if !discord::has_perm(&ctx).await? {
        return Ok(());
    }

    discord::leave(&ctx).await?;
    let _ = &ctx.say("https://tenor.com/view/suisei-oshimachi-suisei-the-first-take-peace-out-bye-bye-gif-27497602").await?;

    // let guild_id = ctx.guild_id().expect("Couldn't get guild_id for leave");
    // let manager = songbird::get(ctx.serenity_context())
    //     .await
    //     .expect("Songbird Voice client placed in at initialisation.")
    //     .clone();
    // let has_handler = manager.get(guild_id).is_some();
    //
    // if has_handler {
    //     check_result(manager.remove(guild_id).await);
    //     check_result(ctx.say("Left the channel").await);
    // } else {
    //     check_result(ctx.say("Not even there").await);
    // }

    Ok(())
}
