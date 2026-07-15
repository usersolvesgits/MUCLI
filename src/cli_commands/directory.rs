use std::path::PathBuf;
use anyhow::Error;
use crate::models::commands_traits::CommandsActions;

use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct DirectoryCommands {
    #[clap(subcommand)]
    pub command: DirectoryCommandsOptions,
}

#[derive(Subcommand, Debug)]
pub enum DirectoryCommandsOptions {
    /// Creates a new directory
    Make {
        /// The name of the directory
        name: String,
        /// Where to save the subdirectory
        path: PathBuf,
    },
    /// Deletes a directory
    Delete {
        dir_path: PathBuf,

        /// Chooses whether to save the elements in the outer directory or not
        #[arg(short, long)]
        save_elements: bool,
    }
}

impl CommandsActions for DirectoryCommands {
    fn run(&self) -> Result<(), Error> {
        todo!()
    }
}