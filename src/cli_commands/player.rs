use crate::models::commands_traits::CommandsActions;

use clap::Args;
use anyhow::Error;

#[derive(Args, Debug)]
pub struct PlayerCommands {
    //TODO
    // Play (plays an mp3, m4a, ... files)
}

impl CommandsActions for PlayerCommands {
    fn run(&self) -> Result<(), Error> {
        todo!()
    }
}