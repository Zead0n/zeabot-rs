mod bot;
mod commands;
mod error;

extern crate dotenv;
use dotenv::dotenv;
use tokio;
use crate::bot::*;

type StdError = Box<dyn std::error::Error + Send + Sync>;
type StdResult<T> = std::result::Result<T, StdError>;
type Context<'a> = poise::Context<'a, Data, StdError>;

#[tokio::main]
async fn main() {    
    dotenv().ok();

    let options = load_options();
    let mut discord_bot = match load_bot(options).await {
        Ok(bot) => bot,
        Err(e) => {
            panic!("Error making bot framework: {:?}", e);
        }
    };
    
    if let Err(e) = discord_bot.start().await {
        panic!("Skill issue: {}", e);
    }

}
