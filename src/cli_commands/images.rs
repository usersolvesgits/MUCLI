use crate::models::commands_traits::CommandsActions;

use clap::Args;
use anyhow::Error;

#[derive(Args, Debug)]
pub struct ImagesCommands {
    //TODO
}

impl CommandsActions for ImagesCommands {
    fn run(&self) -> Result<(), Error> {
        todo!();
    }
}