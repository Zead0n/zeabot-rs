use std::collections::HashMap;
use poise::{FrameworkOptions, Framework};
use poise::serenity_prelude as serenity;
use serenity::prelude::Mutex;
use songbird::serenity::SerenityInit;
use reqwest::Client as HttpClient;
use youtube_dl::SingleVideo;

use crate::{StdError, StdResult};
use crate::{error, commands};

pub struct HttpKey;
pub struct Search;

impl serenity::prelude::TypeMapKey for HttpKey {
    type Value = HttpClient;
}

impl serenity::prelude::TypeMapKey for Search {
    type Value = Mutex<HashMap<u8, SingleVideo>>;
}

pub struct Data {}

pub fn load_options() -> FrameworkOptions<Data, StdError> {
    poise::FrameworkOptions {
        commands: commands::get_commands(),
        on_error: |error| Box::pin(error::on_error(error)),
        skip_checks_for_owners: false,
        ..Default::default()
    }
}

pub async fn load_bot(options: FrameworkOptions<Data, StdError>) -> StdResult<serenity::Client> {
    let framework = Framework::new(options, |ctx, _ready, framework| {
        Box::pin(async move {
            println!("Logged in as {}", _ready.user.name);
            poise::builtins::register_globally(ctx, &framework.options().commands).await?;
            Ok(Data {})
        })
    });

    let discord_token = match std::env::var("DISCORD_TOKEN") {
        Ok(token) => token,
        Err(e) => {
            panic!("No DISCORD_TOKEN found: {}", e);
        }
    };

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
        .type_map_insert::<HttpKey>(HttpClient::new())
        .type_map_insert::<Search>(Mutex::new(HashMap::new()))
        .await
        .expect("Failed creating discord client")
    )
}
