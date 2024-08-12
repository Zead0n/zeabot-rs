mod help;
mod join;
mod leave;
mod r#loop;
mod pause;
mod play;
mod queue;
mod resume;
mod skip;

use crate::error::StandardError;
use crate::prelude::DiscordData;

pub fn get_commands() -> Vec<poise::Command<DiscordData, StandardError>> {
    vec![
        join::join(),
        leave::leave(),
        play::play(),
        pause::pause(),
        skip::skip(),
        poise::Command {
            name: String::from("loop"),
            ..r#loop::r#loop()
        },
        resume::resume(),
        queue::queue(),
    ]
}
