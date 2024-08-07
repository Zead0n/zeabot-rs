#![allow(unused_assignments)]

use crate::prelude::*;
use crate::utils::*;

/// Loop current track
#[poise::command(slash_command)]
pub async fn r#loop(ctx: Context<'_>) -> StdResult<()> {
    match discord::get_player(&ctx) {
        Some(player_context) => {
            let mut loop_state = *player_context.data::<PlayerData>()?.looping.lock().await;

            if loop_state {
                loop_state = false;
                discord::send_message(&ctx, "Enabled looping").await;
            } else {
                loop_state = true;
                discord::send_message(&ctx, "Disabled looping").await;
            }
        }
        None => discord::send_message(&ctx, "Not in Voice channel").await,
    }

    Ok(())
}
