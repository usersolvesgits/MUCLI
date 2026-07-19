mod cli;
mod cli_commands;
mod models;
pub mod api;

use cli::*;
use models::commands_traits::CommandsActions;

use std::io::{self, Write};
use clap::Parser;

fn main() {
    println!("Enter '-q' or '--quit' to quit.");
    println!("Enter '--help' for options.");
    loop {
        print!("mucli> ");
        match io::stdout().flush() {
            Ok(_) => {  },
            Err(_) => {
                println!("Failed to flush stdout");
                break
            },
        };

        let mut input: String = String::new();
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

        let string_split: Vec<String> = split_string(&input);

        let args = std::iter::once("mucli".to_string())
            .chain(string_split);
        let result: CLI = match CLI::try_parse_from(args) {
            Ok(val) => val,
            Err(err) => {
                match err.kind() {
                    clap::error::ErrorKind::DisplayHelp |
                    clap::error::ErrorKind::DisplayVersion => {
                        print!("{}", err);
                    }
                    _ => {
                        eprintln!("ERROR: Error found during parsing the arguments!\n{}\n", err);
                    }
                }
                continue
            },
        };

        if result.quit {
            println!("Exiting mucli.");
            break
        }

        match &result.command {
            Some(Commands::Clear {}) => {
                todo!()
            },

            Some(Commands::File(f)) => {
                match f.run() {
                    Ok(_) => {},
                    Err(err) => {
                        println!("Error: Found error: {}\n", err);
                        continue
                    },
                }
            },

            Some(Commands::Dir(d)) => {
                match d.run() {
                    Ok(_) => {},
                    Err(err) => {
                        println!("Error: Found error: {}\n", err);
                        continue
                    },
                }
            },

            Some(Commands::Images(i)) => {
                match i.run() {
                    Ok(_) => {},
                    Err(err) => {
                        println!("Error: Found error: {}\n", err);
                        continue
                    },
                }
            },

            Some(Commands::System(s)) => {
                match s.run() {
                    Ok(_) => {},
                    Err(err) => {
                        println!("Error: Found error: {}\n", err);
                        continue
                    },
                }
            },

            Some(Commands::Connection(c)) => {
                match c.run() {
                    Ok(_) => {},
                    Err(err) => {
                        println!("Error: Found error: {}\n", err);
                        continue
                    },
                }
            },

            Some(Commands::Misc(m)) => {
                match m.run() {
                    Ok(_) => {},
                    Err(err) => {
                        println!("Error: Found error: {}\n", err);
                        continue
                    },
                }
            },

            None => {
                println!("Error: Make sure to enter a valid command.");
                continue
            }
        }
    }
}

/// Splits the string passed as an argument, checking for double quotes and spaces, for a correct formatting.
/// ### Returns
/// A `Vector` of `String`.
fn split_string(input: &str) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut current: String = String::new();
    let mut in_quotes: bool = false;
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '"' => in_quotes = !in_quotes,
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(c),
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}
