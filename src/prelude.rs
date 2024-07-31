use lavalink_rs::prelude::*;
use serenity::prelude::*;

pub use crate::error::{Error, StandardError};

pub type Result<T> = std::result::Result<T, Error>;
pub type StdResult<T> = std::result::Result<T, StandardError>;
pub type Context<'a> = poise::Context<'a, DiscordData, StandardError>;

// pub struct Wrapper<T>(pub T);

pub struct DiscordData {
    pub lavalink: LavalinkClient,
}

#[derive(Default)]
pub struct LavalinkData {
    pub looping: Mutex<bool>,
}
