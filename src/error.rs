pub type StandardError = Box<dyn std::error::Error + Send + Sync>;
use crate::prelude::DiscordData;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),

    // #[error(transparent)]
    // UnexpectedError(StandardError),
    #[error(transparent)]
    EnvVarError(#[from] std::env::VarError),

    #[error(transparent)]
    SerenityError(#[from] serenity::Error),

    #[error(transparent)]
    SongbirdJoinError(#[from] songbird::error::JoinError),

    #[error(transparent)]
    LavalinkError(#[from] lavalink_rs::error::LavalinkError),
}

pub async fn on_error(error: poise::FrameworkError<'_, DiscordData, StandardError>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}
