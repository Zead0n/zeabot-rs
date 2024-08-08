use crate::prelude::*;
use crate::utils::*;

#[poise::command(slash_command)]
pub async fn resume(ctx: Context<'_>) -> StdResult<()> {
    match discord::get_player(&ctx) {
        Some(player_context) => {
            if let Err(e) = player_context.set_pause(false).await {
                eprintln!("Error unpausing player: {:?}", e);
            }
            discord::send_message(&ctx, "Unpause player").await;
        }
        None => discord::send_message(&ctx, "Not in VC").await,
    }

    Ok(())
}
