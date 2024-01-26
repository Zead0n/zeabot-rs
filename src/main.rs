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
    let mut discord_bot = check_result(load_bot(options).await);
    
    check_result(discord_bot.start().await);
}
