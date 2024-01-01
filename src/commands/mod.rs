mod help;
mod join;
mod leave;
mod play;

use crate::bot::Data;
use crate::StdError;

pub fn get_commands() -> Vec<poise::Command<Data, StdError>> {
   vec![
      help::help(),
      join::join(),
      leave::leave(),
      poise::Command {
         subcommands: vec![
            play::url(),
         ],
         subcommand_required: true,
         ..Default::default()
         ..play::play()
      },
   ]
}