mod bot;
mod commands;
mod error;
mod helper;

use tokio;
use crate::helper::*;
use crate::bot::*;

type StdError = Box<dyn std::error::Error + Send + Sync>;
type StdResult<T> = std::result::Result<T, StdError>;
type Context<'a> = poise::Context<'a, Data, StdError>;

#[tokio::main]
async fn main() {
    let options = load_options();
    let mut discord_bot = match load_bot(options).await {
        Ok(bot) => bot,
        Err(e) => panic!("Error loading bot: {:?}", e),
    };

    if let Err(e) = discord_bot.start().await {
        panic!("Discord bot failed to start (Using nvim btw): {:?}", e);
    }
}
