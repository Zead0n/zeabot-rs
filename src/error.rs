use crate::{StdError, StdResult};
use crate::bot::Data;

pub fn check_result<T, E>(result: Result<T, E>) -> T {
    // if let Err(e) = result {
    //     panic!("Error from result: {:?}", e);
    // } else {
        
    // }

    match result {
        Ok(success) => return success,
        Err(e) => panic!("Error in result: {:?}", e)
    }
}

pub async fn on_error(error: poise::FrameworkError<'_, Data, StdError>) {
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
