use lavalink_rs::prelude::*;
use serenity::prelude::Mutex;

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
    pub looping: Mutex<bool>,
}

impl PlayerData {
    pub fn new() -> Self {
        Self {
            looping: false.into(),
        }
    }

    // pub async fn get_loop_state(&self) -> bool {
    //     let state = *self.looping.lock().await;
    //     state
    // }
    //
    // pub async fn toggle_loop(&mut self) -> bool {
    //     let mut loop_state = *self.looping.lock().await;
    //
    //     if loop_state == true {
    //         loop_state = false;
    //     } else {
    //         loop_state = true;
    //     }
    //
    //     loop_state
    // }
}

// impl Deref for PlayerData {
//     type Target = PlayerData;
//
//     fn deref(&self) -> &Self::Target {
//         &self
//     }
// }
//
// impl DerefMut for PlayerData {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         self
//     }
// }

// impl Copy for PlayerData {}

// impl Clone for PlayerData {
//     fn clone(&self) -> Self {
//         *self
//     }
// }
