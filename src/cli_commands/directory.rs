use crate::models::commands_traits::CommandsActions;

use std::path::PathBuf;
use anyhow::Error;
use clap::{Args, Subcommand};
use std::fs;

#[derive(Args, Debug)]
pub struct DirectoryCommands {
    #[clap(subcommand)]
    pub command: DirectoryCommandsOptions,
}

#[derive(Subcommand, Debug)]
pub enum DirectoryCommandsOptions {
    /// Creates a new directory
    Create {
        /// The name of the directory
        name: String,
        /// Where to save the subdirectory (the parent directory)
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
        match &self.command {
            DirectoryCommandsOptions::Create { name, path } => {
                let dir_path: PathBuf = path.join(&name);
                fs::create_dir(&dir_path)?;
                Ok(println!("Directory '{}' created", dir_path.display()))
            }
            DirectoryCommandsOptions::Delete { dir_path, save_elements } => {
                if save_elements.clone() {
                    let save_path: PathBuf = match containing_dir(&dir_path) {
                        Some(path) => path,
                        None => return Ok(println!("ERROR: Couldn't find a parent directory where to save the files!")),
                    };

                    if let Err(e) = move_dir_contents(&dir_path, &save_path) {
                        return Ok(println!("ERROR: Spostamento fallito: {}", e));
                    }

                    fs::remove_dir(&dir_path)?;

                    Ok(println!("Directory '{}' deleted, and files moved to directory {}",
                                dir_path.display(), save_path.display()))
                } else {
                    fs::remove_dir_all(&dir_path)?;
                    Ok(println!("Directory '{}' deleted", dir_path.display()))
                }
            }
        }
    }
}

/// check whether the `dir_path` directory has a parent directory where to save the files.
fn containing_dir(dir_path: &PathBuf) -> Option<PathBuf> {
    if !dir_path.is_dir() {
        return None;
    }
    dir_path.parent().map(PathBuf::from)
}

/// Moves all the contents present in `main_path` inside the `save_path` directory, making sure there aren't any conflicts of names.
fn move_dir_contents(main_path: &PathBuf, save_path: &PathBuf) -> Result<(), Error> {
    for entry in fs::read_dir(main_path)? {
        let entry = entry?;
        let entry_path: PathBuf = entry.path();
        let file_name = entry.file_name();

        let mut target_path: PathBuf = save_path.join(&file_name);

        if target_path.exists() {
            target_path = unique_target_path(save_path, &file_name);
        }

        fs::rename(&entry_path, &target_path)?;
    }

    Ok(())
}

/// Gives a unique name to an element if another one with the same name exists.
fn unique_target_path(destination: &PathBuf, file_name: &std::ffi::OsStr) -> PathBuf {
    let original_name: String = file_name.to_string_lossy().to_string();

    let (stem, ext) = match original_name.rsplit_once('.') {
        Some((s, e)) if !s.is_empty() => (s.to_string(), Some(e.to_string())),
        _ => (original_name.clone(), None),
    };

    let mut counter = 1;
    loop {
        let candidate_name = match &ext {
            Some(e) => format!("{}_{}.{}", stem, counter, e),
            None => format!("{}_{}", stem, counter),
        };
        let candidate_path = destination.join(&candidate_name);
        if !candidate_path.exists() {
            return candidate_path;
        }
        counter += 1;
    }
}