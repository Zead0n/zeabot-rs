use crate::prelude::*;
use crate::utils::*;

///Join test
#[poise::command(slash_command)]
pub async fn join(ctx: Context<'_>) -> StdResult<()> {
    if !discord::has_perm(&ctx).await? {
        return Ok(());
    }

    discord::join(&ctx).await?;
    let _ = &ctx.say("SUICHAN WAAAAAA").await?;

    Ok(())
}
