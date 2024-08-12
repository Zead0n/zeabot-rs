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

pub enum LoopState {
    Song,
    Queue,
    Cancel,
}

pub struct PlayerData {
    pub loop_state: Mutex<LoopState>,
}

impl PlayerData {
    pub fn new() -> Self {
        Self {
            loop_state: Mutex::new(LoopState::Cancel),
        }
    }

    pub async fn set_loop_state(&mut self, state: LoopState) {
        *self.loop_state.lock().await = state;
    }

    // pub async fn get_loop_state(&self) -> LoopState {
    //     self.loop_state.lock().await.clone()
    // }
}

impl Copy for LoopState {}

impl Clone for LoopState {
    fn clone(&self) -> Self {
        *self
    }
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
