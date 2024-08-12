#![allow(unused_assignments)]

use crate::prelude::*;
use crate::utils::*;

/// Loop current track
#[poise::command(slash_command, subcommands("song", "queue", "off"))]
pub async fn r#loop(_ctx: Context<'_>) -> StdResult<()> {
    Ok(())
}

#[poise::command(slash_command)]
pub async fn song(ctx: Context<'_>) -> StdResult<()> {
    match discord::get_player(&ctx) {
        Some(player_context) => {
            let player_data = &player_context.data::<PlayerData>()?;
            *player_data.loop_state.lock().await = LoopState::Song;
            discord::send_message(&ctx, "Looping song").await
        }
        None => discord::send_message(&ctx, "Not in Voice channel").await,
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn queue(ctx: Context<'_>) -> StdResult<()> {
    match discord::get_player(&ctx) {
        Some(player_context) => {
            let player_data = &player_context.data::<PlayerData>()?;
            *player_data.loop_state.lock().await = LoopState::Queue;
            discord::send_message(&ctx, "Looping queue").await
        }
        None => discord::send_message(&ctx, "Not in Voice channel").await,
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn off(ctx: Context<'_>) -> StdResult<()> {
    match discord::get_player(&ctx) {
        Some(player_context) => {
            let player_data = &player_context.data::<PlayerData>()?;
            *player_data.loop_state.lock().await = LoopState::Cancel;
            discord::send_message(&ctx, "Disabled looping").await
        }
        None => discord::send_message(&ctx, "Not in Voice channel").await,
    }

    Ok(())
}
