use crate::models::commands_traits::CommandsActions;

use clap::{Args, Subcommand};
use anyhow::Error;
use open;
use std::{thread, time};
use reqwest;
use serde::{Serialize, Deserialize};
use serde_json;

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
                let api_url: String = "https://api.open-meteo.com/v1/forecast?latitude={latitude}&longitude={longitude}&hourly=temperature_2m,precipitation&forecast_days=1"
                    .replace("{latitude}", latitude.as_ref())
                    .replace("{longitude}", longitude.as_ref());

                let api_response = reqwest::blocking::get(&api_url)?;
                
                #[derive(Debug, Serialize, Deserialize)]
                struct APIResponse {
                    latitude: f64,
                    longitude: f64,
                    elevation: f32,
                    hourly: Hourly,
                }

                #[derive(Debug, Serialize, Deserialize)]
                struct Hourly {
                    time: Vec<String>,
                    temperature_2m: Vec<f32>,
                    precipitation: Vec<f32>,
                }

                let serialized_output: APIResponse = serde_json::from_str::<APIResponse>(api_response.text()?.as_str())?;

                if !complete && !lat && !long && !elevation && !temperature && !precipitation {
                    println!("ATTENTION: Enter at least one of the given flags.\n\
                              To see all of the available flags and their explanations, enter 'misc weather --help'.");
                    return Ok(())
                }

                if complete.clone() {
                    println!("[latitude: {}], [longitude: {}]", serialized_output.latitude, serialized_output.longitude);
                    println!("[elevation: {}]", serialized_output.elevation);
                    for i in 0..serialized_output.hourly.time.len() {
                        println!("[time: {}], [temp: {}], [precipitation: {}]",
                                 serialized_output.hourly.time[i] ,serialized_output.hourly.temperature_2m[i], serialized_output.hourly.precipitation[i]);
                    }
                } else {
                    if lat.clone() {
                        println!("[latitude: {}]", serialized_output.latitude);
                    }
                    if long.clone() {
                        println!("[longitude: {}]", serialized_output.longitude);
                    }
                    if elevation.clone() {
                        println!("[elevation: {}", serialized_output.elevation);
                    }
                    if temperature.clone() {
                        for i in 0..serialized_output.hourly.time.len() {
                            println!("[time: {}], [temp: {}]",
                                     serialized_output.hourly.time[i] ,serialized_output.hourly.temperature_2m[i]);
                        }
                    }
                    if precipitation.clone() {
                        for i in 0..serialized_output.hourly.time.len() {
                            println!("[time: {}], [precipitation: {}]",
                                     serialized_output.hourly.time[i] ,serialized_output.hourly.precipitation[i]);
                        }
                    }
                }

                Ok(())
            },
            MiscCommandsOptions::Credits { open_link } => {
                if open_link.clone() {
                    open::that(CREDITS_URL)?;
                    thread::sleep(time::Duration::from_secs(3));
                } else {
                    println!("Created by: {}", CREDITS_URL);
                }
                Ok(())
            }
        }
    }
}