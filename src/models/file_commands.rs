use std::path::PathBuf;
use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct FileCommands {
    #[command(subcommand)]
    pub file_commands: FileCommandsOptions,
}

#[derive(Subcommand, Debug)]
pub enum FileCommandsOptions {
    /// Creates a file to the given path.
    Create {
        path: PathBuf
    },
    /// Deletes a file.
    Delete {
        path: PathBuf
    },

    /// Write contents inside the file.
    Write {
        message: String,
        path: PathBuf,

        #[arg(short, long)]
        overwrite: bool,
    },
    /// Outputs the contents of the file in the terminal.
    Read {
        path: PathBuf,
    },
}