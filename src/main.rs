pub mod models;

use models::MUCLI::{CLI, Commands};
use models::file_commands::{FileCommandsOptions};

use std::io::{self, Write};
use std::fs;
use clap::Parser;

fn main() {
    println!("Enter 'q' or 'quit' to quit.");
    loop {
        print!("mucli > ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        let input: &str = match io::stdin().read_line(&mut input) {
            Ok(_) => { input.trim() },
            Err(_) => {
                println!("Failed to read input");
                break;
            },
        };

        if input.is_empty() {
            continue;
        }
        if input.to_lowercase() == "quit" ||
           input.to_lowercase() == "q" {
            println!("Exiting mucli.");
            break
        }

        let args = std::iter::once("mucli")
                                    .chain(input.split_whitespace());
        let result = match CLI::try_parse_from(args) {
            Ok(val) => val,
            Err(err) => {
                match err.kind() {
                    clap::error::ErrorKind::DisplayHelp |
                    clap::error::ErrorKind::DisplayVersion => {
                        print!("{}", err);
                    }
                    _ => {
                        eprintln!("ERRORE: Errore rilevato durante il parsing dei comandi dati!\n{}\n", err);
                    }
                }
                continue;
            },
        };

        if match_args_command(&result.command) {
            continue
        }

    }
}

/// Matches the given command to the function to execute internally to the function
/// #### Returns
/// a `bool` flag that indicate wether or not to continue the main loop of the program
/// Returned in case of error
fn match_args_command(command: &Commands) -> bool {
    match command {
        Commands::File(f) => {
            match &f.file_commands {
                FileCommandsOptions::Create { path } => {
                    println!("not implemented yet");
                },
                FileCommandsOptions::Delete { path } => {
                    println!("not implemented yet");
                },
                FileCommandsOptions::Write { path, message, overwrite } => {
                    match fs::exists(&path) {
                        Ok(val) => {
                            if !val {
                                eprintln!("ERRORE: Non è stato possibile raggiungere il file!");
                                return true
                            }
                        },
                        Err(_) => {
                            eprintln!("ERRORE: Non è stato possibile raggiungere il file!");
                            return true
                        }
                    }

                    if overwrite.clone() {
                        match fs::write(&path, message) {
                            Ok(_) => {},
                            Err(_) => {
                                eprintln!("ERRORE: Errore rilevato durante la scrittura del file!");
                                return true
                            }
                        }
                    } else {
                        let mut file = match fs::OpenOptions::new()
                            .write(true)
                            .append(true)
                            .open(&path) {
                            Ok(val) => val,
                            Err(_) => {
                                eprintln!("ERRORE: Errore rilevato durante l'apertura del file!");
                                return true
                            }
                        };
                        match file.write_all(message.as_bytes()) {
                            Ok(_) => {},
                            Err(_) => {
                                eprintln!("ERRORE: Errore rilevato durante la scritt    ura del file!");
                                return true
                            }
                        }
                    }
                },
                FileCommandsOptions::Read { path } => {
                    let contents = match fs::read_to_string(&path) {
                        Ok(contents) => { println!("{}", contents) },
                        Err(err) => {
                            eprintln!("ERRORE: Errore durante la lettura del file!\n{}\n", err);
                            return true
                        },
                    };
                },
            };
        },
        Commands::System(s) => {
            println!("not implemented yet");
            println!("{:?}", s);
            return true
        },
    }
    false
}
