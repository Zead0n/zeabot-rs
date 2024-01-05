mod help;
mod join;
mod leave;
mod play;
mod queue;
mod skip;

// Miscellaneous/Global functions & structs
use std::sync::Arc;
use serenity::prelude::Mutex;
use serenity::all::GuildId;
use songbird::Call;
use songbird::events::{Event, EventContext, EventHandler as VoiceEventHandler, TrackEvent};
use poise::async_trait;

use crate::bot::Data;
use crate::{StdError, StdResult, Context};

struct TrackErrorNotifier {
   _guild_id: GuildId,
}

impl TrackErrorNotifier {
   fn new(guild_id: GuildId) -> Self {
      TrackErrorNotifier {
         _guild_id: guild_id,
      }
   }
}

#[async_trait]
impl VoiceEventHandler for TrackErrorNotifier {
   async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
      match ctx {
         EventContext::Track(track_list) => {
            for (state, handle) in *track_list {
               println!("Track {:?} encountered an error: {:?}", handle.uuid(), state.playing);
            }
         },
         _ => {},
      }

      None
   }
}

pub fn get_commands() -> Vec<poise::Command<Data, StdError>> {
   vec![
      help::help(),
      join::join(),
      leave::leave(),
      poise::Command {
         subcommands: vec![
            play::url(),
         ],
         subcommand_required: true,
         ..play::play()
      },
      skip::skip(),
      queue::queue(),
   ]
}

pub async fn handler_exist(ctx: Context<'_>) -> Option<Arc<Mutex<Call>>> {
   let guild_id = ctx.guild_id().unwrap();
   let manager = songbird::get(ctx.serenity_context())
      .await
      .expect("Songbird Voice client placed in at initialisation.")
      .clone();
   let has_handler = manager.get(guild_id);

   has_handler
}

pub async fn join_channel(ctx: Context<'_>) -> StdResult<Arc<Mutex<Call>>> {
   let (guild_id, channel_id) = {
      let guild = ctx.guild().expect("Couldn't get guild for join_channel");
      let channel = guild.voice_states.get(&ctx.author().id).and_then(|voice_state| voice_state.channel_id);
      (guild.id, channel)
   };

   let connect_to = match channel_id {
      Some(channel) => channel,
      None => {
         panic!("Couldn't get channel id");
      },
   };

   let manager = songbird::get(ctx.serenity_context())
      .await
      .expect("Songbird Voice client placed in at initialisation.")
      .clone();

   if let Ok(handler_lock) = manager.join(guild_id, connect_to).await {
      // Attach an event handler to see notifications of all track errors.
      let mut handler = handler_lock.lock().await;
      handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier::new(guild_id));
   }

   let handler = match manager.join(guild_id, connect_to).await {
      Ok(handler) => handler,
      Err(e) => panic!("Bruh: {:?}", e),
   };

   handler.lock().await.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier::new(guild_id));

   Ok(handler)
}
