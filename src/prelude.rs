use lavalink_rs::prelude::*;

pub use crate::error::{Error, StandardError};

pub type Result<T> = std::result::Result<T, Error>;
pub type StdResult<T> = std::result::Result<T, StandardError>;
pub type Context<'a> = poise::Context<'a, DiscordData, StandardError>;

// pub struct Wrapper<T>(pub T);

pub struct DiscordData {
    pub lavalink: LavalinkClient,
}

#[derive(Debug)]
pub struct PlayerData {
    pub looping: bool,
}

impl PlayerData {
    pub fn new() -> Self {
        Self { looping: false }
    }

    pub fn toggle_loop(&mut self, toggle: bool) {
        self.looping = toggle;
    }
}

impl Copy for PlayerData {}

impl Clone for PlayerData {
    fn clone(&self) -> Self {
        *self
    }
}
