use crate::{Context, StdResult, commands};

/// Skip current track
#[poise::command(slash_command)]
pub async fn skip(
    ctx: Context<'_>
) -> StdResult<()> {
    if let Some(handler) = commands::handler_exist(ctx).await {
        let handler_lock = handler.lock().await;
        let queue = handler_lock.queue();
        let _bruh = queue.skip();

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