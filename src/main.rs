mod bot;
mod commands;
mod error;
mod prelude;
mod utils;

use crate::bot::*;
use crate::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let options = load_options();
    let mut discord_bot = load_bot(options).await?;

    if let Err(e) = discord_bot.start().await {
        panic!("Discord bot failed to start (Using nvim btw): {:?}", e);
    }

    Ok(())
}
