use lavalink_rs::client::LavalinkClient;
use std::rc::Rc;

pub use crate::error::{Error, StandardError};

pub type Result<T> = std::result::Result<T, Error>;
pub type StdResult<T> = std::result::Result<T, StandardError>;
pub type Context<'a> = poise::Context<'a, Data, StandardError>;

pub struct Wrapper<T>(pub T);

pub struct Data {
    pub lavalink: LavalinkClient,
}
