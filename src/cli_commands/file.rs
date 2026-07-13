use crate::models::commands_traits::CommandsActions;

use clap::{Args, Subcommand};
use anyhow::Error;
use std::path::PathBuf;
use std::fs;

#[derive(Args, Debug)]
pub struct FileCommands {
    #[command(subcommand)]
    pub file_commands: FileCommandsOptions,
}

#[derive(Subcommand, Debug)]
pub enum FileCommandsOptions {
    /// Creates a file to the given path.
    Create {
        /// The name of the file (it `MUST` include its extension).
        file_name: String,
        /// The path of the directory where you want to save the file.
        dir_path: PathBuf
    },
    /// Deletes a file.
    Delete {
        /// The path of the file.
        path: PathBuf
    },

    /// Write contents inside the file.
    Write {
        /// The contents you want in the file written inside double quotes.
        message: String,
        /// The path of the file.
        path: PathBuf,

        /// If selected, the contents of the file will be overwritten by the message.
        #[arg(short, long)]
        overwrite: bool,
    },
    /// Outputs the contents of the file in the terminal.
    Read {
        /// The path of the file.
        path: PathBuf
    },
}

impl CommandsActions for FileCommands {
    fn run(&self) -> Result<(), Error>{
        match &self.file_commands {
            FileCommandsOptions::Create { file_name, dir_path } => {
                let file_path: PathBuf = dir_path.join(&file_name);
                fs::File::create(file_path)?;
                Ok(())
            },
            FileCommandsOptions::Delete { path } => {
                fs::remove_file(path)?;
                Ok(())
            },
            FileCommandsOptions::Write { message, path, overwrite } => {
                if overwrite.clone() {
                    fs::write(path, String::new())?;
                    fs::write(path, message)?;
                } else {
                    fs::write(path, message)?;
                }
                Ok(println!("Writing successful on file '{:?}'", path))
            },
            FileCommandsOptions::Read { path } => {
                let result: String = fs::read_to_string(&path)?;
                Ok(println!(
                    "\nREADING FILE OUTPUT\n\
                    ====================\n\
                    {}", result))
            },
        }
    }
}