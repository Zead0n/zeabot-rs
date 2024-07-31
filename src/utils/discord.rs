use crate::prelude::*;

use lavalink_rs::client::LavalinkClient;
use lavalink_rs::prelude::PlayerContext;
use poise::async_trait;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::RoleId;
use poise::CreateReply;
use serenity::cache::Cache;
use serenity::model::channel::GuildChannel;
use songbird::EventHandler as SongbirdEventHandler;
use songbird::*;
use std::sync::Arc;

const TEST_SERVER: u64 = 884664077643829248;
const MEME_CORP: u64 = 459781165377650688;
const NIPPON: u64 = 270329415404093440;

pub async fn has_perm(ctx: &Context<'_>) -> Result<bool> {
    let member = ctx.author_member().await.expect("No member found");
    let perm = match member.guild_id.get() {
        NIPPON => {
            const MY_BOI: u64 = 540989126803980289;
            const OKAMI: u64 = 153682548017463296;
            const BOT_CHANNEL: u64 = 360582111398330369;

            let roled = member.roles.contains(&RoleId::new(MY_BOI));
            let okami = member.user.id.get() == OKAMI;
            let bot_channel = ctx.channel_id().get() == BOT_CHANNEL;

            (roled || okami) && bot_channel
        }
        TEST_SERVER => true,
        MEME_CORP => true,
        _ => {
            println!("An unknown server has run a command");
            false
        }
    };

    if !perm {
        if let Err(e) = ctx
            .send(
                CreateReply::default()
                    .content("You don't have the permission to run command")
                    .ephemeral(true),
            )
            .await
        {
            return Err(e.into());
        }
    }

    Ok(perm)
}

struct VoiceCallEvent {
    lavalink: LavalinkClient,
    songbird: Arc<Songbird>,
    guild_channel: GuildChannel,
    cache: Arc<Cache>,
}

impl VoiceCallEvent {
    fn new(
        lavalink: LavalinkClient,
        songbird: Arc<Songbird>,
        guild_channel: GuildChannel,
        cache: Arc<Cache>,
    ) -> Self {
        Self {
            lavalink,
            songbird,
            guild_channel,
            cache,
        }
    }
}

#[async_trait]
impl SongbirdEventHandler for VoiceCallEvent {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::ClientDisconnect(_) = ctx {
            let members = match self.guild_channel.members(&self.cache) {
                Ok(count) => count,
                Err(e) => {
                    eprintln!("Error getting users in VC: {:?}", e);
                    return None;
                }
            };

            let guild_id = self.guild_channel.guild_id;

            if members.len() > 1 {
                return None;
            }

            let player_context = self
                .lavalink
                .get_player_context(guild_id)
                .expect("No PlayerContext found");

            if let Err(e) = player_context.stop_now().await {
                eprintln!("Error stopping player: {:?}", e);
            }

            if let Err(e) = self.lavalink.delete_player(guild_id).await {
                eprintln!("Error deleting lavalink player: {:?}", e)
            }

            if self.songbird.get(guild_id).is_some() {
                if let Err(e) = self.songbird.remove(guild_id).await {
                    eprintln!("Error removeing Sonbird Call: {:?}", e);
                }
            }
        }

        None
    }
}

pub async fn send_message<S: Into<String>>(ctx: &Context<'_>, message: S) -> () {
    if let Err(e) = ctx.send(CreateReply::default().content(message)).await {
        eprintln!("Error sending message: {:?}", e);
    }
}

pub fn get_player(ctx: &Context<'_>) -> Option<PlayerContext> {
    let guild_id = ctx.guild_id().unwrap();
    let lava_client = &ctx.data().lavalink;
    lava_client.get_player_context(guild_id)
}

pub async fn join(ctx: &Context<'_>) -> Result<PlayerContext> {
    let (guild_id, channel_id) = {
        let guild = ctx.guild().expect("Couldn't get guild for join_channel");
        let channel = guild
            .voice_states
            .get(&ctx.author().id)
            .and_then(|voice_state| voice_state.channel_id);
        (guild.id, channel)
    };

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            ctx.say("Where you at?").await?;
            return Err(Error::Generic("User is not in a voice channel".into()));
        }
    };

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let lava_client = &ctx.data().lavalink;

    let handler = manager.join_gateway(guild_id, connect_to).await;
    // let guild_id_raw: u64 = guild_id.into();
    let player_context = match handler {
        Ok((connection_info, call)) => {
            let voice_channel = serenity::ChannelId::from(connect_to)
                .to_channel(&ctx)
                .await?
                .guild()
                .expect("No Guild found");

            call.lock().await.add_global_event(
                CoreEvent::ClientDisconnect.into(),
                VoiceCallEvent::new(
                    lava_client.clone(),
                    manager,
                    voice_channel,
                    ctx.serenity_context().cache.clone(),
                ),
            );

            lava_client
                .create_player_context_with_data(
                    guild_id,
                    connection_info,
                    Arc::new(LavalinkData::default()),
                )
                .await?
        }
        Err(e) => return Err(e.into()),
    };

    Ok(player_context)
}

pub async fn leave(ctx: &Context<'_>) -> Result<()> {
    let guild_id = ctx.guild_id().expect("No guild_id found");

    let lava_client = &ctx.data().lavalink;
    let player_context = lava_client
        .get_player_context(guild_id)
        .expect("No PlayerContext found");

    if let Err(e) = player_context.stop_now().await {
        eprintln!("Error stopping player: {:?}", e);
    }

    lava_client.delete_player(guild_id).await?;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation")
        .clone();

    if manager.get(guild_id).is_some() {
        manager.remove(guild_id).await?;
    }

    Ok(())
}
