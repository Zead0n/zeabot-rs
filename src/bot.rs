use poise::serenity_prelude as serenity;
use poise::{Framework, FrameworkOptions};
use serenity::Settings as CacheSettings;
use songbird::serenity::SerenityInit;

use crate::commands::get_commands;
use crate::error::*;
use crate::prelude::*;
use crate::utils::*;

pub fn load_options() -> FrameworkOptions<DiscordData, StandardError> {
    poise::FrameworkOptions {
        commands: get_commands(),
        on_error: |error| Box::pin(on_error(error)),
        command_check: Some(|ctx| {
            Box::pin(async move { Ok(discord::has_perm(&ctx).await.unwrap_or(false)) })
        }),
        skip_checks_for_owners: false,
        ..Default::default()
    }
}

pub async fn load_bot(
    options: FrameworkOptions<DiscordData, StandardError>,
) -> Result<serenity::Client> {
    let framework = Framework::new(options, |ctx, _ready, framework| {
        Box::pin(async move {
            println!("Logged in as {}", _ready.user.name);

            // Setup Avatar
            let mut bot_user = match ctx.http.get_current_user().await {
                Ok(user) => user,
                Err(e) => return Err(e.into()),
            };
            let new_avatar = serenity::CreateAttachment::path("/data/avatar.gif").await?;
            let profile = serenity::EditProfile::new().avatar(&new_avatar);

            bot_user.edit(ctx, profile).await?;
            poise::builtins::register_globally(ctx, &framework.options().commands).await?;

            // Create LavalinkClient
            // let lavalink_password = match std::env::var("LAVALINK_PASSWORD") {
            //     Ok(token) => token,
            //     Err(e) => panic!("Failed to obtain LAVALINK_PASSWORD: {:?}", e),
            // };

            let user_id = ctx.cache.current_user().id;

            // let node_local = NodeBuilder {
            //     hostname: "lavalink:2333".to_string(),
            //     is_ssl: false,
            //     events: events::Events::default(),
            //     password: lavalink_password,
            //     user_id: user_id_raw.into(),
            //     session_id: None,
            // };

            // let custom_events = events::Events {
            //     track_end: Some()
            //     ..Default::default()
            // };

            // let lava_client = LavalinkClient::new_with_data(
            //     events::Events::default(),
            //     vec![node_local],
            //     NodeDistributionStrategy::new(),
            //     Arc::new(LavalinkData::default()),
            // )
            // .await;

            let lava_client = lavalink::create_lavalink_client(user_id.into()).await?;

            Ok(DiscordData {
                lavalink: lava_client,
            })
        })
    });

    let discord_token = match std::env::var("DISCORD_TOKEN") {
        Ok(token) => token,
        Err(e) => panic!("Failed to obtain DISCORD_TOKEN: {:?}", e),
    };

    let mut cache_settings = CacheSettings::default();
    cache_settings.cache_users = false;

    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILDS
        | serenity::GatewayIntents::GUILD_MEMBERS
        | serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::GUILD_PRESENCES
        | serenity::GatewayIntents::GUILD_VOICE_STATES;

    Ok(serenity::Client::builder(discord_token, intents)
        .framework(framework)
        .register_songbird()
        .cache_settings(cache_settings)
        .await
        .expect("Failed creating discord client"))
}
