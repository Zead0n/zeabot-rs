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

        check_result(queue.skip());
        check_result(ctx.say("Skipped current track").await);
    } else {
        check_result(ctx.say("Not in VC").await);
    }

    Ok(())
}