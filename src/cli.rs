use clap::{Parser, Subcommand};
use crate::cli_commands::file::{FileCommands};
use crate::cli_commands::system::{SystemCommands};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CLI {
    /// Command to execute
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Handles file related tasks
    File(FileCommands),
    /// Handles system related tasks
    System(SystemCommands),
}