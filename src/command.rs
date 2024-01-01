use songbird::events::{Event, EventContext, EventHandler as VoiceEventHandler, TrackEvent};
use poise::async_trait;
use songbird::input::YoutubeDl;
//use songbird::tracks::{ControlError, PlayError};
use crate::{Context, StdResult};

struct TrackErrorNotifier;

#[async_trait]
impl VoiceEventHandler for TrackErrorNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            for (state, handle) in *track_list {
                println!(
                    "Track {:?} encountered an error: {:?}",
                    handle.uuid(),
                    state.playing
                );
            }
        }

        None
    }
}

/// Show this help menu
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> StdResult<()> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "Avaliable commands. (Note: craft command is only avaible in #forge)",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

///Join test
#[poise::command(slash_command)]
pub async fn join(
    ctx: Context<'_>
) -> StdResult<()> {
    let (guild_id, channel_id) = {
        let guild = ctx.guild().expect("Couldn't get guild for join");
        let channel = guild.voice_states.get(&ctx.author().id).and_then(|voice_state| voice_state.channel_id);
        (guild.id, channel)
    };

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            if let Err(e) = ctx.say("Pull up in VC").await {
                panic!("Failed to tell em: {:?}", e);
            }

            return Ok(());
        },
    };

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Ok(handler_lock) = manager.join(guild_id, connect_to).await {
        // Attach an event handler to see notifications of all track errors.
        let mut handler = handler_lock.lock().await;
        handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier);
    }

    Ok(())
}

///Leave test
#[poise::command(slash_command)]
pub async fn leave(
    ctx: Context<'_>
) -> StdResult<()> {
    let guild_id = ctx.guild_id().expect("Couldn't get guild_id for leave");
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            if let Err(e) = ctx.say(format!("Oops something went wrong: {:?}", e)).await {
                panic!("Failed send error for leave: {:?}", e);
            }
        }

        if let Err(e) = ctx.say("Snooze time?").await {
            panic!("Failed to kiss goodnight: {:?}",e);
        }
    } else {
        if let Err(e) = ctx.say("Really goonna me hanging huh? (need to be in VC)").await {
            panic!("Failed to send failed leave call: {:?}", e);
        }
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Enter a URL"] url: String
) -> StdResult<()> {
    let _do_search = !url.starts_with("http");
    let guild_id = ctx.guild_id().unwrap();
    let http_client = ctx.data().http_key.clone();
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        let src = YoutubeDl::new(http_client, url);

        let _ = handler.play_input(src.clone().into());
        if let Err(e) = ctx.say("Successfully found a track").await {
            panic!("Failed to send success play notice: {:?}", e);
        }
    } else {
        if let Err(e) = ctx.say("I ain't here (need to be in VC)").await {
            panic!("Failed to send error play notice: {:?}", e);
        }
    }

    Ok(())
}