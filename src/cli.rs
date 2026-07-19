use crate::cli_commands::{
    file,
    directory,
    images,
    system,
    connection,
    misc
};
use file::FileCommands;
use directory::DirectoryCommands;
use images::ImagesCommands;
use system::SystemCommands;
use connection::ConnectionCommands;
use misc::MiscCommands;

use clap::{Parser, Subcommand};

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
    /// Clears the console.
    Clear {  },

    /// Handles file related tasks.
    File(FileCommands),
    /// Handles directories related tasks
    Dir(DirectoryCommands),
    /// Handles images related tasks
    Images(ImagesCommands),

    /// Handles system related tasks.
    System(SystemCommands),
    /// Handles connectivity related tasks
    Connection(ConnectionCommands),

    /// Handles miscellaneous tasks (weather info, etc.).
    Misc(MiscCommands),
}

//TODO (Clear)