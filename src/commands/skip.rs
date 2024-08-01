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

    Ok(())
}
