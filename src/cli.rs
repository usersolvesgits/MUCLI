use clap::{Parser, Subcommand};
use crate::cli_commands::{file, system, misc};
use file::FileCommands;
use system::SystemCommands;
use misc::MiscCommands;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct CLI {
    /// Command to execute.
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Quits the program.
    #[arg(short, long)]
    pub quit: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Handles file related tasks.
    File(FileCommands),
    /// Handles system related tasks.
    System(SystemCommands),
    /// Handles miscellaneous tasks (weather info, etc.).
    Misc(MiscCommands),
}