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
    /// Sends a `GET` request to a remote api url, with optional field to store the result of the request in a text file.
    GetRequest {
        api_url: String,
        api_key: Option<String>,
        save_file: Option<PathBuf>,
    }
}

impl CommandsActions for ConnectionCommands {
    fn run(&self) -> Result<(), Error> {
        //TODO
        match &self.command {
            ConnectionCommandsOptions::Ping { remote_host } => {
                Ok(())
            },
            ConnectionCommandsOptions::GetRequest { api_url, api_key, save_file } => {
                Ok(())
            }
        }
    }
}