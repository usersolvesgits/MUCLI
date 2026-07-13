use crate::models::commands_traits::CommandsActions;

use clap::{Args};
use anyhow::Error;

#[derive(Args, Debug)]
pub struct SystemCommands {
    // Run (opens an application (VS, Notepad, etc.))
    // Configs (CPU, RAM, etc.)
}

impl CommandsActions for SystemCommands {
    fn run(&self) -> Result<(), Error> {
        todo!()
    }
}