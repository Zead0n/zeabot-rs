mod help;
mod join;
mod leave;
mod play;

// Miscellaneous/Global functions & structs
use serenity::all::GuildId;
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
      // if let EventContext::Track(track_list) = ctx {
      //    for (state, handle) in *track_list {
      //       println!("Track {:?} encountered an error: {:?}", handle.uuid(), state.playing);
      //    }
      // }

      match ctx {
         EventContext::Track(track_list) => {
            for (state, handle) in *track_list {
               println!("Track {:?} encountered an error: {:?}", handle.uuid(), state.playing);
            }
         },
         // EventContext::ClientDisconnect(client) => {

         //    if handler_exist(ctx, guild_id).await.unwrap() {

         //    }
         // },
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
   ]
}

pub async fn handler_exist(ctx: Context<'_>, guild_id: GuildId) -> StdResult<bool> {
   let manager = songbird::get(ctx.serenity_context())
      .await
      .expect("Songbird Voice client placed in at initialisation.")
      .clone();
   let has_handler = manager.get(guild_id).is_some();

   Ok(has_handler)
}

pub async fn join_channel(ctx: Context<'_>) -> StdResult<()> {
   let (guild_id, channel_id) = {
      let guild = ctx.guild().expect("Couldn't get guild for join_channel");
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
      handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier::new(guild_id));
   }

   Ok(())
}