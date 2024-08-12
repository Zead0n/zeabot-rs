// use poise::serenity_prelude as serenity;

use futures::future;
use futures::StreamExt;

// use crate::{Context, StdResult, commands};
use crate::prelude::*;
use crate::utils::*;

// /// Show queue
#[poise::command(slash_command)]
pub async fn queue(ctx: Context<'_>) -> StdResult<()> {
    let Some(player_context) = discord::get_player(&ctx) else {
        discord::send_message(&ctx, "Not in VC").await;
        return Ok(());
    };

    let queue = player_context.get_queue();
    let player = player_context.get_player().await?;
    let max = queue.get_count().await?.min(9);

    let queue_message = queue
        .enumerate()
        .take_while(|(idx, _)| future::ready(*idx < max))
        .map(|(idx, song)| match &song.track.info.uri {
            Some(uri) => format!(
                "**{}.** [{} - {}](<{}>)",
                idx + 1,
                song.track.info.title,
                song.track.info.author,
                uri
            ),
            None => format!(
                "**{}.** {} - {}",
                idx + 1,
                song.track.info.title,
                song.track.info.author
            ),
        })
        .collect::<Vec<String>>()
        .await
        .join("\n");

    let current_message = match player.track {
        Some(track) => match &track.info.uri {
            Some(uri) => format!(
                "Now playing: [{} - {}](<{}>)",
                track.info.title, track.info.author, uri
            ),
            None => format!("Now playing: {} - {}", track.info.title, track.info.author),
        },
        None => format!("Now playing: Nothing"),
    };

    discord::send_message(&ctx, format!("{}\n\n{}", current_message, queue_message)).await;

    Ok(())
}
