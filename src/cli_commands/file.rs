use crate::models::commands_traits::CommandsActions;

use clap::{Args, Subcommand};
use anyhow::Error;
use std::path::PathBuf;
use std::fs;
use std::io::{Read, Write};

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
    /// Copies the text from one file to the other.
    Copy {
        /// The path of the file you want to copy the contents from.
        copy_path: PathBuf,
        /// The path of the file you want copy the contents in.
        paste_path: PathBuf,

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
                fs::File::create(&file_path)?;
                Ok(println!("Successfully created file '{:?}'", file_path))
            },
            FileCommandsOptions::Delete { path } => {
                fs::remove_file(path)?;
                Ok(println!("Successfully deleted file '{:?}'", path))
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
            FileCommandsOptions::Copy { copy_path, paste_path, overwrite } => {
                let mut file = fs::File::open(&copy_path)?;
                let mut file_contents: String = String::new();
                file.read_to_string(&mut file_contents)?;

                if overwrite.clone() {
                    fs::write(&paste_path, file_contents)?;
                } else {
                    let mut file = fs::OpenOptions::new()
                        .write(true)
                        .append(true)
                        .open(&copy_path)?;
                    file.write_all(file_contents.as_bytes())?;
                }
                Ok(println!("File contents copied successfully on file '{:?}'!", paste_path))
            },
            FileCommandsOptions::Read { path } => {
                let result: String = fs::read_to_string(&path)?;
                Ok(println!(
                    "\nREADING FILE OUTPUT\n\
                    ====================\n\
                    {}\n\
                    ====================", result))
            },
        }
    }
}