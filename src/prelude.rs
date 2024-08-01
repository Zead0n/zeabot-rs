use std::{thread::sleep, time::Duration};

use lavalink_rs::prelude::*;

pub use crate::error::{Error, StandardError};

pub type Result<T> = std::result::Result<T, Error>;
pub type StdResult<T> = std::result::Result<T, StandardError>;
pub type Context<'a> = poise::Context<'a, DiscordData, StandardError>;

// pub struct Wrapper<T>(pub T);

pub struct DiscordData {
    pub lavalink: LavalinkClient,
}

pub enum TimeoutStatus {
    TimedOut,
    Resetted,
}

#[derive(Debug)]
pub struct PlayerData {
    pub looping: bool,
    pub timeout: Duration,
    countdown: bool,
}

impl PlayerData {
    pub fn new() -> Self {
        Self {
            looping: false,
            timeout: Duration::from_secs(60 * 3),
            countdown: false,
        }
    }

    pub fn toggle_loop(&mut self, toggle: bool) {
        self.looping = toggle;
    }

    pub fn stop_timeout(&mut self) {
        self.countdown = false;
    }

    pub fn start_timeout(&mut self) -> TimeoutStatus {
        self.countdown = true;
        let mut timeout_clone = self.timeout.clone();

        while !timeout_clone.is_zero() {
            if !self.countdown {
                println!("Timeout count resseted");
                return TimeoutStatus::Resetted;
            }

            sleep(Duration::from_secs(1));
            timeout_clone -= Duration::from_secs(1);
        }

        println!("{:#?}", self);
        println!("TimedOut, leaving");
        TimeoutStatus::TimedOut
    }
}

impl Copy for PlayerData {}

impl Clone for PlayerData {
    fn clone(&self) -> Self {
        *self
    }
}
