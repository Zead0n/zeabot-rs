use crate::prelude::*;
use crate::utils::*;

/// Loop current track
#[poise::command(slash_command)]
pub async fn r#loop(ctx: Context<'_>) -> StdResult<()> {
    match discord::get_player(&ctx) {
        Some(player_context) => {
            let mut player_data = *player_context.data::<PlayerData>()?;

            if player_data.looping {
                player_data.looping = false;
                discord::send_message(&ctx, "Disabled looping").await;
            } else {
                player_data.looping = true;
                discord::send_message(&ctx, "Enabled looping").await;
            }
        }
        None => discord::send_message(&ctx, "Not in Voice channel").await,
    }

    Ok(())
}
