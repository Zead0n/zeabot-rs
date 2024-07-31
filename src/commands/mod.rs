pub mod help;
pub mod join;
pub mod leave;
pub mod r#loop;
pub mod play;
pub mod queue;
pub mod skip;

// Miscellaneous/Global functions & structs
// use poise::async_trait;
// use serenity::cache::Cache;
// use serenity::model::channel::GuildChannel;
// use songbird::events::EventHandler as VoiceEventHandler;
// use poise::serenity_prelude as serenity;
// use serenity::prelude::Mutex;
// use songbird::*;
// use std::sync::Arc;

// use crate::utils::*;
// use bot::Data;
use crate::error::StandardError;
use crate::prelude::Data;

pub fn get_commands() -> Vec<poise::Command<Data, StandardError>> {
    vec![
        join::join(),
        leave::leave(),
        // poise::Command {
        //     subcommands: vec![play::url(), play::search()],
        //     subcommand_required: true,
        //     ..play::play()
        // },
        play::play(),
        skip::skip(),
        poise::Command {
            name: String::from("loop"),
            ..r#loop::r#loop()
        },
        // queue::queue(),
    ]
}

// struct VoiceCallEvent {
//     manager: Arc<Songbird>,
//     channel: GuildChannel,
//     cache: Arc<Cache>,
// }
//
// impl VoiceCallEvent {
//     fn new(manager: Arc<Songbird>, channel: GuildChannel, cache: Arc<Cache>) -> Self {
//         Self {
//             manager,
//             channel,
//             cache,
//         }
//     }
// }
//
// #[async_trait]
// impl VoiceEventHandler for VoiceCallEvent {
//     async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
//         match ctx {
//             EventContext::ClientDisconnect(_) => {
//                 let check_empty = check_result(self.channel.members(&self.cache)).len() <= 1;
//                 let guild_id = self.channel.guild_id;
//
//                 if check_empty {
//                     check_result(self.manager.remove(guild_id).await);
//                 }
//             }
//             _ => {}
//         }
//
//         None
//     }
// }

// pub async fn handler_exist(ctx: Context<'_>) -> Option<Arc<Mutex<Call>>> {
//     let guild_id = ctx.guild_id().unwrap();
//     let manager = songbird::get(ctx.serenity_context())
//         .await
//         .expect("Songbird Voice client placed in at initialisation.")
//         .clone();
//
//     manager.get(guild_id)
// }

// pub async fn join_channel(ctx: Context<'_>) -> StdResult<Arc<Mutex<Call>>> {
//     let (guild_id, channel_id) = {
//         let guild = ctx.guild().expect("Couldn't get guild for join_channel");
//         let channel = guild
//             .voice_states
//             .get(&ctx.author().id)
//             .and_then(|voice_state| voice_state.channel_id);
//         (guild.id, channel)
//     };
//
//     let connect_to = match channel_id {
//         Some(channel) => channel,
//         None => {
//             check_result(ctx.say("Where you at?").await);
//
//             panic!("Couldn't get channel id");
//         }
//     };
//
//     let serenity_context = ctx.serenity_context();
//     let manager = songbird::get(serenity_context)
//         .await
//         .expect("Songbird Voice client placed in at initialisation.")
//         .clone();
//     let handler = match manager.join(guild_id, connect_to).await {
//         Ok(handler) => handler,
//         Err(e) => panic!("Bruh: {:?}", e),
//     };
//
//     let voice_channel = check_result(serenity::ChannelId::from(connect_to).to_channel(&ctx).await)
//         .guild()
//         .expect("No Guild found");
//
//     handler.lock().await.add_global_event(
//         CoreEvent::ClientDisconnect.into(),
//         VoiceCallEvent::new(manager, voice_channel, ctx.serenity_context().cache.clone()),
//     );
//
//     Ok(handler)
// }
