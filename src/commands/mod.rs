pub mod join;
pub mod leave;
pub mod play;
pub mod queue;
pub mod skip;
pub mod help;

// Miscellaneous/Global functions & structs
use std::sync::Arc;
use serenity::prelude::Mutex;
use songbird::Call;
use songbird::events::{*, EventHandler as VoiceEventHandler};
use poise::serenity_prelude as serenity;
use poise::async_trait;

use crate::*;
use helper::*;
use bot::Data;

struct VoiceCallEvent {
   context: serenity::Context,
   handler: Arc<Mutex<Call>>,
}

impl VoiceCallEvent {
   fn new(context: serenity::Context, handler: Arc<Mutex<Call>>) -> Self {
      Self {
         context,
         handler
      }
   }
}

#[async_trait]
impl VoiceEventHandler for VoiceCallEvent {
   async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
      match ctx {
         EventContext::ClientDisconnect(_) => {
            let manager = songbird::get(&self.context)
               .await
               .expect("Songbird Voice client placed in at initialisation.")
               .clone();
            let songbird_call = self.handler.lock().await;
            let songbird_channel_id = songbird_call.current_channel().expect("No channel id found").0;
            let guild_channel = check_result(serenity::ChannelId::from(songbird_channel_id).to_channel(&self.context).await).guild().expect("No Guild found");
            let check_empty = check_result(guild_channel.members(&self.context)).len() <= 1;
            let guild_id = guild_channel.guild_id;

            if check_empty {
               check_result(manager.remove(guild_id).await);
            }
         }
         _ => {},
      }

      None
   }
}

// Return the list of commands to be registered
pub fn get_commands() -> Vec<poise::Command<Data, StdError>> {
   vec![
      join::join(),
      leave::leave(),
      poise::Command {
         subcommands: vec![
            play::url(),
            play::search(),
         ],
         subcommand_required: true,
         ..play::play()
      },
      skip::skip(),
      // queue::queue(),
   ]
}

// Check if handler exists and return it if it does
pub async fn handler_exist(ctx: Context<'_>) -> Option<Arc<Mutex<Call>>> {
   let guild_id = ctx.guild_id().unwrap();
   let manager = songbird::get(ctx.serenity_context())
      .await
      .expect("Songbird Voice client placed in at initialisation.")
      .clone();
   let has_handler = manager.get(guild_id);

   has_handler
}

// Join a channel and return the handler
pub async fn join_channel(ctx: Context<'_>) -> StdResult<Arc<Mutex<Call>>> {
   let (guild_id, channel_id) = {
      let guild = ctx.guild().expect("Couldn't get guild for join_channel");
      let channel = guild.voice_states.get(&ctx.author().id).and_then(|voice_state| voice_state.channel_id);
      (guild.id, channel)
   };

   let connect_to = match channel_id {
      Some(channel) => channel,
      None => {
         check_message(ctx.say("Where you at?").await);

         panic!("Couldn't get channel id");
      },
   };

   let serenity_context = ctx.serenity_context();
   let manager = songbird::get(serenity_context)
      .await
      .expect("Songbird Voice client placed in at initialisation.")
      .clone();

   // if let Ok(handler_lock) = manager.join(guild_id, connect_to).await {
   //    // Attach an event handler to see notifications of all track errors.
   //    let mut handler = handler_lock.lock().await;
   //    handler.add_global_event(TrackEvent::Error.into(), VoiceCallEvent::new(guild_id));
   // }

   let handler = match manager.join(guild_id, connect_to).await {
      Ok(handler) => handler,
      Err(e) => panic!("Bruh: {:?}", e),
   };

   handler.lock().await.add_global_event(CoreEvent::ClientDisconnect.into(), VoiceCallEvent::new(serenity_context.clone(), handler.clone()));

   Ok(handler)
}

// Discord check message 
pub fn check_message(result: Result<poise::reply::ReplyHandle, poise::serenity_prelude::Error>) {
   if let Err(e) = result {
      panic!("Error sending check message: {:?}", e)
   }
}
