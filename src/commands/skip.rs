use crate::*;
use helper::*;

/// Skip current track
#[poise::command(slash_command)]
pub async fn skip(
    ctx: Context<'_>
) -> StdResult<()> {
    if !has_perm(&ctx).await {
        return Ok(());
    }

    if let Some(handler) = commands::handler_exist(ctx).await {
        let handler_lock = handler.lock().await;
        let queue = handler_lock.queue();
        if let Err(e) = queue.skip() {
            panic!("Error skipping current track: {:?}", e);
        }

        if let Err(e) = ctx.say("Skipped current track").await {
            panic!("Error sending skipped track: {:?}", e);
        }
    } else {
        if let Err(e) = ctx.say("Not in VC").await {
            panic!("Error sending unavailable in /skip: {:?}", e);
        }
    }

    Ok(())
}