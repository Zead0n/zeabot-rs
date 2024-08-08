use crate::prelude::*;
use crate::utils::*;

#[poise::command(slash_command)]
pub async fn pause(ctx: Context<'_>) -> StdResult<()> {
    match discord::get_player(&ctx) {
        Some(player_context) => {
            // let player_state = player_context.get_player().await?;
            if let Err(e) = player_context.set_pause(true).await {
                eprintln!("Error unpausing player: {:?}", e);
            }
            discord::send_message(&ctx, "Pausing player").await
        }
        None => discord::send_message(&ctx, "Not in VC").await,
    }

    Ok(())
}
