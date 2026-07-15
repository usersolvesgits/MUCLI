use crate::models::commands_traits::CommandsActions;
use crate::api::weather_api::{APIResponse, get_api_response};

use clap::{Args, Subcommand};
use anyhow::Error;
use webbrowser;

const CREDITS_URL: &str = "https://github.com/usersolvesgits";

#[derive(Args, Debug)]
pub struct MiscCommands {
    #[command(subcommand)]
    command: MiscCommandsOptions
}

#[derive(Subcommand, Debug)]
pub enum MiscCommandsOptions {
    Weather {
        latitude: String,
        longitude: String,

        /// Prints out in the terminal the latitude.
        #[arg(long)]
        lat: bool,
        /// Prints out in the terminal the longitude.
        #[arg(long)]
        long: bool,
        /// Prints out in the terminal the elevation.
        #[arg(short, long)]
        elevation: bool,

        /// Prints out in the terminal the hourly estimate of the temperature (tries also to predict future temperatures).
        #[arg(short, long)]
        temperature: bool,
        /// Prints out in the terminal the hourly estimate of the precipitation levels (tries also to predict future precipitation levels).
        #[arg(short, long)]
        precipitation: bool,

        /// Prints out in the terminal every important value to know (elevation, hourly temperatures, precipitations, etc.).
        #[arg(short, long)]
        complete: bool,
    },
    PasswordGenerator {
        //TODO
    },
    Credits {
        /// Tries to open the creators GitHub page in the default browser.
        #[arg(short, long)]
        open_link: bool,
    }
}

impl CommandsActions for MiscCommands {
    fn run(&self) -> Result<(), Error> {
        match &self.command {
            MiscCommandsOptions::Weather { longitude, latitude,
                lat, long, elevation, temperature, precipitation, complete} => {

                let api_serialized_output: APIResponse = get_api_response(longitude, latitude)?;

                if !complete && !lat && !long && !elevation && !temperature && !precipitation {
                    println!("ATTENTION: Enter at least one of the given flags.\n\
                              To see all of the available flags and their explanations, enter 'misc weather --help'.");
                    return Ok(())
                }

                if complete.clone() {
                    println!("[latitude: {}], [longitude: {}]", api_serialized_output.latitude, api_serialized_output.longitude);
                    println!("[elevation: {}]", api_serialized_output.elevation);
                    for i in 0..api_serialized_output.hourly.time.len() {
                        println!("[time: {}], [temp: {}], [precipitation: {}]",
                                 api_serialized_output.hourly.time[i] ,api_serialized_output.hourly.temperature_2m[i], api_serialized_output.hourly.precipitation[i]);
                    }
                } else {
                    if lat.clone() {
                        println!("[latitude: {}]", api_serialized_output.latitude);
                    }
                    if long.clone() {
                        println!("[longitude: {}]", api_serialized_output.longitude);
                    }
                    if elevation.clone() {
                        println!("[elevation: {}", api_serialized_output.elevation);
                    }
                    if temperature.clone() {
                        for i in 0..api_serialized_output.hourly.time.len() {
                            println!("[time: {}], [temp: {}]",
                                     api_serialized_output.hourly.time[i] ,api_serialized_output.hourly.temperature_2m[i]);
                        }
                    }
                    if precipitation.clone() {
                        for i in 0..api_serialized_output.hourly.time.len() {
                            println!("[time: {}], [precipitation: {}]",
                                     api_serialized_output.hourly.time[i] ,api_serialized_output.hourly.precipitation[i]);
                        }
                    }
                }

                Ok(())
            },
            MiscCommandsOptions::PasswordGenerator {  } => {
                //TODO
                Ok(())
            },
            MiscCommandsOptions::Credits { open_link } => {
                if open_link.clone() {
                    webbrowser::open(CREDITS_URL)?;
                } else {
                    println!("Created by: {}", CREDITS_URL);
                }
                Ok(())
            }
        }
    }
}