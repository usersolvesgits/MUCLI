use crate::models::commands_traits::CommandsActions;

use clap::{Args, Subcommand};
use anyhow::Error;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct ConnectionCommands {
    #[command(subcommand)]
    pub command: ConnectionCommandsOptions,
}

#[derive(Subcommand, Debug)]
pub enum ConnectionCommandsOptions {
    /// Sends 4 small packets to a remote host to ensure a connection status.
    Ping {
        remote_host: String,
    },
}

impl CommandsActions for ConnectionCommands {
    fn run(&self) -> Result<(), Error> {
        //TODO
        match &self.command {
            ConnectionCommandsOptions::Ping { remote_host } => {
                Ok(())
            },
        }
    }
}