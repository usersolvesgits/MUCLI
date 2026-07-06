use clap::{Parser, Subcommand};
use crate::models::system_commands::{SystemCommands};
use crate::models::file_commands::{FileCommands};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
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