use crate::utils::*;
use crate::*;

/// Skip current track
#[poise::command(slash_command)]
pub async fn skip(ctx: Context<'_>) -> StdResult<()> {
    if !discord::has_perm(&ctx).await? {
        return Ok(());
    }

    if let Some(player_context) = discord::get_player(&ctx) {
        player_context.skip()?;
        discord::send_message(&ctx, "Skipped current track").await;
    } else {
        discord::send_message(&ctx, "Not in a Voice Channel").await;
    }

    // if let Some(handler) = commands::handler_exist(ctx).await {
    //     let handler_lock = handler.lock().await;
    //     let queue = handler_lock.queue();

    //     // check_result(queue.skip());
    //     if let Err(e) = queue.skip() {
    //         panic!("Error skipping track: {:?}", e);
    //     }
    //     // check_result(ctx.say("Skipped current track").await);
    //     if let Err(e) = ctx.say("Skipped current track").await {
    //         panic!("Error sending skip sucess message: {:?}", e);
    //     }
    // } else {
    //     // check_result(ctx.say("Not in VC").await);
    //     if let Err(e) = ctx.say("Not in VC").await {
    //         panic!("Error sending skip failed message: {:?}", e);
    //     }
    // }

    Ok(())
}
