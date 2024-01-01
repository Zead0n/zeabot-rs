use songbird::events::{Event, EventContext, EventHandler as VoiceEventHandler, TrackEvent};
use poise::async_trait;

use crate::{Context, StdResult};

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