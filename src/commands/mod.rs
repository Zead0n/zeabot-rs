mod help;
mod join;
mod leave;
mod r#loop;
mod play;
mod queue;
mod skip;

use crate::error::StandardError;
use crate::prelude::DiscordData;

pub fn get_commands() -> Vec<poise::Command<DiscordData, StandardError>> {
    vec![
        join::join(),
        leave::leave(),
        // poise::Command {
        //     subcommands: vec![play::url(), play::search()],
        //     subcommand_required: true,
        //     ..play::play()
        // },
        play::play(),
        skip::skip(),
        poise::Command {
            name: String::from("loop"),
            ..r#loop::r#loop()
        },
        // queue::queue(),
    ]
}
